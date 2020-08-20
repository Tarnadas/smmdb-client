use crate::{Component, Message};

use iced::{
    container::{Style, StyleSheet},
    Align, Color, Column, Container, Image, Length, Row, Space, Text,
};
use iced_native::{widget::image::Handle, Element};
use iced_wgpu::Renderer;
use smmdb_lib::SavedCourse;

#[derive(Clone, Debug)]
pub struct CoursePanel {
    course: SavedCourse,
}

impl Component for CoursePanel {
    fn view(&mut self) -> Element<Message, Renderer> {
        let course = self.course.get_course();
        let course_header = course.get_course().get_header();

        let content = Column::new()
            .push(Text::new(format!("{}", course_header.get_title())).size(24))
            .push(Space::with_height(Length::Units(10)))
            .push(
                Row::new()
                    .push(
                        Container::new(Image::new(Handle::from_memory(
                            course.get_course_thumb().unwrap().clone().take_jpeg(),
                        )))
                        .max_width(320),
                    )
                    .push(Space::with_width(Length::Units(10)))
                    .push(
                        Text::new(format!("{}", course_header.get_description()))
                            .size(15)
                            .width(Length::Fill),
                    )
                    .align_items(Align::Center),
            )
            .width(Length::Shrink);

        Container::new(content)
            .style(CoursePanelStyle)
            .padding(12)
            .width(Length::Units(480))
            .into()
    }
}

impl CoursePanel {
    pub fn new(course: SavedCourse) -> CoursePanel {
        CoursePanel { course }
    }
}

struct CoursePanelStyle;

impl StyleSheet for CoursePanelStyle {
    fn style(&self) -> Style {
        Style {
            border_color: Color::BLACK,
            border_radius: 4,
            border_width: 1,
            ..Style::default()
        }
    }
}
