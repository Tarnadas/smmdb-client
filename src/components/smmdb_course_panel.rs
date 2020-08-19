use crate::{smmdb::Course2Response, Component, Message};

use iced::{
    container::{Style, StyleSheet},
    Align, Column, Container, Element, Image, Length, Row, Space, Text,
};

#[derive(Clone, Debug)]
pub struct SmmdbCoursePanel {
    course: Course2Response,
}

impl Component for SmmdbCoursePanel {
    fn view(&mut self) -> Element<Message> {
        let course = self.course.get_course();
        let course_header = course.get_header();

        let content = Column::new()
            .push(Text::new(format!("{}", course_header.get_title())).size(24))
            .push(Space::with_height(Length::Units(10)))
            .push(
                Row::new()
                    // .push(Image::new(Handle::from_memory(
                    //     course.get_course_thumb().unwrap().clone().take_jpeg(),
                    // )))
                    .push(Space::with_width(Length::Units(10)))
                    .push(Text::new(format!("{}", course_header.get_description())).size(15))
                    .align_items(Align::Center),
            );

        Container::new(content)
            .padding(12)
            .width(Length::Units(480))
            .height(Length::Units(170))
            .into()
    }
}

impl SmmdbCoursePanel {
    pub fn new(course: Course2Response) -> SmmdbCoursePanel {
        SmmdbCoursePanel { course }
    }
}
