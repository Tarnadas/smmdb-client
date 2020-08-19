use crate::{
    components::{CoursePanel, SmmdbCoursePanel},
    widgets::{SaveWidget, SmmdbWidget},
};

use iced::{Element, Row};
use std::path::PathBuf;

pub struct SavePage {
    save: smmdb_lib::Save,
    location: PathBuf,
    save_widget: SaveWidget,
    smmdb_widget: SmmdbWidget,
}

impl SavePage {
    pub fn new(save: smmdb_lib::Save, location: PathBuf) -> SavePage {
        SavePage {
            save_widget: SaveWidget::new(&save),
            save,
            location,
            smmdb_widget: SmmdbWidget::new(),
        }
    }

    pub fn view<'a>(
        &'a mut self,
        smmdb_course_panels: &'a mut Vec<SmmdbCoursePanel>,
    ) -> Element<crate::Message> {
        Row::new()
            .push(self.save_widget.view())
            .push(self.smmdb_widget.view(smmdb_course_panels))
            .into()
    }
}
