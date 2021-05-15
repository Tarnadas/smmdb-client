use crate::{components::SaveButton, font::*, styles::*, AppErrorState, AppState, Message};

use iced::{button, Button, Column, Element, Length, Space, Text};

#[derive(Clone, Debug)]
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
    pub fn view(&mut self, state: &AppState, error_state: &AppErrorState) -> Element<Message> {
        let mut content = self.save_buttons.iter_mut().fold(
            Column::new()
                .padding(CONTAINER_PADDING)
                .spacing(LIST_SPACING),
            |acc, save_button| acc.push(save_button.view(state)),
        );

        let mut custom_save_button = Button::new(
            &mut self.open_custom_save,
            Text::new("Select another save folder"),
        )
        .padding(BUTTON_PADDING)
        .style(DefaultButtonStyle);
        custom_save_button = match state {
            AppState::Loading => custom_save_button,
            _ => custom_save_button.on_press(Message::OpenCustomSave),
        };
        content = content.push(custom_save_button);

        content = if let AppErrorState::Some(err) = error_state {
            content.push(Space::with_height(Length::Units(16))).push(
                Text::new(err)
                    .font(HELVETICA_BOLD)
                    .size(22)
                    .color(COLOR_DARK_RED),
            )
        } else {
            content
        };

        Column::new()
            .push(Text::new("Please select your save folder").size(36))
            .push(content)
            .padding(CONTAINER_PADDING)
            .spacing(PAGE_SPACING)
            .into()
    }
}
