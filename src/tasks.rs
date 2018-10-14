use chrono::{DateTime, Utc};

use Client;
use Error;

/// Represents a task from the storyboard API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// The ID of the user that this task is assigned.
    assignee_id: Option<i32>,
    /// The ID of the branch.
    branch_id: Option<i32>,
    /// The ID of the user that created this task.
    creator_id: Option<i32>,
    /// The due dates related with this task
    due_dates: Option<Vec<DateTime<Utc>>>,
    /// The link to the related resource to this task.
    link: Option<String>,
    /// The ID of the corresponding milestone.
    milestone_id: Option<i32>,
    /// The priority of this task.
    // TODO: Change this to an enum.
    priority: Option<String>,
    /// The ID of the corresponding project.
    project_id: Option<i32>,
    /// The status of this task.
    // TODO: Change this to an enum.
    status: Option<String>,
    /// The ID of the corresponding story.
    story_id: i32,
    /// An optional short label for this task.
    title: String,
}

/// A counter of task status changes.
///
/// This type is usually used internally.
// TODO: Change this type to be private.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatusCount {
    /// The counter of changes
    count: Option<i32>,
    /// The key.
    key: String,
}

impl Client {

    /// Search tasks with the given search string
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// use storyboard_client::{Client, Error};
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<Error>> {
    ///     let client = Client::new("https://storyboard.openstack.org/api/v1");
    ///     let tasks = client.search_tasks("stx")?;
    ///     assert_ne!(tasks.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn search_tasks(&self, s: &str) -> Result<Vec<Task>, Error> {
        let url = format!("{}/tasks/search?q={}", self.uri, s);
        let stories: Vec<Task> = self.fetch_url(&url)?;
        Ok(stories)
    }
}
