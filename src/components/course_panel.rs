use crate::{icon, Message};

use iced::{
    button,
    container::{Style, StyleSheet},
    image, Align, Button, Color, Column, Container, Element, Image, Length, Row, Space, Text,
};
use smmdb_lib::SavedCourse;

#[derive(Clone, Debug)]
pub struct CoursePanel {
    add_state: button::State,
    delete_state: button::State,
    course: Option<SavedCourse>,
}

impl CoursePanel {
    pub fn new(course: Option<SavedCourse>) -> CoursePanel {
        CoursePanel {
            add_state: button::State::new(),
            delete_state: button::State::new(),
            course,
        }
    }

    pub fn view(&mut self, index: usize) -> Element<Message> {
        let content: Element<Message> = if let Some(course) = &self.course {
            let course = course.get_course();
            let course_header = course.get_course().get_header();

            Column::new()
                .push(Text::new(format!("{}", course_header.get_title())).size(24))
                .push(Space::with_height(Length::Units(10)))
                .push(
                    Row::new()
                        .push(
                            Container::new(Image::new(image::Handle::from_memory(
                                course.get_course_thumb().unwrap().clone().take_jpeg(),
                            )))
                            .max_width(240),
                        )
                        .push(Space::with_width(Length::Units(10)))
                        .push(
                            Text::new(format!("{}", course_header.get_description()))
                                .size(15)
                                .width(Length::Fill),
                        )
                        .align_items(Align::Center),
                )
                .width(Length::Shrink)
                .into()
        } else {
            Container::new(Text::new("empty").size(18).width(Length::Shrink))
                .width(Length::Fill)
                .height(Length::Units(80))
                .center_x()
                .center_y()
                .into()
        };

        let panel = Container::new(content)
            .style(CoursePanelStyle)
            .padding(12)
            .width(Length::Fill);

        let mut actions = Column::new();
        if self.course.is_some() {
            actions = actions
                .push(Button::new(
                    &mut self.add_state,
                    icon::SORT
                        .clone()
                        .width(Length::Units(24))
                        .height(Length::Units(24)),
                ))
                .push(Space::with_height(Length::Units(10)))
                .push(Button::new(
                    &mut self.delete_state,
                    icon::DELETE
                        .clone()
                        .width(Length::Units(24))
                        .height(Length::Units(24)),
                ));
        } else {
            actions = actions.push(Button::new(
                &mut self.add_state,
                icon::ADD
                    .clone()
                    .width(Length::Units(24))
                    .height(Length::Units(24)),
            ));
        }

        Row::new()
            .align_items(Align::Center)
            .push(panel)
            .push(Space::with_width(Length::Units(10)))
            .push(actions)
            .into()
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
