use crate::{
    emu::*,
    pages::{InitPage, SavePage},
    smmdb::{Course2Response, QueryParams},
    EmuSave, Page, Smmdb,
};

use iced::{executor, Application, Command, Element};
use nfd::Response;
use std::path::PathBuf;

pub struct App {
    current_page: Page,
    smmdb: Smmdb,
    window_size: WindowSize,
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
                // self.save_data = Some(SaveData::new(smmdb_save, location));
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
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match &mut self.current_page {
            Page::Init(init_page) => init_page.view().into(),
            Page::Save(save_page) => save_page.view(self.smmdb.get_course_panels()),
        }
    }
}
