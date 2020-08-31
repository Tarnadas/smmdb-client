#![allow(dead_code)]

use iced::{Background, Color};

pub const CONTAINER_PADDING: u16 = 20;
pub const PAGE_SPACING: u16 = 20;
pub const LIST_SPACING: u16 = 12;

pub const COLOR_SELECT: Color = Color::from_rgba(0.2, 0.2, 0.9, 0.2);
pub const BACKGROUND_SELECT: Background = Background::Color(COLOR_SELECT);

pub const COLOR_HOVER: Color = Color::from_rgba(0.1, 0.1, 0.8, 0.3);
pub const BACKGROUND_HOVER: Background = Background::Color(COLOR_HOVER);
