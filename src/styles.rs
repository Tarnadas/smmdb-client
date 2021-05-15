#![allow(dead_code)]

use iced::{button, container, pick_list, text_input, Background, Color};

// Spacings
pub const CONTAINER_PADDING: u16 = 20;
pub const TAB_PADDING: u16 = 12;
pub const BUTTON_PADDING: u16 = 8;
pub const TAB_BUTTON_PADDING: u16 = 6;
pub const PAGE_SPACING: u16 = 20;
pub const LIST_SPACING: u16 = 12;
pub const TAB_SPACING: u16 = 2;

// Colors
pub const COLOR_YELLOW: Color = Color::from_rgb(1., 0.812, 0.);
pub const COLOR_YELLOW2: Color = Color::from_rgb(1., 0.86, 0.2);
pub const COLOR_LIGHT_YELLOW: Color = Color::from_rgb(0.996, 0.952, 0.827);
pub const COLOR_LIGHTER_YELLOW: Color = Color::from_rgb(1., 0.992, 0.933);
pub const COLOR_GREEN: Color = Color::from_rgb(0., 0.592, 0.518);
pub const COLOR_LIGHT_GREEN: Color = Color::from_rgb(0.665, 0.941, 0.598);
pub const COLOR_LIGHTER_GREEN: Color = Color::from_rgb(0.85, 1., 0.85);
pub const COLOR_RED: Color = Color::from_rgb(1., 0., 0.);
pub const COLOR_DARK_RED: Color = Color::from_rgb(0.6, 0., 0.);
pub const COLOR_BROWN: Color = Color::from_rgb(0.38, 0.094, 0.129);
pub const COLOR_GRAY: Color = Color::from_rgb(0.4, 0.4, 0.4);

// Panels
pub const PANEL_ACTIVE: Background = Background::Color(Color::WHITE);
pub const PANEL_DOWNLOAD: Background = Background::Color(COLOR_LIGHTER_GREEN);
pub const PANEL_SELECT_ACTIVE: Background = Background::Color(COLOR_LIGHTER_GREEN);
pub const PANEL_SELECT_HOVER: Background = Background::Color(COLOR_LIGHT_GREEN);
pub const PANEL_TAB: Background = Background::Color(COLOR_YELLOW2);

// Buttons
pub const BUTTON_ACTIVE: Background = Background::Color(Color::WHITE);
pub const BUTTON_TAB_ACTIVE: Background = Background::Color(COLOR_LIGHT_YELLOW);
pub const BUTTON_SELECT_ACTIVE: Background = Background::Color(COLOR_LIGHTER_YELLOW);
pub const BUTTON_SELECT_CANCEL: Background = Background::Color(COLOR_RED);
pub const BUTTON_HOVER: Background = Background::Color(COLOR_LIGHT_GREEN);
pub const BUTTON_SELECT_HOVER: Background = Background::Color(COLOR_LIGHT_YELLOW);
pub const BUTTON_CONFIRM: Background = Background::Color(COLOR_GREEN);
pub const BUTTON_DANGER: Background = Background::Color(COLOR_RED);
pub const BUTTON_DISABLED: Background = Background::Color(COLOR_GRAY);

// TextInput
pub const TEXT_INPUT_ACTIVE: Background = Background::Color(Color::WHITE);
pub const TEXT_INPUT_FOCUS: Background = Background::Color(COLOR_LIGHTER_GREEN);
pub const TEXT_INPUT_PLACEHOLDER_COLOR: Color = COLOR_GRAY;
pub const TEXT_INPUT_VALUE_COLOR: Color = Color::BLACK;
pub const TEXT_INPUT_SELECTION_COLOR: Color = COLOR_LIGHT_GREEN;

// Text
pub const TEXT_HELP_COLOR: Color = COLOR_GRAY;
pub const TEXT_HIGHLIGHT_COLOR: Color = COLOR_GREEN;
pub const TEXT_DANGER_COLOR: Color = COLOR_RED;

// PickList
pub const PICK_LIST_MENU: Background = Background::Color(Color::WHITE);
pub const PICK_LIST_ACTIVE: Background = Background::Color(Color::WHITE);
pub const PICK_LIST_HOVER: Background = Background::Color(COLOR_LIGHTER_GREEN);
pub const PICK_LIST_SELECT: Background = Background::Color(COLOR_LIGHT_GREEN);

pub struct DefaultButtonStyle;

impl button::StyleSheet for DefaultButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_ACTIVE),
            border_radius: 4.,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_HOVER),
            border_radius: 4.,
            ..button::Style::default()
        }
    }
}

pub struct DefaultButtonDangerStyle;

impl button::StyleSheet for DefaultButtonDangerStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_ACTIVE),
            border_radius: 4.,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_DANGER),
            border_radius: 4.,
            ..button::Style::default()
        }
    }
}

pub struct DeleteButtonStyle;

impl button::StyleSheet for DeleteButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_ACTIVE),
            border_radius: 4.,
            border_width: 0.,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            background: Some(BUTTON_DANGER),
            border_radius: 4.,
            ..button::Style::default()
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(BUTTON_DISABLED),
            border_radius: 4.,
            ..button::Style::default()
        }
    }
}

pub struct TabButtonStyle(pub bool);

impl button::StyleSheet for TabButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(if self.0 {
                BUTTON_TAB_ACTIVE
            } else {
                BUTTON_ACTIVE
            }),
            border_radius: 2.,
            border_width: 1.,
            border_color: if self.0 {
                Color::BLACK
            } else {
                Color::TRANSPARENT
            },
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(if self.0 {
                BUTTON_TAB_ACTIVE
            } else {
                BUTTON_HOVER
            }),
            border_radius: 2.,
            border_width: 1.,
            border_color: if self.0 {
                Color::BLACK
            } else {
                Color::TRANSPARENT
            },
            ..button::Style::default()
        }
    }
}

pub struct TabContainerStyle;

impl container::StyleSheet for TabContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(PANEL_TAB),
            border_radius: 8.,
            ..container::Style::default()
        }
    }
}

pub struct DefaultTextInputStyle;

impl text_input::StyleSheet for DefaultTextInputStyle {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: TEXT_INPUT_ACTIVE,
            border_radius: 4.,
            ..text_input::Style::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: TEXT_INPUT_FOCUS,
            border_radius: 4.,
            ..text_input::Style::default()
        }
    }

    fn placeholder_color(&self) -> Color {
        TEXT_INPUT_PLACEHOLDER_COLOR
    }

    fn value_color(&self) -> Color {
        TEXT_INPUT_VALUE_COLOR
    }

    fn selection_color(&self) -> Color {
        TEXT_INPUT_SELECTION_COLOR
    }
}

pub struct DefaultPickListStyle;

impl pick_list::StyleSheet for DefaultPickListStyle {
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            background: PICK_LIST_MENU,
            selected_background: PICK_LIST_SELECT,
            ..pick_list::Menu::default()
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: PICK_LIST_ACTIVE,
            border_radius: 4.,
            ..pick_list::Style::default()
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: PICK_LIST_HOVER,
            border_radius: 4.,
            ..pick_list::Style::default()
        }
    }
}
