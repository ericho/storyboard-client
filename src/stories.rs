use chrono::{DateTime, Utc};
use std::error;
use Client;
use TaskStatusCount;
use Task;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Story {
    pub creator_id: Option<i32>,
    pub description: String,
    pub due_dates: Option<Vec<i32>>,
    pub is_bug: bool,
    pub private: bool,
    pub status: String,
    pub story_type_id: Option<i32>,
    pub tags: Vec<String>,
    pub task_statuses: Vec<TaskStatusCount>,
    pub teams: Vec<Team>,
    pub title: String,
    pub users: Vec<User>,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    updated_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    email: String,
    enable_login: bool,
    full_name: String,
    is_superuser: bool,
    last_login: DateTime<Utc>,
    openid: String,
    id: i32,
}

impl Client {
    pub fn search_stories(&self, s: &str) -> Result<Vec<Story>, Box<error::Error>> {
        let url = format!("{}/stories/search?q={}", self.uri, s);
        let stories: Vec<Story> = self.fetch_url(&url)?;
        Ok(stories)
    }

    pub fn get_tasks_in_story(&self, story: &Story) -> Result<Vec<Task>, Box<error::Error>> {
        let url = format!("{}/stories/{}/tasks", self.uri, story.id);
        let tasks: Vec<Task> = self.fetch_url(&url)?;
        Ok(tasks)
    }
}
