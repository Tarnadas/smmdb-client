use crate::{components::CoursePanel, generate_page, styles::*, Component, Page};

use iced::{scrollable, Element, Scrollable};

pub struct SaveWidget {
    state: scrollable::State,
}

impl SaveWidget {
    pub fn new() -> SaveWidget {
        SaveWidget {
            state: scrollable::State::new(),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        course_panels: Vec<&'a mut CoursePanel>,
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
