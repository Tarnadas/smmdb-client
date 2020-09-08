use crate::{components::CoursePanel, font, styles::*, AppState};

use iced::{scrollable, Element, Length, Scrollable, Text};

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

    pub fn view<'a>(
        &'a mut self,
        state: &AppState,
        display_name: &String,
    ) -> Element<crate::Message> {
        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .push(Text::new(display_name).font(font::SMME));
        for (index, panel) in self.course_panels.iter_mut().enumerate() {
            content = content.push(panel.view(state, index));
        }

        content.width(Length::FillPortion(1)).into()
    }

    pub fn generate_course_panels(&mut self, save: &smmdb_lib::Save) {
        self.course_panels = save
            .get_own_courses()
            .iter()
            .map(|course| CoursePanel::new(course.clone()))
            .collect();
    }
}
