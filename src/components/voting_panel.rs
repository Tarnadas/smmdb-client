use crate::{icon, styles::*, AppState, Message};

use iced::{
    button, container, Align, Background, Button, Color, Column, Element, Length, Space, Text,
};

#[derive(Clone, Debug)]
pub struct VotingPanel {
    upvote_state: button::State,
    downvote_state: button::State,
}

impl VotingPanel {
    pub fn new() -> VotingPanel {
        VotingPanel {
            upvote_state: button::State::new(),
            downvote_state: button::State::new(),
        }
    }

    pub fn view(&mut self, course_id: String, votes: i32, own_vote: i32) -> Element<Message> {
        let mut upvote = Button::new(
            &mut self.upvote_state,
            if own_vote > 0 {
                icon::UP_ARROW_GREEN.clone()
            } else {
                icon::UP_ARROW.clone()
            }
            .width(Length::Units(24))
            .height(Length::Units(24)),
        )
        .style(DefaultButtonStyle);
        upvote = match own_vote {
            n if n > 0 => upvote.on_press(Message::ResetCourseVote(course_id.clone())),
            _ => upvote.on_press(Message::UpvoteCourse(course_id.clone())),
        };
        let mut votes = Text::new(format!("{}", votes));
        match own_vote {
            n if n > 0 => {
                votes = votes.color(TEXT_HIGHLIGHT_COLOR);
            }
            n if n < 0 => {
                votes = votes.color(TEXT_DANGER_COLOR);
            }
            _ => {}
        };
        let mut downvote = Button::new(
            &mut self.downvote_state,
            if own_vote < 0 {
                icon::DOWN_ARROW_RED.clone()
            } else {
                icon::DOWN_ARROW.clone()
            }
            .width(Length::Units(24))
            .height(Length::Units(24)),
        )
        .style(DefaultButtonStyle);
        downvote = match own_vote {
            n if n < 0 => downvote.on_press(Message::ResetCourseVote(course_id.clone())),
            _ => downvote.on_press(Message::DownvoteCourse(course_id.clone())),
        };

        Column::new()
            .width(Length::Units(20))
            .align_items(Align::Center)
            .push(upvote)
            .push(Space::with_height(Length::Units(16)))
            .push(votes)
            .push(Space::with_height(Length::Units(16)))
            .push(downvote)
            .into()
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
