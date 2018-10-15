use chrono::{DateTime, Utc};

use Client;
use Error;

/// Represents an user.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// The email of the user.
    email: String,
    /// To check of the user is allowed to login.
    enable_login: bool,
    /// The full name of the user.
    full_name: String,
    /// To check if the user has superuser privileges.
    is_superuser: bool,
    /// The last date when the user logged in.
    last_login: DateTime<Utc>,
    /// The `openid` string
    openid: String,
    /// The ID of the user.
    id: i32,
}

impl Client {
    /// Search users with the given search string
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
    ///     let users = client.search_users("erich")?;
    ///     assert_ne!(users.len(), 0);
    ///     Ok(())
    /// }
    /// ```
    pub fn search_users(&self, s: &str) -> Result<Vec<User>, Error> {
        let url = format!("{}/users/search?q={}", self.uri, s);
        let users: Vec<User> = self.fetch_url(&url)?;
        Ok(users)
    }

    /// Search users with the given email.
    // TODO: This function shall no return a Vec
    pub fn get_user_by_email(&self, email: &str) -> Result<Vec<User>, Error> {
        let url = format!("{}/users?email={}", self.uri, email);
        let users: Vec<User> = self.fetch_url(&url)?;
        Ok(users)
    }
}
