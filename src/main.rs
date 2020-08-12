#[macro_use]
extern crate downcast_rs;

mod app;
mod components;
mod emu;
mod pages;
mod save_data;
mod styles;

pub use app::Message;
pub use components::Component;
pub use emu::{EmuSave, EmuType};
pub use pages::{generate_page, Page};
pub use save_data::SaveData;

fn main() {
    use app::*;
    use iced::{Application, Settings};

    App::run(Settings::default());
}
