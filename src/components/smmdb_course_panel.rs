use crate::{smmdb::Course2Response, styles::*, AppState, Message};

use iced::{
    button, container, Align, Background, Button, Color, Column, Container, Element, Image, Length,
    Row, Space, Text,
};
use iced_native::widget::image::Handle;

#[derive(Debug)]
pub struct SmmdbCoursePanel {
    panel_state: button::State,
    course: Course2Response,
    thumbnail: Option<Vec<u8>>,
}

impl SmmdbCoursePanel {
    pub fn new(course: Course2Response) -> SmmdbCoursePanel {
        SmmdbCoursePanel {
            panel_state: button::State::new(),
            course,
            thumbnail: None,
        }
    }

    pub fn view(&mut self, state: &AppState) -> Element<Message> {
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

        Button::new(&mut self.panel_state, content)
            .style(SmmdbCoursePanelStyle(state.clone()))
            .padding(12)
            .width(Length::Fill)
            .on_press(match state {
                AppState::DownloadSelect(index) => {
                    Message::DownloadCourse(*index, self.course.get_id().clone())
                }
                _ => Message::Empty,
            })
            .into()
    }

    pub fn get_id(&self) -> &String {
        self.course.get_id()
    }

    pub fn set_thumbnail(&mut self, thumbnail: Vec<u8>) {
        self.thumbnail = Some(thumbnail);
    }
}

struct SmmdbCoursePanelStyle(AppState);

impl button::StyleSheet for SmmdbCoursePanelStyle {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: Color::BLACK,
            background: match self.0 {
                AppState::DownloadSelect(_) => Some(BACKGROUND_SELECT),
                _ => None,
            },
            border_color: Color::BLACK,
            border_radius: 4,
            border_width: 1,
            ..button::Style::default()
        }
    }
}

struct ThumbnailStyle;

impl container::StyleSheet for ThumbnailStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgba(0., 0., 0., 0.5))),
            ..container::Style::default()
        }
    }
}
