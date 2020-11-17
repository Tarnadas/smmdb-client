#![allow(dead_code)]

use iced::Font;

pub const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/helvetica.ttf");

pub const HELVETICA: Font = Font::External {
    name: "Helvetica Bold",
    bytes: DEFAULT_FONT_BYTES,
};

pub const HELVETICA_BOLD: Font = Font::External {
    name: "Helvetica Bold",
    bytes: include_bytes!("../assets/fonts/helvetica-bold.ttf"),
};

pub const SMME: Font = Font::External {
    name: "Super Mario Maker Extended",
    bytes: include_bytes!("../assets/fonts/smme.ttf"),
};
