use crate::{components::CoursePanel, Component};

use std::path::PathBuf;

pub struct SaveData {
    save: smmdb::Save,
    location: PathBuf,
}

impl SaveData {
    pub fn new(
        save: smmdb::Save,
        location: PathBuf,
        course_panels: &mut Vec<Box<dyn Component>>,
    ) -> SaveData {
        *course_panels = vec![];
        save.get_own_courses()
            .iter()
            .filter_map(|course| course.as_ref())
            .for_each(|course| {
                course_panels.push(Box::new(CoursePanel::new(course.clone())));
            });
        SaveData { save, location }
    }

    pub fn get_location(&self) -> &PathBuf {
        &self.location
    }
}
