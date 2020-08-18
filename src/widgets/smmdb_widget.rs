use crate::{components::SmmdbCoursePanel, generate_page, styles::*, Component, Page};

use iced::{scrollable, Element, Scrollable};

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
        course_panels: Vec<&'a mut SmmdbCoursePanel>,
    ) -> Element<crate::Message> {
        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING);
        for panel in course_panels {
            content = content.push(panel.view());
        }

        content.into()
    }
}
