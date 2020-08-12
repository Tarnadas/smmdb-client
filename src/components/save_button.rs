use crate::{Component, EmuSave, EmuType, Message};

use iced::{button, Button, Text};
use iced_native::Element;
use iced_wgpu::Renderer;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SaveButton {
    state: button::State,
    save: EmuSave,
}

impl Component for SaveButton {
    fn view(&mut self) -> Element<Message, Renderer> {
        Button::new(&mut self.state, Text::new(format!("{}", self.save)))
            .on_press(Message::OpenSave(self.save.clone()))
            .into()
    }
}

impl SaveButton {
    pub fn new(location: PathBuf, emu_type: EmuType) -> SaveButton {
        SaveButton {
            state: button::State::new(),
            save: EmuSave::new(location, emu_type),
        }
    }
}
