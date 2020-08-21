use crate::{components::SmmdbCoursePanel, styles::*, Component};

use iced::{scrollable, Element, Length, Scrollable};
use indexmap::IndexMap;

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
        course_panels: &'a mut IndexMap<String, SmmdbCoursePanel>,
    ) -> Element<crate::Message> {
        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING);
        for panel in course_panels.values_mut() {
            content = content.push(panel.view());
        }

        content.width(Length::FillPortion(1)).into()
    }
}
