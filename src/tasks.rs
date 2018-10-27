use chrono::{DateTime, Utc};

use Client;
use Error;
use ProjectGroup;
use User;

/// Represents a task from the storyboard API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// The ID of the user that this task is assigned.
    pub assignee_id: Option<i32>,
    /// The ID of the branch.
    pub branch_id: Option<i32>,
    /// The ID of the user that created this task.
    pub creator_id: Option<i32>,
    /// The due dates related with this task
    pub due_dates: Option<Vec<DateTime<Utc>>>,
    /// The link to the related resource to this task.
    pub link: Option<String>,
    /// The ID of the corresponding milestone.
    pub milestone_id: Option<i32>,
    /// The priority of this task.
    // TODO: Change this to an enum.
    pub priority: Option<String>,
    /// The ID of the corresponding project.
    pub project_id: Option<i32>,
    /// The status of this task.
    // TODO: Change this to an enum.
    pub status: Option<String>,
    /// The ID of the corresponding story.
    pub story_id: i32,
    /// An optional short label for this task.
    pub title: String,
}

/// A counter of task status changes.
///
/// This type is usually used internally.
// TODO: Change this type to be private.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStatusCount {
    /// The counter of changes
    pub count: Option<i32>,
    /// The key.
    pub key: String,
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
    /// fn example() -> Result<(), Error> {
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

    /// Get all tasks with the given `ProjectGroup`
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// use storyboard_client::{Client, Error, ProjectGroup};
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Error> {
    ///     let client = Client::new("https://storyboard.openstack.org/api/v1");
    ///     let group = ProjectGroup { id: 86, ..Default::default() };
    ///     let tasks = client.get_tasks_in_project_group(&group)?;
    ///     assert_ne!(tasks.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_tasks_in_project_group(&self, g: &ProjectGroup)
                                      -> Result<Vec<Task>, Error> {
        let url = format!("{}/tasks?project_group_id={}", self.uri, g.id);
        let tasks: Vec<Task> = self.fetch_url(&url)?;
        Ok(tasks)
    }

        /// Get all tasks with the given `ProjectGroup`
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// use storyboard_client::{Client, Error, User};
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Error> {
    ///     let client = Client::new("https://storyboard.openstack.org/api/v1");
    ///     let user = User {id : 4662, ..Default::default() };
    ///     let tasks = client.get_tasks_assigned_to(&user)?;
    ///     assert_ne!(tasks.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_tasks_assigned_to(&self, u: &User)
                                      -> Result<Vec<Task>, Error> {
        let url = format!("{}/tasks?assignee_id={}", self.uri, u.id);
        let tasks: Vec<Task> = self.fetch_url(&url)?;
        Ok(tasks)
    }
}
