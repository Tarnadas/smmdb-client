use crate::{components::CoursePanel, generate_page, styles::*, Component, Page};

use iced::Column;

pub struct SavePage;

impl SavePage {
    pub fn new() -> SavePage {
        SavePage {}
    }
}

impl Page for SavePage {
    fn view<'a>(
        &'a mut self,
        title: &str,
        components: &'a mut Vec<Box<dyn Component + '_>>,
    ) -> Column<crate::Message> {
        let mut content = Column::new()
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING);
        // for component in components.iter_mut() {
        //     if let Some(_course_panel) = component.downcast_mut::<CoursePanel>() {
        //         content = content.push(component.view());
        //     }
        // }

        generate_page(title, content)
    }
}
