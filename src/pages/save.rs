use crate::{
    smmdb::Course2Response,
    widgets::{SaveWidget, SmmdbTab, SmmdbWidget},
    AppState, Smmdb,
};

use anyhow::Result;
use iced::{Element, Row};
use smmdb_lib::SavedCourse;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SavePage {
    save: smmdb_lib::Save,
    display_name: String,
    save_widget: SaveWidget,
    smmdb_widget: SmmdbWidget,
}

impl SavePage {
    pub fn new(
        save: smmdb_lib::Save,
        display_name: String,
        course_responses: &HashMap<String, Course2Response>,
    ) -> SavePage {
        SavePage {
            save_widget: SaveWidget::new(&save, course_responses),
            save,
            display_name,
            smmdb_widget: SmmdbWidget::new(),
        }
    }

    pub fn set_course_response(&mut self, courses: &HashMap<String, Course2Response>) {
        self.save_widget.set_course_response(courses);
    }

    pub fn set_smmdb_tab(&mut self, tab: SmmdbTab) {
        self.smmdb_widget.set_smmdb_tab(tab);
    }

    pub fn view<'a>(
        &'a mut self,
        state: &AppState,
        smmdb: &'a mut Smmdb,
        is_logged_in: bool,
    ) -> Element<crate::Message> {
        Row::new()
            .push(
                self.save_widget
                    .view(state, &self.display_name, is_logged_in),
            )
            .push(self.smmdb_widget.view(state, smmdb, is_logged_in))
            .into()
    }

    pub async fn swap_courses(
        &mut self,
        first: u8,
        second: u8,
        course_responses: &HashMap<String, Course2Response>,
    ) -> Result<()> {
        self.save.swap_course(first, second)?;
        self.save
            .save()
            .await
            .map_err(|err| -> anyhow::Error { err.into() })?;
        self.generate_course_panels(course_responses);
        Ok(())
    }

    pub async fn add_course(
        &mut self,
        index: u8,
        course: smmdb_lib::Course2,
        course_responses: &HashMap<String, Course2Response>,
    ) -> Result<()> {
        self.save.add_course(index, course)?;
        self.save
            .save()
            .await
            .map_err(|err| -> anyhow::Error { err.into() })?;
        self.generate_course_panels(course_responses);
        Ok(())
    }

    pub async fn delete_course(
        &mut self,
        index: u8,
        course_responses: &HashMap<String, Course2Response>,
    ) -> Result<()> {
        self.save.remove_course(index)?;
        self.save
            .save()
            .await
            .map_err(|err| -> anyhow::Error { err.into() })?;
        self.generate_course_panels(course_responses);
        Ok(())
    }

    fn generate_course_panels(&mut self, course_responses: &HashMap<String, Course2Response>) {
        self.save_widget
            .regenerate_course_panels(&self.save, course_responses);
    }
}
