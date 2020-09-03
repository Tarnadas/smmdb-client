use crate::{
    emu::*,
    pages::{InitPage, SavePage},
    smmdb::{Course2Response, QueryParams},
    styles::*,
    EmuSave, Page, Smmdb,
};

use iced::{container, executor, Application, Background, Command, Container, Element, Length};
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
    SetCourse(usize, Vec<u8>),
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
            Message::DownloadCourse(index, id) => {
                self.state = AppState::Loading;

                Command::perform(
                    async move {
                        futures::join!(
                            Smmdb::download_course(id),
                            futures::future::ok::<usize, usize>(index)
                        )
                    },
                    |(data, index)| {
                        if let (Ok(data), Ok(index)) = (data, index) {
                            Message::SetCourse(index, data)
                        } else {
                            todo!()
                        }
                    },
                )
            }
            Message::SetCourse(index, data) => {
                match self.current_page {
                    Page::Save(ref mut save_page) => {
                        let course: smmdb_lib::Course2 = data.try_into().unwrap();
                        let fut = save_page.add_course(index as u8, course);
                        futures::executor::block_on(fut).unwrap();
                        // TODO find better way than block_on
                        Command::perform(async {}, |_| Message::ResetState)
                    }
                    _ => Command::none(),
                }
            }
            Message::ResetState => {
                self.state = AppState::Default;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(match &mut self.current_page {
            Page::Init(init_page) => init_page.view().into(),
            Page::Save(save_page) => save_page.view(&self.state, self.smmdb.get_course_panels()),
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
