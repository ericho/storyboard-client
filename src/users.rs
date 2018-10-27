use Client;
use Error;

/// Represents an user.
#[derive(Clone, Serialize, Default, Deserialize, Debug)]
pub struct User {
    /// The email of the user.
    pub email: String,
    /// To check of the user is allowed to login.
    pub enable_login: bool,
    /// The full name of the user.
    pub full_name: String,
    /// To check if the user has superuser privileges.
    pub is_superuser: bool,
    /// The `openid` string
    pub openid: String,
    /// The ID of the user.
    pub id: i32,
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

    /// Search user with the given email.
    pub fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        let url = format!("{}/users?email={}", self.uri, email);
        let users: Vec<User> = self.fetch_url(&url)?;
        if users.len() == 1 {
            Ok(users[0].clone())
        } else {
            Err(Error::OtherError)
        }
    }
}
