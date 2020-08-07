use crate::{components::SaveButton, emu::*, pages::InitPage, Component, EmuType, Page, Save};

use iced::{executor, Application, Command, Element};
use nfd::Response;
use std::path::PathBuf;

pub struct App {
    current_page: Box<dyn Page>,
    save: Option<smmdb::Save>,
    save_buttons: Vec<Box<dyn Component>>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ChangeView,
    OpenSave(Save),
    OpenCustomSave,
    LoadSave(smmdb::Save),
    LoadSaveError(String),
}

impl Application for App {
    type Executor = executor::Null;
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
            Message::ChangeView => unimplemented!(),
            Message::OpenSave(save) => Command::perform(
                async move { smmdb::Save::new(save.get_location().clone()).await },
                |res| match res {
                    Ok(save) => Message::LoadSave(save),
                    Err(err) => Message::LoadSaveError("".to_string()), // TODO
                },
            ),
            Message::OpenCustomSave => {
                let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
                    panic!(e);
                });

                match result {
                    Response::Okay(file_path) => {
                        let file_path: PathBuf = file_path.into();
                        if is_yuzu_dir(file_path.clone()) {
                            self.save_buttons
                                .insert(0, Box::new(SaveButton::new(file_path, EmuType::Yuzu)));
                        } else if is_ryujinx_dir(file_path.clone()) {
                            self.save_buttons
                                .insert(0, Box::new(SaveButton::new(file_path, EmuType::Ryujinx)));
                        }
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
                }
            }
            Message::LoadSave(save) => {
                self.save = Some(save);
                Command::none()
            }
            Message::LoadSaveError(err) => {
                // TODO show error
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.current_page.view(&mut self.save_buttons).into()
    }
}
