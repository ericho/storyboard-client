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
mod error;
mod users;

pub use projects::{Project, ProjectGroup};
pub use stories::{Story, Team};
pub use tasks::{Task, TaskStatusCount};
pub use error::{ApiError, Error};
pub use users::{User};

/// A client type to connect to the StoryBoard API
pub struct Client {
    /// The uri of the API.
    uri: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiResult<T, ApiError> {
    Ok(T),
    Err(ApiError),
}
