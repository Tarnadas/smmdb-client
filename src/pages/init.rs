use crate::{components::SaveButton, styles::*, Message};

use iced::{button, Button, Column, Element, Text};

pub struct InitPage {
    open_custom_save: button::State,
    save_buttons: Vec<SaveButton>,
}

impl InitPage {
    pub fn new(save_buttons: Vec<SaveButton>) -> InitPage {
        InitPage {
            open_custom_save: button::State::new(),
            save_buttons,
        }
    }
}

impl InitPage {
    pub fn view<'a>(&'a mut self) -> Element<crate::Message> {
        let mut content = self.save_buttons.iter_mut().fold(
            Column::new()
                .padding(CONTAINER_PADDING)
                .spacing(LIST_SPACING),
            |acc, save_button| acc.push(save_button.view()),
        );

        content = content.push(
            Button::new(
                &mut self.open_custom_save,
                Text::new("Select another save folder"),
            )
            .padding(BUTTON_PADDING)
            .style(DefaultButtonStyle)
            .on_press(Message::OpenCustomSave),
        );

        Column::new()
            .push(Text::new("Please select your save folder").size(36))
            .push(content)
            .padding(CONTAINER_PADDING)
            .spacing(PAGE_SPACING)
            .into()
    }
}
