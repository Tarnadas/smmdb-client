use crate::{font, styles::*, AppState, Message, Smmdb};

use iced::{button, scrollable, Align, Button, Element, Length, Row, Scrollable, Space, Text};

pub struct SmmdbWidget {
    state: scrollable::State,
    backward_state: button::State,
    forward_state: button::State,
}

impl SmmdbWidget {
    pub fn new() -> SmmdbWidget {
        SmmdbWidget {
            state: scrollable::State::new(),
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
        let paginate_text = Text::new(&format!(
            "{} – {}",
            query_params.skip + 1,
            query_params.skip + query_params.limit
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
            .push(paginator);
        for panel in smmdb.get_course_panels().values_mut() {
            content = content.push(panel.view(state));
        }

        content.width(Length::FillPortion(1)).into()
    }
}
