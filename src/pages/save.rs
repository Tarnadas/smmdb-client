use crate::{components::CoursePanel, generate_page, styles::*, Component, Page};

use iced::{scrollable, Element, Scrollable};

pub struct SavePage {
    state: scrollable::State,
}

impl SavePage {
    pub fn new() -> SavePage {
        SavePage {
            state: scrollable::State::new(),
        }
    }
}

impl Page for SavePage {
    fn view<'a>(
        &'a mut self,
        title: &str,
        components: &'a mut Vec<Box<dyn Component + '_>>,
    ) -> Element<crate::Message> {
        let mut content = Scrollable::new(&mut self.state)
            .padding(CONTAINER_PADDING)
            .spacing(LIST_SPACING);
        for component in components.iter_mut() {
            if let Some(_course_panel) = component.downcast_mut::<CoursePanel>() {
                content = content.push(component.view());
            }
        }

        generate_page(title, content).into()
    }
}
