use super::{CoursesWidget, UploadsWidget};
use crate::{font, styles::*, AppState, Message, Smmdb};

use iced::{
    button, scrollable, Align, Button, Container, Element, Length, Row, Scrollable, Space, Text,
};

#[derive(Clone, Debug, PartialEq)]
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
    ) -> impl Into<Element<crate::Message>> {
        let courses_button = Button::new(&mut self.courses_state, Text::new("Courses".to_string()))
            .style(TabButtonStyle(self.tab == SmmdbTab::Courses))
            .padding(TAB_BUTTON_PADDING)
            .on_press(Message::SetSmmdbTab(SmmdbTab::Courses));
        let uploads_button = Button::new(&mut self.uploads_state, Text::new("Uploads".to_string()))
            .style(TabButtonStyle(self.tab == SmmdbTab::Uploads))
            .padding(TAB_BUTTON_PADDING)
            .on_press(Message::SetSmmdbTab(SmmdbTab::Uploads));

        let tab_buttons = Row::new()
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .align_items(Align::Center)
            .push(courses_button)
            .push(uploads_button);

        let tab_content = Container::new(match self.tab {
            SmmdbTab::Courses => self.courses_widget.view(state, smmdb).into(),
            SmmdbTab::Uploads => self.uploads_widget.view(state, smmdb).into(),
        })
        .style(TabContainerStyle);

        Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(TAB_SPACING)
            .width(Length::FillPortion(1))
            .push(Text::new("SMMDB").font(font::SMME))
            .push(Space::with_height(Length::Units(8)))
            .push(tab_buttons)
            .push(tab_content)
    }
}
