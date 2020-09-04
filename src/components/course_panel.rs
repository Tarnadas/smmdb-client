use crate::{font::*, icon, styles::*, AppState, Message};

use iced::{
    button, container, image, Align, Button, Color, Column, Container, Element, Image, Length,
    ProgressBar, Row, Space, Text,
};
use smmdb_lib::SavedCourse;

#[derive(Clone, Debug)]
pub struct CoursePanel {
    panel_state: button::State,
    add_state: button::State,
    delete_state: button::State,
    delete_confirm_state: button::State,
    delete_cancel_state: button::State,
    course: Option<SavedCourse>,
}

impl CoursePanel {
    pub fn new(course: Option<SavedCourse>) -> CoursePanel {
        CoursePanel {
            panel_state: button::State::new(),
            add_state: button::State::new(),
            delete_state: button::State::new(),
            delete_confirm_state: button::State::new(),
            delete_cancel_state: button::State::new(),
            course,
        }
    }

    pub fn view(&mut self, state: &AppState, index: usize) -> Element<Message> {
        let content: Element<Message> = if let Some(course) = &self.course {
            let course = course.get_course();
            let course_header = course.get_course().get_header();

            let mut content = Column::new()
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
                .width(Length::Shrink);

            content = if let AppState::DeleteSelect(idx) = state {
                if *idx == index {
                    content
                        .push(Space::with_height(Length::Units(18)))
                        .push(
                            Text::new("Do you really want to delete this course?")
                                .size(16)
                                .font(HELVETICA_BOLD),
                        )
                        .push(
                            Row::new()
                                .push(Space::with_width(Length::Fill))
                                .push(
                                    Button::new(
                                        &mut self.delete_cancel_state,
                                        Text::new("No").size(20).font(HELVETICA_BOLD),
                                    )
                                    .padding(BUTTON_PADDING)
                                    .style(DefaultButtonStyle)
                                    .on_press(Message::ResetState),
                                )
                                .push(Space::with_width(Length::Units(16)))
                                .push(
                                    Button::new(
                                        &mut self.delete_confirm_state,
                                        Text::new("Yes").size(20).font(HELVETICA_BOLD),
                                    )
                                    .padding(BUTTON_PADDING)
                                    .style(DeleteButtonStyle)
                                    .on_press(Message::DeleteCourse(index)),
                                ),
                        )
                } else {
                    content
                }
            } else {
                content
            };

            content.into()
        } else {
            let empty_text = Text::new("empty").size(18).width(Length::Shrink);
            let content: Element<Message> = if let AppState::Downloading {
                save_index,
                progress,
                ..
            } = state
            {
                if *save_index == index {
                    ProgressBar::new(0.0..=100.0, *progress).into()
                } else {
                    empty_text.into()
                }
            } else {
                empty_text.into()
            };

            Container::new(content)
                .width(Length::Fill)
                .height(Length::Units(80))
                .center_x()
                .center_y()
                .into()
        };

        let panel: Element<Message> = match state {
            AppState::SwapSelect(idx) => Button::new(&mut self.panel_state, content)
                .style(CoursePanelButtonStyle(state.clone(), index))
                .padding(12)
                .width(Length::Fill)
                .on_press(Message::SwapCourse(*idx, index))
                .into(),
            _ => Container::new(content)
                .style(CoursePanelStyle(state.clone(), index))
                .padding(12)
                .width(Length::Fill)
                .into(),
        };

        let mut actions = Column::new();
        if self.course.is_some() {
            let mut swap_button = Button::new(
                &mut self.add_state,
                icon::SORT
                    .clone()
                    .width(Length::Units(24))
                    .height(Length::Units(24)),
            )
            .style(SwapButtonStyle(state.clone(), index));
            swap_button = match state {
                AppState::SwapSelect(idx) => {
                    if *idx == index {
                        swap_button.on_press(Message::ResetState)
                    } else {
                        swap_button.on_press(Message::InitSwapCourse(index))
                    }
                }
                AppState::Loading => swap_button,
                _ => swap_button.on_press(Message::InitSwapCourse(index)),
            };

            let mut delete_button = Button::new(
                &mut self.delete_state,
                icon::DELETE
                    .clone()
                    .width(Length::Units(24))
                    .height(Length::Units(24)),
            )
            .style(DeleteButtonStyle);
            delete_button = match state {
                AppState::DeleteSelect(idx) => {
                    if *idx == index {
                        delete_button.on_press(Message::ResetState)
                    } else {
                        delete_button.on_press(Message::InitDeleteCourse(index))
                    }
                }
                AppState::Loading => delete_button,
                _ => delete_button.on_press(Message::InitDeleteCourse(index)),
            };

            actions = actions
                .push(swap_button)
                .push(Space::with_height(Length::Units(10)))
                .push(delete_button);
        } else {
            let mut download_button = Button::new(
                &mut self.add_state,
                icon::ADD
                    .clone()
                    .width(Length::Units(24))
                    .height(Length::Units(24)),
            )
            .style(DownloadButtonStyle(state.clone(), index));
            download_button = match state {
                AppState::DownloadSelect(idx) => {
                    if *idx == index {
                        download_button.on_press(Message::ResetState)
                    } else {
                        download_button.on_press(Message::InitDownloadCourse(index))
                    }
                }
                AppState::Loading => download_button,
                _ => download_button.on_press(Message::InitDownloadCourse(index)),
            };

            actions = actions.push(download_button);
        }

        Row::new()
            .align_items(Align::Center)
            .push(panel)
            .push(Space::with_width(Length::Units(10)))
            .push(actions)
            .into()
    }
}

struct CoursePanelButtonStyle(AppState, usize);

impl button::StyleSheet for CoursePanelButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: Color::BLACK,
            background: match self.0 {
                AppState::SwapSelect(index) => {
                    if self.1 != index {
                        Some(PANEL_SELECT_ACTIVE)
                    } else {
                        Some(PANEL_ACTIVE)
                    }
                }
                _ => Some(PANEL_ACTIVE),
            },
            border_radius: 8,
            border_width: 0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::BLACK,
            background: match self.0 {
                AppState::SwapSelect(index) => {
                    if self.1 != index {
                        Some(PANEL_SELECT_HOVER)
                    } else {
                        Some(PANEL_ACTIVE)
                    }
                }
                AppState::DownloadSelect(index) => {
                    if self.1 == index {
                        Some(PANEL_DOWNLOAD)
                    } else {
                        Some(PANEL_ACTIVE)
                    }
                }
                _ => Some(PANEL_ACTIVE),
            },
            border_radius: 8,
            border_width: 0,
            ..button::Style::default()
        }
    }
}

struct CoursePanelStyle(AppState, usize);

impl container::StyleSheet for CoursePanelStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(PANEL_ACTIVE),
            border_radius: 8,
            border_width: 0,
            ..container::Style::default()
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
                        Some(BUTTON_SELECT_ACTIVE)
                    } else {
                        Some(BUTTON_ACTIVE)
                    }
                }
                _ => Some(BUTTON_ACTIVE),
            },
            border_radius: 4,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            background: match self.0 {
                AppState::SwapSelect(index) => {
                    if self.1 == index {
                        Some(BUTTON_SELECT_CANCEL)
                    } else {
                        Some(BUTTON_HOVER)
                    }
                }
                _ => Some(BUTTON_HOVER),
            },
            border_radius: 4,
            ..button::Style::default()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_DISABLED),
            border_radius: 4,
            ..button::Style::default()
        }
    }
}

struct DownloadButtonStyle(AppState, usize);

impl button::StyleSheet for DownloadButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: match self.0 {
                AppState::DownloadSelect(index) => {
                    if self.1 == index {
                        Some(BUTTON_SELECT_ACTIVE)
                    } else {
                        Some(BUTTON_ACTIVE)
                    }
                }
                _ => Some(BUTTON_ACTIVE),
            },
            border_radius: 4,
            border_width: 0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            background: match self.0 {
                AppState::DownloadSelect(index) => {
                    if self.1 == index {
                        Some(BUTTON_SELECT_CANCEL)
                    } else {
                        Some(BUTTON_HOVER)
                    }
                }
                _ => Some(BUTTON_HOVER),
            },
            border_radius: 4,
            ..button::Style::default()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_DISABLED),
            border_radius: 4,
            ..button::Style::default()
        }
    }
}

struct DeleteButtonStyle;

impl button::StyleSheet for DeleteButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_ACTIVE),
            border_radius: 4,
            border_width: 0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            background: Some(BUTTON_DANGER),
            border_radius: 4,
            ..button::Style::default()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_DISABLED),
            border_radius: 4,
            ..button::Style::default()
        }
    }
}
