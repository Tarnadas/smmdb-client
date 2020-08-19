use crate::{components::SmmdbCoursePanel, styles::*, Component};

use iced::{scrollable, Element, Length, Scrollable};

pub struct SmmdbWidget {
    state: scrollable::State,
}

impl SmmdbWidget {
    pub fn new() -> SmmdbWidget {
        SmmdbWidget {
            state: scrollable::State::new(),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        course_panels: &'a mut Vec<SmmdbCoursePanel>,
    ) -> Element<crate::Message> {
        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING);
        for panel in course_panels {
            content = content.push(panel.view());
        }

        content.width(Length::FillPortion(3)).into()
    }
}
