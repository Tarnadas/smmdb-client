mod app;
mod components;
mod emu;
mod font;
mod pages;
mod smmdb;
mod styles;
mod widgets;

pub use app::Message;
pub use components::Component;
pub use emu::{EmuSave, EmuType};
pub use pages::Page;
pub use smmdb::Smmdb;

use anyhow::Result;

fn main() -> Result<()> {
    use app::*;
    use iced::{window, Application, Settings};

    let icon = match image::load_from_memory(include_bytes!("../assets/icons/icon.png")) {
        Ok(buffer) => {
            let buffer = buffer.to_rgba();
            let width = buffer.width();
            let height = buffer.height();
            let dynamic_image = image::DynamicImage::ImageRgba8(buffer);
            match iced::window::icon::Icon::from_rgba(dynamic_image.to_bytes(), width, height) {
                Ok(icon) => Some(icon),
                Err(_) => None,
            }
        }
        Err(_) => None,
    };
    let window = window::Settings {
        min_size: Some((960, 500)),
        icon,
        ..window::Settings::default()
    };
    let settings = Settings {
        window,
        default_font: Some(font::DEFAULT_FONT_BYTES),
        ..Settings::default()
    };

    App::run(settings);
    Ok(())
}
