use crate::{icon, styles::*, AppState, Message};

use iced::{
    button, image, Align, Button, Color, Column, Container, Element, Image, Length, Row, Space,
    Text,
};
use smmdb_lib::SavedCourse;

#[derive(Clone, Debug)]
pub struct CoursePanel {
    panel_state: button::State,
    add_state: button::State,
    delete_state: button::State,
    course: Option<SavedCourse>,
}

impl CoursePanel {
    pub fn new(course: Option<SavedCourse>) -> CoursePanel {
        CoursePanel {
            panel_state: button::State::new(),
            add_state: button::State::new(),
            delete_state: button::State::new(),
            course,
        }
    }

    pub fn view(&mut self, state: &AppState, index: usize) -> Element<Message> {
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

        let panel = Button::new(&mut self.panel_state, content)
            .style(CoursePanelStyle(state.clone(), index))
            .padding(12)
            .width(Length::Fill)
            .on_press(match state {
                AppState::SwapSelect(idx) => Message::SwapCourse(*idx, index),
                _ => Message::Empty,
            });

        let mut actions = Column::new();
        if self.course.is_some() {
            actions = actions
                .push(
                    Button::new(
                        &mut self.add_state,
                        icon::SORT
                            .clone()
                            .width(Length::Units(24))
                            .height(Length::Units(24)),
                    )
                    .on_press(match state {
                        AppState::SwapSelect(_) => Message::ResetState,
                        AppState::Default => Message::InitSwapCourse(index),
                        AppState::DownloadSelect(_) => Message::Empty,
                    })
                    .style(SwapButtonStyle(state.clone(), index)),
                )
                .push(Space::with_height(Length::Units(10)))
                .push(Button::new(
                    &mut self.delete_state,
                    icon::DELETE
                        .clone()
                        .width(Length::Units(24))
                        .height(Length::Units(24)),
                ));
        } else {
            actions = actions.push(
                Button::new(
                    &mut self.add_state,
                    icon::ADD
                        .clone()
                        .width(Length::Units(24))
                        .height(Length::Units(24)),
                )
                .on_press(match state {
                    AppState::DownloadSelect(_) => Message::ResetState,
                    AppState::Default => Message::InitDownloadCourse(index),
                    AppState::SwapSelect(_) => Message::Empty,
                }),
            );
        }

        Row::new()
            .align_items(Align::Center)
            .push(panel)
            .push(Space::with_width(Length::Units(10)))
            .push(actions)
            .into()
    }
}

struct CoursePanelStyle(AppState, usize);

impl button::StyleSheet for CoursePanelStyle {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: Color::BLACK,
            background: match self.0 {
                AppState::SwapSelect(index) => {
                    if self.1 != index {
                        Some(BACKGROUND_SELECT)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            border_color: Color::BLACK,
            border_radius: 4,
            border_width: 1,
            ..button::Style::default()
        }
    }
}

struct SwapButtonStyle(AppState, usize);

impl button::StyleSheet for SwapButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: match self.0 {
                AppState::SwapSelect(index) => {
                    if self.1 == index {
                        Some(BACKGROUND_SELECT)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            border_radius: 4,
            ..button::Style::default()
        }
    }
}
