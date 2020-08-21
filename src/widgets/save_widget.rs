use crate::{components::CoursePanel, font, styles::*, Component};

use iced::{scrollable, Element, Font, Length, Scrollable, Text};
use std::path::PathBuf;

pub struct SaveWidget {
    state: scrollable::State,
    course_panels: Vec<CoursePanel>,
}

impl SaveWidget {
    pub fn new(save: &smmdb_lib::Save) -> SaveWidget {
        let course_panels = save
            .get_own_courses()
            .iter()
            .map(|course| CoursePanel::new(course.clone()))
            .collect();
        SaveWidget {
            state: scrollable::State::new(),
            course_panels,
        }
    }

    pub fn view<'a>(&'a mut self, path: &PathBuf) -> Element<crate::Message> {
        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .push(Text::new(format!("{:?}", path)).font(font::SMME));
        for panel in self.course_panels.iter_mut() {
            content = content.push(panel.view());
        }

        content.width(Length::FillPortion(1)).into()
    }
}
