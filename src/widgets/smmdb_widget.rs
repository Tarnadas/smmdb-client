use crate::{font, styles::*, AppState, Message, Smmdb};

use iced::{
    button, scrollable, text_input, Align, Button, Column, Element, Length, Row, Scrollable, Space,
    Text, TextInput,
};

pub struct SmmdbWidget {
    state: scrollable::State,
    title_state: text_input::State,
    uploader_state: text_input::State,
    search_state: button::State,
    backward_state: button::State,
    forward_state: button::State,
}

impl SmmdbWidget {
    pub fn new() -> SmmdbWidget {
        SmmdbWidget {
            state: scrollable::State::new(),
            title_state: text_input::State::new(),
            uploader_state: text_input::State::new(),
            search_state: button::State::new(),
            backward_state: button::State::new(),
            forward_state: button::State::new(),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        state: &AppState,
        smmdb: &'a mut Smmdb,
    ) -> Element<crate::Message> {
        let query_params = smmdb.get_query_params();

        let title_text_input = TextInput::new(
            &mut self.title_state,
            "Title",
            query_params.get_title(),
            Message::TitleChanged,
        )
        .style(DefaultTextInputStyle)
        .padding(4);
        let uploader_text_input = TextInput::new(
            &mut self.uploader_state,
            "Uploader",
            query_params.get_uploader(),
            Message::UploaderChanged,
        )
        .style(DefaultTextInputStyle)
        .padding(4);
        let search_button = Button::new(&mut self.search_state, Text::new("Search"))
            .style(DefaultButtonStyle)
            .on_press(Message::ApplyFilters);

        let filter = Column::new()
            .push(title_text_input)
            .push(Space::with_height(Length::Units(4)))
            .push(uploader_text_input)
            .push(Space::with_height(Length::Units(4)))
            .push(search_button);

        let paginate_text = Text::new(&format!(
            "{} – {}",
            query_params.skip + 1,
            query_params.skip + smmdb.get_course_panels().len() as u32
        ));

        let mut backward_button = Button::new(&mut self.backward_state, Text::new("<").size(24))
            .style(DefaultButtonStyle);
        backward_button = if smmdb.can_paginate_backward() {
            backward_button.on_press(Message::PaginateBackward)
        } else {
            backward_button
        };
        let mut forward_button =
            Button::new(&mut self.forward_state, Text::new(">").size(24)).style(DefaultButtonStyle);
        forward_button = if smmdb.can_paginate_forward() {
            forward_button.on_press(Message::PaginateForward)
        } else {
            forward_button
        };

        let paginator = Row::new()
            .align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(paginate_text)
            .push(Space::with_width(Length::Units(16)))
            .push(backward_button)
            .push(Space::with_width(Length::Units(16)))
            .push(forward_button);

        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .push(Text::new("SMMDB").font(font::SMME))
            .push(filter)
            .push(Space::with_height(Length::Units(8)))
            .push(paginator);
        for panel in smmdb.get_course_panels().values_mut() {
            content = content.push(panel.view(state));
        }

        content.width(Length::FillPortion(1)).into()
    }
}
