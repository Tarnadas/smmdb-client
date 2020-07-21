use crate::{EmuType, Message, Save};

use iced::{button, Button, Text};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SaveButton {
    state: button::State,
    save: Save,
}

impl SaveButton {
    pub fn new(location: PathBuf, emu_type: EmuType) -> SaveButton {
        SaveButton {
            state: button::State::new(),
            save: Save::new(location, emu_type),
        }
    }
}

impl SaveButton {
    pub fn view(&mut self) -> Button<Message> {
        Button::new(&mut self.state, Text::new(format!("{}", self.save)))
            .on_press(Message::OpenSave(self.save.clone()))
    }
}
