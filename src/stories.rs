use chrono::{DateTime, Utc};

use Client;
use TaskStatusCount;
use Task;
use Error;
use User;

/// A type that represents a story in storyboard.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Story {
    /// The ID of the story creator.
    pub creator_id: Option<i32>,
    /// A complete description of the story.
    pub description: String,
    /// The due dates related with this story.
    ///
    /// There is a (possible) flaw in the design of the API. In the Task type
    /// the `due_dates` are retrieved as `DateTime`, however, in the story case,
    /// the API returns an array of integers.`
    pub due_dates: Option<Vec<i32>>,
    /// If the story is a bug or a feature.
    pub is_bug: bool,
    /// If the story is private.
    pub private: bool,
    /// The current status of the story.
    // TODO: Change this to an enum
    pub status: String,
    /// The ID of the story type.
    pub story_type_id: Option<i32>,
    /// The list of tags associated with this story.
    pub tags: Vec<String>,
    /// The statuses of the tasks within this story.
    pub task_statuses: Vec<TaskStatusCount>,
    /// The teams related with this story.
    pub teams: Vec<Team>,
    /// A descriptive label for the story.
    pub title: String,
    /// The set of users with permissions to see this story if it is private.
    pub users: Vec<User>,
    /// The ID of the story.
    pub id: i32,
}

/// Representation of a team
#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    /// The date when this story was updated.
    pub updated_at: Option<DateTime<Utc>>,
    /// The date when this story was created.
    pub created_at: DateTime<Utc>,
    /// The name of the team.
    pub name: String,
}

impl Client {

    /// Search stories with the given search string
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
    ///     let stories = client.search_stories("stx")?;
    ///     assert_ne!(stories.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn search_stories(&self, s: &str) -> Result<Vec<Story>, Error> {
        let url = format!("{}/stories/search?q={}", self.uri, s);
        let stories: Vec<Story> = self.fetch_url(&url)?;
        Ok(stories)
    }

    /// Gets all the tasks within a story.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// # use storyboard_client::{Client, Error, Story};
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Error> {
    /// #    let client = Client::new("https://storyboard.openstack.org/api/v1");
    ///     let story = Story { id: 19, ..Default::default() };
    ///     let tasks = client.get_tasks_in_story(&story)?;
    ///     assert_ne!(tasks.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_tasks_in_story(&self, story: &Story) -> Result<Vec<Task>, Error> {
        let url = format!("{}/stories/{}/tasks", self.uri, story.id);
        let tasks: Vec<Task> = self.fetch_url(&url)?;
        Ok(tasks)
    }
}
