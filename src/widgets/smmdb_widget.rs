use super::{CoursesWidget, UploadsWidget};
use crate::{font, styles::*, AppState, Message, Smmdb};

use iced::{button, scrollable, Align, Button, Element, Length, Row, Scrollable, Text};

#[derive(Clone, Debug)]
pub enum SmmdbTab {
    Courses,
    Uploads,
}

#[derive(Clone, Debug)]
pub struct SmmdbWidget {
    tab: SmmdbTab,
    courses_widget: CoursesWidget,
    uploads_widget: UploadsWidget,
    state: scrollable::State,
    courses_state: button::State,
    uploads_state: button::State,
}

impl SmmdbWidget {
    pub fn new() -> SmmdbWidget {
        SmmdbWidget {
            tab: SmmdbTab::Courses,
            courses_widget: CoursesWidget::new(),
            uploads_widget: UploadsWidget::new(),
            state: scrollable::State::new(),
            courses_state: button::State::new(),
            uploads_state: button::State::new(),
        }
    }

    pub fn set_smmdb_tab(&mut self, tab: SmmdbTab) {
        self.tab = tab;
    }

    pub fn view<'a>(
        &'a mut self,
        state: &AppState,
        smmdb: &'a mut Smmdb,
        is_logged_in: bool,
    ) -> Element<crate::Message> {
        let courses_button = Button::new(&mut self.courses_state, Text::new("Courses".to_string()))
            .style(DefaultButtonStyle)
            .padding(BUTTON_PADDING)
            .on_press(Message::SetSmmdbTab(SmmdbTab::Courses));
        let uploads_button = Button::new(&mut self.uploads_state, Text::new("Uploads".to_string()))
            .style(DefaultButtonStyle)
            .padding(BUTTON_PADDING)
            .on_press(Message::SetSmmdbTab(SmmdbTab::Uploads));

        let tab_buttons = Row::new()
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .align_items(Align::Center)
            .push(courses_button)
            .push(uploads_button);

        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .width(Length::FillPortion(1))
            .push(Text::new("SMMDB").font(font::SMME))
            .push(tab_buttons);

        content = match self.tab {
            SmmdbTab::Courses => content.push(self.courses_widget.view(state, smmdb, is_logged_in)),
            SmmdbTab::Uploads => content.push(self.uploads_widget.view(state, smmdb, is_logged_in)),
        };

        content.into()
    }
}
