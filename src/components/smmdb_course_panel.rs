use crate::{smmdb::Course2Response, Component, Message};

use iced::{
    container::{Style, StyleSheet},
    Align, Background, Color, Column, Container, Element, Image, Length, Row, Space, Text,
};
use iced_native::widget::image::Handle;

#[derive(Debug)]
pub struct SmmdbCoursePanel {
    course: Course2Response,
    thumbnail: Option<Vec<u8>>,
}

impl Component for SmmdbCoursePanel {
    fn view(&mut self) -> Element<Message> {
        let course = self.course.get_course();
        let course_header = course.get_header();

        let thumbnail: Element<Message> = if let Some(thumbnail) = &self.thumbnail {
            Image::new(Handle::from_memory(thumbnail.clone()))
                .width(Length::Units(240))
                .height(Length::Units(135))
                .into()
        } else {
            Space::new(Length::Units(240), Length::Units(135)).into()
        };

        let content = Column::new()
            .push(Text::new(format!("{}", course_header.get_title())).size(24))
            .push(Space::with_height(Length::Units(10)))
            .push(
                Row::new()
                    .push(Container::new(thumbnail).style(ThumbnailStyle))
                    .push(Space::with_width(Length::Units(10)))
                    .push(Text::new(format!("{}", course_header.get_description())).size(15))
                    .align_items(Align::Center),
            );

        Container::new(content)
            .style(SmmdbCoursePanelStyle)
            .padding(12)
            .width(Length::Units(480))
            .into()
    }
}

impl SmmdbCoursePanel {
    pub fn new(course: Course2Response) -> SmmdbCoursePanel {
        SmmdbCoursePanel {
            course,
            thumbnail: None,
        }
    }

    pub fn get_id(&self) -> &String {
        self.course.get_id()
    }

    pub fn set_thumbnail(&mut self, thumbnail: Vec<u8>) {
        self.thumbnail = Some(thumbnail);
    }
}

struct SmmdbCoursePanelStyle;

impl StyleSheet for SmmdbCoursePanelStyle {
    fn style(&self) -> Style {
        Style {
            border_color: Color::BLACK,
            border_radius: 4,
            border_width: 1,
            ..Style::default()
        }
    }
}

struct ThumbnailStyle;

impl StyleSheet for ThumbnailStyle {
    fn style(&self) -> Style {
        Style {
            background: Some(Background::Color(Color::from_rgba(0., 0., 0., 0.5))),
            ..Style::default()
        }
    }
}
