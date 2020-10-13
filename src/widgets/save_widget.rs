use crate::{components::CoursePanel, font, smmdb::Course2Response, styles::*, AppState};

use iced::{scrollable, Element, Length, Scrollable, Text};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SaveWidget {
    state: scrollable::State,
    course_panels: Vec<CoursePanel>,
}

impl SaveWidget {
    pub fn new(save: &smmdb_lib::Save) -> SaveWidget {
        let course_panels = save
            .get_own_courses()
            .iter()
            .map(|course| CoursePanel::new(course.clone(), None))
            .collect();
        SaveWidget {
            state: scrollable::State::new(),
            course_panels,
        }
    }

    pub fn set_course_response(&mut self, courses: &HashMap<String, Course2Response>) {
        self.course_panels
            .iter_mut()
            .filter_map(|course_panel| {
                if let Some(smmdb_id) = course_panel.get_smmdb_id() {
                    if let Some(course_response) = courses.get(&smmdb_id) {
                        Some((course_response, course_panel))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .for_each(|(course_response, course_panel)| {
                course_panel.set_response(course_response.clone());
            })
    }

    pub fn view<'a>(
        &'a mut self,
        state: &AppState,
        display_name: &String,
        is_logged_in: bool,
    ) -> Element<crate::Message> {
        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING)
            .push(Text::new(display_name).font(font::SMME));
        for (index, panel) in self.course_panels.iter_mut().enumerate() {
            content = content.push(panel.view(state, index, is_logged_in));
        }

        content.width(Length::FillPortion(1)).into()
    }

    pub fn generate_course_panels(
        &mut self,
        save: &smmdb_lib::Save,
        course_responses: &HashMap<String, Course2Response>,
    ) {
        self.course_panels = save
            .get_own_courses()
            .iter()
            .map(|course| {
                let course_response = if let Some(course) = course {
                    if let Some(smmdb_id) = course.get_course().get_smmdb_id() {
                        course_responses
                            .get(&smmdb_id)
                            .map(|response| response.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };
                CoursePanel::new(course.clone(), course_response)
            })
            .collect();
    }
}
