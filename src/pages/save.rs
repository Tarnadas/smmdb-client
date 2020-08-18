use crate::{
    components::{CoursePanel, SmmdbCoursePanel},
    widgets::{SaveWidget, SmmdbWidget},
    Component, Page,
};

use iced::{Element, Row};

pub struct SavePage {
    save_widget: SaveWidget,
    smmdb_widget: SmmdbWidget,
}

impl SavePage {
    pub fn new() -> SavePage {
        SavePage {
            save_widget: SaveWidget::new(),
            smmdb_widget: SmmdbWidget::new(),
        }
    }
}

impl Page for SavePage {
    fn view<'a>(
        &'a mut self,
        title: &str,
        components: &'a mut Vec<Box<dyn Component + '_>>,
    ) -> Element<crate::Message> {
        let mut content = Row::new();

        let mut course_panels: Vec<&mut CoursePanel> = vec![];
        let mut smmdb_course_panels: Vec<&mut SmmdbCoursePanel> = vec![];

        components.into_iter().for_each(|component| {
            if let Some(course_panel) = component.downcast_mut::<CoursePanel>() {
                course_panels.push(course_panel);
            } else if let Some(course_panel) = component.downcast_mut::<SmmdbCoursePanel>() {
                smmdb_course_panels.push(course_panel);
            }
        });
        // .collect::<Vec<_>>()
        // .filter(|c| c.is_some())
        // .map(|c| c.unwrap())
        // .collect();

        // let smmdb_course_panels: Vec<&mut SmmdbCoursePanel> = components
        //     .into_iter()
        //     .map(|component| component.downcast_mut::<SmmdbCoursePanel>())
        //     // .collect::<Vec<_>>()
        //     .filter(|c| c.is_some())
        //     .map(|c| c.unwrap())
        //     .collect();

        content = content.push(self.save_widget.view(course_panels));
        content = content.push(self.smmdb_widget.view(smmdb_course_panels));

        content.into()
    }
}
