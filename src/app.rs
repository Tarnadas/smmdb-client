use crate::{
    components::SaveButton,
    emu::*,
    pages::{InitPage, SavePage},
    Component, EmuSave, EmuType, Page,
};

use iced::{executor, Application, Command, Element};
use nfd::Response;
use std::path::PathBuf;

pub struct App {
    current_page: Box<dyn Page>,
    save: Option<(smmdb::Save, PathBuf)>,
    save_buttons: Vec<Box<dyn Component>>,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenSave(EmuSave),
    OpenCustomSave,
    LoadSave(smmdb::Save, PathBuf),
    LoadSaveError(String),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let mut save_buttons = vec![];
        guess_emu_dir(&mut save_buttons);
        (
            App {
                current_page: Box::new(InitPage::new()),
                save: None,
                save_buttons,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SMMDB")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::OpenSave(save) => Command::perform(
                smmdb::Save::new(save.get_smm2_location()),
                move |res| match res {
                    Ok(smmdb_save) => Message::LoadSave(smmdb_save, save.get_location().clone()),
                    Err(err) => Message::LoadSaveError(err.into()),
                },
            ),
            Message::OpenCustomSave => match nfd::open_pick_folder(None) {
                Ok(result) => match result {
                    Response::Okay(file_path) => {
                        let file_path: PathBuf = file_path.into();
                        if is_yuzu_dir(file_path.clone()) {
                            self.save_buttons
                                .insert(0, Box::new(SaveButton::new(file_path, EmuType::Yuzu)));
                        } else if is_ryujinx_dir(file_path.clone()) {
                            self.save_buttons
                                .insert(0, Box::new(SaveButton::new(file_path, EmuType::Ryujinx)));
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
                self.save = Some((smmdb_save, location));
                self.current_page = Box::new(SavePage::new());
                Command::none()
            }
            Message::LoadSaveError(err) => {
                dbg!(&err);
                // TODO show error
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        if self.current_page.downcast_ref::<InitPage>().is_some() {
            self.current_page
                .view("Please select your save folder", &mut self.save_buttons)
                .into()
        } else if self.current_page.downcast_ref::<SavePage>().is_some() {
            let title = self.save.as_ref().unwrap().1.to_str().unwrap();
            self.current_page
                .view(title, unsafe { &mut *std::ptr::null_mut() })
                .into()
        } else {
            panic!()
        }
    }
}
