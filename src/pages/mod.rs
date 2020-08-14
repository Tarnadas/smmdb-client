use crate::{styles::*, Component};

use downcast_rs::DowncastSync;
use iced::{Column, Element, Text};

mod init;
mod save;

pub use init::InitPage;
pub use save::SavePage;

pub trait Page: DowncastSync {
    fn view<'a>(
        &'a mut self,
        title: &str,
        components: &'a mut Vec<Box<dyn Component + '_>>,
    ) -> Element<crate::Message>;
}
impl_downcast!(sync Page);

pub fn generate_page<'a, T>(title: &str, content: T) -> Column<'a, crate::Message>
where
    T: Into<Element<'a, crate::Message>>,
{
    Column::new()
        .push(Text::new(title).size(36))
        .push(content)
        .padding(CONTAINER_PADDING)
        .spacing(PAGE_SPACING)
}
