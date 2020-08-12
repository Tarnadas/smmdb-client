use crate::Message;

use downcast_rs::DowncastSync;
use iced_native::Element;
use iced_wgpu::Renderer;

mod course_panel;
mod save_button;

pub use course_panel::*;
pub use save_button::*;

pub trait Component: DowncastSync {
    fn view(&mut self) -> Element<Message, Renderer>;
}
impl_downcast!(sync Component);
