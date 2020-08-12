use crate::{Component, Message};

use iced::Text;
use iced_native::Element;
use iced_wgpu::Renderer;
use smmdb::SavedCourse;

#[derive(Clone, Debug)]
pub struct CoursePanel {
    course: SavedCourse,
}

impl Component for CoursePanel {
    fn view(&mut self) -> Element<Message, Renderer> {
        Text::new(format!(
            "{:?}",
            self.course
                .get_course()
                .get_course()
                .get_header()
                .get_title()
        ))
        .into()
    }
}

impl CoursePanel {
    pub fn new(course: SavedCourse) -> CoursePanel {
        CoursePanel { course }
    }
}
