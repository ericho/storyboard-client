use chrono::{DateTime, Utc};

use std::error;

use Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    assignee_id: Option<i32>,
    branch_id: Option<i32>,
    creator_id: Option<i32>,
    due_dates: Option<Vec<DateTime<Utc>>>,
    link: Option<String>,
    milestone_id: Option<i32>,
    priority: Option<String>,
    project_id: Option<i32>,
    status: Option<String>,
    story_id: i32,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatusCount {
    count: Option<i32>,
    key: String,
}

impl Client {
    pub fn search_tasks(&self, s: &str) -> Result<Vec<Task>, Box<error::Error>> {
        let url = format!("{}/tasks/search?q={}", self.uri, s);
        let stories: Vec<Task> = self.fetch_url(&url)?;
        Ok(stories)
    }
}
