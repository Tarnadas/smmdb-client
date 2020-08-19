mod app;
mod components;
mod emu;
mod pages;
mod smmdb;
mod styles;
mod widgets;

pub use app::Message;
pub use components::Component;
pub use emu::{EmuSave, EmuType};
pub use pages::Page;
pub use smmdb::Smmdb;

fn main() {
    use app::*;
    use iced::{Application, Settings};

    App::run(Settings::default());
}
