use crate::{styles::*, AppState, EmuSave, EmuType, Message};

use iced::{button, Button, Element, Text};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SaveButton {
    display_name: String,
    state: button::State,
    save: EmuSave,
}

impl SaveButton {
    pub fn new(display_name: String, location: PathBuf, emu_type: EmuType) -> SaveButton {
        SaveButton {
            display_name: display_name.clone(),
            state: button::State::new(),
            save: EmuSave::new(display_name, location, emu_type),
        }
    }

    pub fn view(&mut self, state: &AppState) -> Element<Message> {
        let mut save_button = Button::new(&mut self.state, Text::new(&self.display_name))
            .padding(BUTTON_PADDING)
            .style(SaveButtonStyle);
        save_button = match state {
            AppState::Loading => save_button,
            _ => save_button.on_press(Message::OpenSave(self.save.clone())),
        };
        save_button.into()
    }
}

struct SaveButtonStyle;

impl button::StyleSheet for SaveButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_ACTIVE),
            border_radius: 4,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_HOVER),
            border_radius: 4,
            ..button::Style::default()
        }
    }
}
