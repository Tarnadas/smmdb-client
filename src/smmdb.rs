use crate::{components::SmmdbCoursePanel, Download, Progress};

use anyhow::Result;
use iced::Subscription;
use indexmap::IndexMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use smmdb_lib::proto::SMM2Course::SMM2Course;
use std::io::{self, ErrorKind};

#[derive(Debug)]
pub struct Smmdb {
    client: Client,
    query_params: QueryParams,
    course_panels: IndexMap<String, SmmdbCoursePanel>,
}

impl Smmdb {
    pub fn new() -> Smmdb {
        Smmdb {
            client: Client::new(),
            query_params: serde_json::from_str::<QueryParams>("{}").unwrap(),
            course_panels: IndexMap::new(),
        }
    }

    pub fn set_courses(&mut self, courses: Vec<Course2Response>) {
        self.course_panels.clear();
        courses
            .into_iter()
            .map(SmmdbCoursePanel::new)
            .for_each(|course| {
                self.course_panels.insert(course.get_id().clone(), course);
            });
    }

    pub fn set_course_panel_thumbnail(&mut self, id: &String, thumbnail: Vec<u8>) {
        if let Some(course_panel) = self.course_panels.get_mut(id) {
            course_panel.set_thumbnail(thumbnail);
        }
    }

    pub fn get_course_panels(&mut self) -> &mut IndexMap<String, SmmdbCoursePanel> {
        &mut self.course_panels
    }

    pub fn get_query_params(&self) -> &QueryParams {
        &self.query_params
    }

    pub fn can_paginate_forward(&self) -> bool {
        self.course_panels.len() as u32 == self.query_params.limit
    }

    pub fn can_paginate_backward(&self) -> bool {
        self.query_params.skip > 0
    }

    pub fn paginate_forward(&mut self) {
        self.query_params.skip += self.query_params.limit;
    }

    pub fn paginate_backward(&mut self) {
        self.query_params.skip -= self.query_params.limit;
    }

    pub async fn update(query_params: QueryParams) -> Result<Vec<Course2Response>> {
        let qs = serde_qs::to_string(&query_params)
            .map_err(|err| io::Error::new(ErrorKind::Other, err.to_string()))?;
        let body = Client::new()
            .get(&format!("https://api.smmdb.net/courses2?{}", qs))
            .send()
            .await?
            .text()
            .await?;
        let response: Vec<Course2Response> = serde_json::from_str(&body)?;
        Ok(response)
    }

    pub async fn fetch_thumbnail(id: String) -> Result<Vec<u8>> {
        let bytes = Client::new()
            .get(&format!(
                "https://api.smmdb.net/courses2/thumbnail/{}?size=m",
                id
            ))
            .send()
            .await?
            .bytes()
            .await?;
        Ok(bytes.into_iter().collect())
    }

    pub fn download_course(id: String) -> Subscription<Progress> {
        Subscription::from_recipe(Download {
            url: format!("https://api.smmdb.net/courses2/download/{}", id),
        })
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
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_course(&self) -> &SMM2Course {
        &self.course
    }

    pub fn get_difficulty(&self) -> Option<&Difficulty> {
        self.difficulty.as_ref()
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
    pub limit: u32,
    #[serde(default)]
    pub skip: u32,
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
    25
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
