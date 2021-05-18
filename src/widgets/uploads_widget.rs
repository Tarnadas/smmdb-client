use crate::{
    font::*,
    smmdb::{Difficulty, SortOptions},
    styles::*,
    AppState, Message, Smmdb,
};

use iced::{
    button, pick_list, text_input, Align, Button, Column, Element, Length, Row, Space, Text,
};

#[derive(Clone, Debug)]
pub struct UploadsWidget {
    title_state: text_input::State,
    uploader_state: text_input::State,
    difficulty_state: pick_list::State<Difficulty>,
    sort_state: pick_list::State<SortOptions>,
    search_state: button::State,
    backward_state: button::State,
    forward_state: button::State,
}

impl UploadsWidget {
    pub fn new() -> UploadsWidget {
        UploadsWidget {
            title_state: text_input::State::new(),
            uploader_state: text_input::State::new(),
            difficulty_state: pick_list::State::default(),
            sort_state: pick_list::State::default(),
            search_state: button::State::new(),
            backward_state: button::State::new(),
            forward_state: button::State::new(),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        state: &AppState,
        smmdb: &'a mut Smmdb,
    ) -> impl Into<Element<Message>> {
        let mut content = Column::new()
            .padding(TAB_PADDING)
            .spacing(LIST_SPACING)
            .width(Length::FillPortion(1));

        content = if smmdb.get_user().is_none() {
            content
                .align_items(Align::Center)
                .push(Space::with_height(Length::Units(36)))
                .push(
                    Text::new(
                        "You are not yet logged in! Please open settings and paste your API key.",
                    )
                    .font(HELVETICA_BOLD)
                    .size(20),
                )
                .push(Space::with_height(Length::Units(36)))
        } else if smmdb.get_own_course_panels().is_empty() {
            content
                .align_items(Align::Center)
                .push(Space::with_height(Length::Units(36)))
                .push(
                    Text::new("You don't yet have any uploaded courses!")
                        .font(HELVETICA_BOLD)
                        .size(20),
                )
                .push(Space::with_height(Length::Units(36)))
        } else {
            let query_params = smmdb.get_own_query_params();

            let paginate_text = Text::new(&format!(
                "{} – {}",
                query_params.skip + 1,
                query_params.skip + smmdb.get_own_course_panels().len() as u32
            ));

            let mut backward_button =
                Button::new(&mut self.backward_state, Text::new("<").size(24))
                    .style(DefaultButtonStyle);
            backward_button = match state {
                AppState::Loading | AppState::Downloading { .. } => backward_button,
                _ => {
                    if smmdb.can_self_paginate_backward() {
                        backward_button.on_press(Message::PaginateSelfBackward)
                    } else {
                        backward_button
                    }
                }
            };
            let mut forward_button = Button::new(&mut self.forward_state, Text::new(">").size(24))
                .style(DefaultButtonStyle);
            forward_button = match state {
                AppState::Loading | AppState::Downloading { .. } => forward_button,
                _ => {
                    if smmdb.can_self_paginate_forward() {
                        forward_button.on_press(Message::PaginateSelfForward)
                    } else {
                        forward_button
                    }
                }
            };

            let paginator = Row::new()
                .align_items(Align::Center)
                .push(Space::with_width(Length::Fill))
                .push(paginate_text)
                .push(Space::with_width(Length::Units(16)))
                .push(backward_button)
                .push(Space::with_width(Length::Units(16)))
                .push(forward_button);

            let smmdb_user = smmdb.get_user().cloned();
            for panel in smmdb.get_own_course_panels().values_mut() {
                content = content.push(panel.view(state, smmdb_user.as_ref()));
            }

            content.push(paginator)
        };

        content
    }
}
