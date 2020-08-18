use crate::{components::SmmdbCoursePanel, Component};

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use smmdb_lib::proto::SMM2Course::SMM2Course;
use std::io::{self, ErrorKind};

#[derive(Debug)]
pub struct Smmdb {
    client: Client,
    // courses: Vec<Course2Response>,
    query_params: QueryParams,
}

impl Smmdb {
    pub fn new() -> Smmdb {
        Smmdb {
            client: Client::new(),
            // courses: vec![],
            query_params: QueryParams::default(),
        }
    }

    pub fn set_courses(
        &mut self,
        courses: Vec<Course2Response>,
        components: &mut Vec<Box<dyn Component>>,
    ) {
        // self.courses = courses;

        components
            .into_iter()
            .map(|component| component.downcast_mut::<SmmdbCoursePanel>())
            .collect::<Vec<Option<&mut SmmdbCoursePanel>>>()
            .retain(|c| c.is_none());

        courses
            .into_iter()
            .map(SmmdbCoursePanel::new)
            .map(Box::new)
            .for_each(|course_panel| components.push(course_panel));
    }

    pub async fn update(query_params: QueryParams) -> Result<Vec<Course2Response>> {
        let qs = serde_qs::to_string(&query_params)
            .map_err(|err| io::Error::new(ErrorKind::Other, err.to_string()))?;
        dbg!(&qs);
        let body = Client::new()
            .get("https://api.smmdb.net/courses2")
            // .query(&qs)
            .send()
            .await?
            .text()
            .await?;
        let response: Vec<Course2Response> = serde_json::from_str(&body)?;
        Ok(response)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Course2Response {
    id: String,
    owner: String,
    uploader: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    difficulty: Option<Difficulty>,
    last_modified: i64,
    uploaded: i64,
    course: SMM2Course,
}

impl Course2Response {
    pub fn get_course(&self) -> &SMM2Course {
        &self.course
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Normal,
    Expert,
    SuperExpert,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct QueryParams {
    #[serde(default)]
    limit: u32,
    skip: Option<u32>,
    id: Option<String>,
    ids: Option<Vec<String>>,
    title: Option<String>,
    #[serde(default)]
    title_exact: bool,
    #[serde(default)]
    title_case_sensitive: bool,
    #[serde(default = "is_true")]
    title_trimmed: bool,
    owner: Option<String>,
    uploader: Option<String>,
    sort: Option<Vec<Sort>>,
    difficulty: Option<Difficulty>,
}

// fn is_true() -> bool {
//     true
// }

#[derive(Clone, Debug, Serialize)]
struct Sort {
    pub val: SortValue,
    dir: i32,
}

impl Default for Sort {
    fn default() -> Self {
        Sort {
            val: SortValue::LastModified,
            dir: -1,
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
enum SortValue {
    #[serde(rename = "last_modified")]
    LastModified,
    #[serde(rename = "uploaded")]
    Uploaded,
}
