#[macro_use]
extern crate lazy_static;

mod app;
mod components;
mod download;
mod emu;
mod font;
mod icon;
mod pages;
mod settings;
mod smmdb;
mod styles;
mod widgets;

pub use app::{AppErrorState, AppState, Message};
pub use download::{Download, Progress};
pub use emu::{EmuSave, EmuType};
pub use pages::Page;
pub use settings::Settings;
pub use smmdb::Smmdb;

use anyhow::Result;

fn main() -> Result<()> {
    use app::*;
    use iced::{window, Application};
    
    std::env::set_var("RUST_BACKTRACE", "1");

    human_panic::setup_panic!();

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
    let settings = iced::Settings {
        antialiasing: false,
        window,
        default_font: Some(font::DEFAULT_FONT_BYTES),
        ..iced::Settings::default()
    };

    match App::run(settings) {
        Err(iced::Error::GraphicsAdapterNotFound) => {
            panic!("Your GPU does not seem to support the Vulkan graphics API");
        }
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
    .unwrap();
    Ok(())
}
