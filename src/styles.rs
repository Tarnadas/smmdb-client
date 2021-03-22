#![allow(dead_code)]

use iced::{button, pick_list, text_input, Background, Color};

// Spacings
pub const CONTAINER_PADDING: u16 = 20;
pub const BUTTON_PADDING: u16 = 8;
pub const PAGE_SPACING: u16 = 20;
pub const LIST_SPACING: u16 = 12;

// Colors
pub const COLOR_YELLOW: Color = Color::from_rgb(1., 0.812, 0.);
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

// Buttons
pub const BUTTON_ACTIVE: Background = Background::Color(Color::WHITE);
pub const BUTTON_SELECT_ACTIVE: Background = Background::Color(COLOR_LIGHTER_YELLOW);
pub const BUTTON_SELECT_CANCEL: Background = Background::Color(COLOR_RED);
pub const BUTTON_HOVER: Background = Background::Color(COLOR_LIGHT_GREEN);
pub const BUTTON_SELECT_HOVER: Background = Background::Color(COLOR_LIGHT_YELLOW);
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
