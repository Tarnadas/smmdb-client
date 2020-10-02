use crate::{components::SmmdbCoursePanel, Download, Progress};

use anyhow::Result;
use iced::Subscription;
use indexmap::IndexMap;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use smmdb_lib::proto::SMM2Course::SMM2Course;
use std::{
    collections::HashMap,
    fmt,
    io::{self, ErrorKind},
};

#[derive(Debug)]
pub struct Smmdb {
    client: Client,
    apikey: Option<String>,
    query_params: QueryParams,
    course_responses: HashMap<String, Course2Response>,
    course_panels: IndexMap<String, SmmdbCoursePanel>,
}

impl Smmdb {
    pub fn new(apikey: Option<String>) -> Smmdb {
        Smmdb {
            client: Client::new(),
            apikey,
            query_params: serde_json::from_str::<QueryParams>("{}").unwrap(),
            course_responses: HashMap::new(),
            course_panels: IndexMap::new(),
        }
    }

    pub fn set_courses(&mut self, courses: Vec<Course2Response>, update_panels: bool) {
        courses
            .iter()
            .cloned()
            .map(|course| (course.get_id().clone(), course))
            .for_each(|(smmdb_id, course_response)| {
                self.course_responses.insert(smmdb_id, course_response);
            });
        if update_panels {
            self.course_panels.clear();
            courses
                .into_iter()
                .map(SmmdbCoursePanel::new)
                .for_each(|course| {
                    self.course_panels.insert(course.get_id().clone(), course);
                });
        }
    }

    pub fn get_course_responses(&self) -> &HashMap<String, Course2Response> {
        &self.course_responses
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

    pub fn reset_pagination(&mut self) {
        self.query_params.skip = 0;
    }

    pub fn set_title(&mut self, title: String) {
        if let "" = title.as_ref() {
            self.query_params.title = None;
        } else {
            self.query_params.title = Some(title);
        }
    }

    pub fn set_uploader(&mut self, uploader: String) {
        if let "" = uploader.as_ref() {
            self.query_params.uploader = None;
        } else {
            self.query_params.uploader = Some(uploader);
        }
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        if let Difficulty::Unset = difficulty {
            self.query_params.difficulty = None;
        } else {
            self.query_params.difficulty = Some(difficulty);
        }
    }

    pub fn set_sort(&mut self, sort: SortOptions) {
        self.query_params.sort = Some(sort);
    }

    pub fn set_apikey(&mut self, apikey: String) {
        self.apikey = Some(apikey);
    }

    pub fn set_own_vote(&mut self, course_id: String, value: i32) {
        if let Some(course) = self.course_panels.get_mut(&course_id) {
            course.set_own_vote(value);
        }
        self.course_responses
            .get_mut(&course_id)
            .map(|course| course.set_own_vote(value));
    }

    pub async fn update(
        query_params: QueryParams,
        apikey: Option<String>,
    ) -> Result<Vec<Course2Response>> {
        let qs = serde_qs::to_string(&query_params)
            .map_err(|err| io::Error::new(ErrorKind::Other, err.to_string()))?;
        let mut client = Client::new().get(&format!("http://localhost:3030/courses2?{}", qs));
        if let Some(apikey) = apikey {
            client = client.header(header::AUTHORIZATION, &format!("APIKEY {}", apikey));
        }

        let body = client.send().await?.text().await?;
        let response: Vec<Course2Response> = serde_json::from_str(&body)?;
        Ok(response)
    }

    pub async fn fetch_thumbnail(id: String) -> Result<Vec<u8>> {
        let bytes = Client::new()
            .get(&format!(
                "http://localhost:3030/courses2/thumbnail/{}?size=m",
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
            url: format!("http://localhost:3030/courses2/download/{}", id),
        })
    }

    pub async fn try_sign_in(apikey: String) -> std::result::Result<(), String> {
        match Client::new()
            .post("http://localhost:3030/login")
            .header(header::AUTHORIZATION, &format!("APIKEY {}", apikey))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err("Could not sign in! Your API key seems to be wrong.".to_string())
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn vote(
        course_id: String,
        value: i32,
        apikey: String,
    ) -> std::result::Result<(), String> {
        let body = serde_json::to_string(&VoteBody { value }).map_err(|err| err.to_string())?;
        match Client::new()
            .post(&format!(
                "http://localhost:3030/courses2/vote/{}",
                course_id
            ))
            .header(header::AUTHORIZATION, &format!("APIKEY {}", apikey))
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err("Could not sign in! Your API key seems to be wrong.".to_string())
                }
            }
            Err(err) => Err(err.to_string()),
        }
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
    votes: i32,
    #[serde(default)]
    own_vote: i32,
    course: SMM2Course,
}

impl Course2Response {
    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_votes(&self) -> i32 {
        self.votes
    }

    pub fn get_own_vote(&self) -> i32 {
        self.own_vote
    }

    pub fn set_own_vote(&mut self, value: i32) {
        let diff = value - self.own_vote;
        self.votes = self.votes + diff;
        self.own_vote = value;
    }

    pub fn get_course(&self) -> &SMM2Course {
        &self.course
    }

    pub fn get_difficulty(&self) -> Option<&Difficulty> {
        self.difficulty.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Unset,
    Easy,
    Normal,
    Expert,
    SuperExpert,
}

impl Difficulty {
    pub const ALL: [Difficulty; 5] = [
        Difficulty::Unset,
        Difficulty::Easy,
        Difficulty::Normal,
        Difficulty::Expert,
        Difficulty::SuperExpert,
    ];
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Unset => write!(f, ""),
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Normal => write!(f, "Normal"),
            Difficulty::Expert => write!(f, "Expert"),
            Difficulty::SuperExpert => write!(f, "SuperExpert"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QueryParams {
    #[serde(default = "limit_default")]
    pub limit: u32,
    #[serde(default)]
    pub skip: u32,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub title_exact: bool,
    #[serde(default)]
    pub title_case_sensitive: bool,
    #[serde(default = "is_true")]
    pub title_trimmed: bool,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub uploader: Option<String>,
    #[serde(default)]
    pub sort: Option<SortOptions>,
    #[serde(default)]
    pub difficulty: Option<Difficulty>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoteBody {
    #[serde(default)]
    pub value: i32,
}

impl QueryParams {
    pub fn get_title(&self) -> &str {
        if let Some(title) = self.title.as_ref() {
            title
        } else {
            ""
        }
    }

    pub fn get_uploader(&self) -> &str {
        if let Some(uploader) = self.uploader.as_ref() {
            uploader
        } else {
            ""
        }
    }

    pub fn get_sort(&self) -> Option<SortOptions> {
        self.sort.clone()
    }

    pub fn get_difficulty(&self) -> Option<Difficulty> {
        self.difficulty.clone()
    }
}

fn limit_default() -> u32 {
    25
}

fn is_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Sort {
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

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
pub struct SortOptions(Vec<Sort>);

lazy_static! {
    pub static ref SORT_OPTIONS: [SortOptions; 2] = [
        SortOptions(vec![Sort {
            val: SortValue::LastModified,
            dir: -1,
        }]),
        SortOptions(vec![
            Sort {
                val: SortValue::Votes,
                dir: -1,
            },
            Sort {
                val: SortValue::LastModified,
                dir: -1,
            },
        ]),
    ];
}

impl fmt::Display for SortOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.get(0).unwrap().val {
            SortValue::LastModified => write!(f, "Last Modified"),
            SortValue::Uploaded => write!(f, "Uploaded"),
            SortValue::CourseHeaderTitle => write!(f, "Title"),
            SortValue::Votes => write!(f, "Votes"),
        }
    }
}

impl PartialEq for SortOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq, Serialize)]
pub enum SortValue {
    #[serde(rename = "last_modified")]
    LastModified,
    #[serde(rename = "uploaded")]
    Uploaded,
    #[serde(rename = "course.header.title")]
    CourseHeaderTitle,
    #[serde(rename = "votes")]
    Votes,
}
