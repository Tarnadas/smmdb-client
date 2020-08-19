use crate::Message;

use iced_native::Element;
use iced_wgpu::Renderer;

mod course_panel;
mod save_button;
mod smmdb_course_panel;

pub use course_panel::*;
pub use save_button::*;
pub use smmdb_course_panel::*;

pub trait Component {
    fn view(&mut self) -> Element<Message, Renderer>;
}
