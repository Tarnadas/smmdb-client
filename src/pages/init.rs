use crate::{components::SaveButton, generate_page, styles::*, Message, Page};

use iced::{button, Button, Column, Text};

pub struct InitPage {
    open_custom_save: button::State,
}

impl InitPage {
    pub fn new() -> InitPage {
        InitPage {
            open_custom_save: button::State::new(),
        }
    }
}

impl Page for InitPage {
    fn view<'a>(&'a mut self, save_buttons: &'a mut Vec<SaveButton>) -> Column<crate::Message> {
        let mut content = save_buttons.iter_mut().fold(
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
            .on_press(Message::OpenCustomSave),
        );

        generate_page("Please select your save folder", content)
    }
}
