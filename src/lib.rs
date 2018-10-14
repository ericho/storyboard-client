extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod projects;
mod stories;
mod tasks;
mod client;

pub use projects::{Project, ProjectGroup};
pub use stories::{Story, Team, User};
pub use tasks::{Task, TaskStatusCount};

/// A client type to connect to the StoryBoard API
pub struct Client {
    /// The uri of the API.
    uri: String
}
