#![allow(dead_code)]

use iced::{Background, Color};

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
pub const BUTTON_DISABLED: Background = Background::Color(COLOR_GRAY);
