use crate::{
    emu::*,
    pages::{InitPage, SavePage},
    smmdb::{Course2Response, QueryParams},
    styles::*,
    EmuSave, Page, Progress, Smmdb,
};

use iced::{
    container, executor, Application, Background, Command, Container, Element, Length, Subscription,
};
use iced_native::{keyboard, subscription, Event};
use nfd::Response;
use std::{convert::TryInto, path::PathBuf};

pub struct App {
    state: AppState,
    current_page: Page,
    smmdb: Smmdb,
    window_size: WindowSize,
}

#[derive(Clone, Debug)]
pub enum AppState {
    Default,
    Loading,
    SwapSelect(usize),
    DownloadSelect(usize),
    DeleteSelect(usize),
    Downloading {
        save_index: usize,
        smmdb_id: String,
        progress: f32,
    },
}

#[derive(Clone, Debug)]
pub enum Message {
    Empty,
    SetWindowSize(WindowSize),
    OpenSave(EmuSave),
    OpenCustomSave,
    LoadSave(smmdb_lib::Save, PathBuf),
    LoadSaveError(String),
    FetchCourses(QueryParams),
    FetchError(String),
    SetSmmdbCourses(Vec<Course2Response>),
    SetSmmdbCourseThumbnail(Vec<u8>, String),
    InitSwapCourse(usize),
    SwapCourse(usize, usize),
    InitDownloadCourse(usize),
    DownloadCourse(usize, String),
    DownloadProgressed(Progress),
    InitDeleteCourse(usize),
    DeleteCourse(usize),
    PaginateForward,
    PaginateBackward,
    ResetState,
}

#[derive(Clone, Debug)]
pub enum WindowSize {
    S,
    M,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let components = guess_emu_dir();
        let smmdb = Smmdb::new();
        let query_params = smmdb.get_query_params().clone();
        (
            App {
                state: AppState::Default,
                current_page: Page::Init(InitPage::new(components)),
                smmdb,
                window_size: WindowSize::M,
            },
            Command::perform(async {}, move |_| {
                Message::FetchCourses(query_params.clone())
            }),
        )
    }

    fn title(&self) -> String {
        String::from("SMMDB")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Empty => Command::none(),
            Message::SetWindowSize(window_size) => {
                // TODO listen to application resize somehow
                self.window_size = window_size;
                Command::none()
            }
            Message::OpenSave(save) => {
                Command::perform(smmdb_lib::Save::new(save.get_smm2_location()), move |res| {
                    match res {
                        Ok(smmdb_save) => {
                            Message::LoadSave(smmdb_save, save.get_location().clone())
                        }
                        Err(err) => Message::LoadSaveError(err.into()),
                    }
                })
            }
            Message::OpenCustomSave => match nfd::open_pick_folder(None) {
                Ok(result) => match result {
                    Response::Okay(file_path) => {
                        let file_path: PathBuf = file_path.into();
                        if is_yuzu_dir(file_path.clone()) {
                            // TODO
                            // self.components
                            //     .insert(0, Box::new(SaveButton::new(file_path, EmuType::Yuzu)));
                        } else if is_ryujinx_dir(file_path.clone()) {
                            // TODO
                            // self.components
                            //     .insert(0, Box::new(SaveButton::new(file_path, EmuType::Ryujinx)));
                        }
                        // TODO save path on success
                        Command::none()
                    }
                    Response::OkayMultiple(_files) => {
                        println!("Not multifile select");
                        Command::none()
                    }
                    Response::Cancel => {
                        println!("User canceled");
                        Command::none()
                    }
                },
                Err(err) => Command::perform(async {}, move |_| {
                    Message::LoadSaveError(format!("{:?}", err))
                }),
            },
            Message::LoadSave(smmdb_save, location) => {
                self.current_page = Page::Save(SavePage::new(smmdb_save, location));
                Command::none()
            }
            Message::LoadSaveError(err) => {
                dbg!(&err);
                // TODO show error
                Command::none()
            }
            Message::FetchCourses(query_params) => {
                Command::perform(Smmdb::update(query_params), move |res| match res {
                    Ok(courses) => Message::SetSmmdbCourses(courses),
                    Err(err) => Message::FetchError(err.to_string()),
                })
            }
            Message::FetchError(err) => {
                dbg!(err);
                // TODO handle error
                Command::none()
            }
            Message::SetSmmdbCourses(courses) => {
                self.smmdb.set_courses(courses);
                let course_ids: Vec<String> =
                    self.smmdb.get_course_panels().keys().cloned().collect();

                let mut commands = Vec::<Command<Message>>::new();
                for id in course_ids {
                    commands.push(Command::perform(
                        async move {
                            futures::join!(
                                Smmdb::fetch_thumbnail(id.clone()),
                                futures::future::ok::<String, String>(id)
                            )
                        },
                        |(thumbnail, id)| {
                            if let (Ok(thumbnail), Ok(id)) = (thumbnail, id) {
                                Message::SetSmmdbCourseThumbnail(thumbnail, id)
                            } else {
                                // TODO handle error
                                Message::Empty
                            }
                        },
                    ));
                }
                Command::batch(commands)
            }
            Message::SetSmmdbCourseThumbnail(thumbnail, id) => {
                self.smmdb.set_course_panel_thumbnail(&id, thumbnail);
                Command::none()
            }
            Message::InitSwapCourse(index) => {
                self.state = AppState::SwapSelect(index);
                Command::none()
            }
            Message::SwapCourse(first, second) => {
                self.state = AppState::Loading;

                match self.current_page {
                    Page::Save(ref mut save_page) => {
                        let fut = save_page.swap_courses(first as u8, second as u8);
                        futures::executor::block_on(fut).unwrap();
                        // TODO find better way than block_on
                        Command::perform(async {}, |_| Message::ResetState)
                    }
                    _ => Command::none(),
                }
            }
            Message::InitDownloadCourse(index) => {
                self.state = AppState::DownloadSelect(index);
                Command::none()
            }
            Message::DownloadCourse(save_index, smmdb_id) => {
                self.state = AppState::Downloading {
                    save_index,
                    smmdb_id,
                    progress: 0.,
                };
                Command::none()
            }
            Message::DownloadProgressed(message) => {
                match &mut self.state {
                    AppState::Downloading {
                        save_index,
                        progress,
                        ..
                    } => match message {
                        Progress::Started => {
                            *progress = 0.;
                        }
                        Progress::Advanced(percentage) => {
                            *progress = percentage;
                        }
                        Progress::Finished(data) => {
                            let save_index = save_index.clone();
                            match self.current_page {
                                Page::Save(ref mut save_page) => {
                                    let course: smmdb_lib::Course2 = data.try_into().unwrap();
                                    let fut = save_page.add_course(save_index as u8, course);
                                    futures::executor::block_on(fut).unwrap();
                                    // TODO find better way than block_on
                                    return Command::perform(async {}, |_| Message::ResetState);
                                }
                                _ => {
                                    // TODO
                                }
                            }
                        }
                        Progress::Errored => {
                            // TODO
                        }
                    },
                    _ => {}
                };
                Command::none()
            }
            Message::InitDeleteCourse(index) => {
                self.state = AppState::DeleteSelect(index);
                Command::none()
            }
            Message::DeleteCourse(index) => {
                self.state = AppState::Loading;

                match self.current_page {
                    Page::Save(ref mut save_page) => {
                        let fut = save_page.delete_course(index as u8);
                        futures::executor::block_on(fut).unwrap();
                        // TODO find better way than block_on
                        Command::perform(async {}, |_| Message::ResetState)
                    }
                    _ => Command::none(),
                }
            }
            Message::PaginateForward => {
                self.smmdb.paginate_forward();
                Command::perform(
                    Smmdb::update(self.smmdb.get_query_params().clone()),
                    move |res| match res {
                        Ok(courses) => Message::SetSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::PaginateBackward => {
                self.smmdb.paginate_backward();
                Command::perform(
                    Smmdb::update(self.smmdb.get_query_params().clone()),
                    move |res| match res {
                        Ok(courses) => Message::SetSmmdbCourses(courses),
                        Err(err) => Message::FetchError(err.to_string()),
                    },
                )
            }
            Message::ResetState => {
                self.state = AppState::Default;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            AppState::SwapSelect(_) | AppState::DownloadSelect(_) | AppState::DeleteSelect(_) => {
                subscription::events().map(|event| match event {
                    Event::Keyboard(keyboard::Event::KeyReleased {
                        key_code: keyboard::KeyCode::Escape,
                        modifiers: _,
                    }) => Message::ResetState,
                    _ => Message::Empty,
                })
            }
            AppState::Downloading { smmdb_id, .. } => {
                Smmdb::download_course(smmdb_id.clone()).map(Message::DownloadProgressed)
            }
            AppState::Default | AppState::Loading => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(match &mut self.current_page {
            Page::Init(init_page) => init_page.view().into(),
            Page::Save(save_page) => save_page.view(&self.state, &mut self.smmdb),
        })
        .style(AppStyle)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

struct AppStyle;

impl container::StyleSheet for AppStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(COLOR_YELLOW)),
            ..container::Style::default()
        }
    }
}
