use crate::components::SmmdbCoursePanel;

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use smmdb_lib::proto::SMM2Course::SMM2Course;
use std::io::{self, ErrorKind};

#[derive(Debug)]
pub struct Smmdb {
    client: Client,
    query_params: QueryParams,
    course_panels: Vec<SmmdbCoursePanel>,
}

impl Smmdb {
    pub fn new() -> Smmdb {
        Smmdb {
            client: Client::new(),
            query_params: serde_json::from_str::<QueryParams>("{}").unwrap(),
            course_panels: vec![],
        }
    }

    pub fn set_courses(&mut self, courses: Vec<Course2Response>) -> &Vec<SmmdbCoursePanel> {
        self.course_panels = courses.into_iter().map(SmmdbCoursePanel::new).collect();
        &self.course_panels
    }

    pub fn get_course_panels(&mut self) -> &mut Vec<SmmdbCoursePanel> {
        &mut self.course_panels
    }

    pub fn get_query_params(&self) -> &QueryParams {
        &self.query_params
    }

    pub async fn update(query_params: QueryParams) -> Result<Vec<Course2Response>> {
        let qs = serde_qs::to_string(&query_params)
            .map_err(|err| io::Error::new(ErrorKind::Other, err.to_string()))?;
        dbg!(&qs);
        let body = Client::new()
            .get(&format!("https://api.smmdb.net/courses2?{}", qs))
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryParams {
    #[serde(default = "limit_default")]
    limit: u32,
    #[serde(default)]
    skip: Option<u32>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    ids: Option<Vec<String>>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    title_exact: bool,
    #[serde(default)]
    title_case_sensitive: bool,
    #[serde(default = "is_true")]
    title_trimmed: bool,
    #[serde(default)]
    owner: Option<String>,
    #[serde(default)]
    uploader: Option<String>,
    #[serde(default)]
    sort: Option<Vec<Sort>>,
    #[serde(default)]
    difficulty: Option<Difficulty>,
}

fn limit_default() -> u32 {
    50
}

fn is_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
