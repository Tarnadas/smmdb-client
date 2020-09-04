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

        match state {
            AppState::DownloadSelect(index) => Button::new(&mut self.panel_state, content)
                .style(SmmdbCoursePanelButtonStyle(state.clone()))
                .padding(12)
                .width(Length::Fill)
                .on_press(Message::DownloadCourse(
                    *index,
                    self.course.get_id().clone(),
                ))
                .into(),
            _ => Container::new(content)
                .style(SmmdbCoursePanelStyle)
                .padding(12)
                .width(Length::Fill)
                .into(),
        }
    }

    pub fn get_id(&self) -> &String {
        self.course.get_id()
    }

    pub fn set_thumbnail(&mut self, thumbnail: Vec<u8>) {
        self.thumbnail = Some(thumbnail);
    }
}

struct SmmdbCoursePanelButtonStyle(AppState);

impl button::StyleSheet for SmmdbCoursePanelButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: Color::BLACK,
            background: match self.0 {
                AppState::DownloadSelect(_) => Some(PANEL_SELECT_ACTIVE),
                _ => Some(PANEL_ACTIVE),
            },
            border_radius: 8,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::BLACK,
            background: match self.0 {
                AppState::DownloadSelect(_) => Some(PANEL_SELECT_HOVER),
                _ => Some(PANEL_ACTIVE),
            },
            border_radius: 8,
            ..button::Style::default()
        }
    }
}

struct SmmdbCoursePanelStyle;

impl container::StyleSheet for SmmdbCoursePanelStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(PANEL_ACTIVE),
            border_radius: 8,
            ..container::Style::default()
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
