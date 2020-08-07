use crate::{styles::*, Component};

use iced::{Column, Text};

mod init;

pub use init::InitPage;

pub enum PageType {
    Init,
}

pub trait Page {
    fn view<'a>(
        &'a mut self,
        components: &'a mut Vec<Box<dyn Component + '_>>,
    ) -> Column<crate::Message>;
}

pub fn generate_page<'a>(
    title: &str,
    content: Column<'a, crate::Message>,
) -> Column<'a, crate::Message> {
    Column::new()
        .push(Text::new(title).size(36))
        .push(content)
        .padding(CONTAINER_PADDING)
        .spacing(PAGE_SPACING)
}