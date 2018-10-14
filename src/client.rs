extern crate reqwest;
use std::error;
use serde::de::DeserializeOwned;

use Client;

impl Client {
    /// Creates a new client for the StoryBoard API.
    ///
    /// Receives the url of the API endpoint to perform the operations.
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate storyboard_client;
    ///
    /// use storyboard_client::Client;
    ///
    /// fn main() {
    ///     let client = Client::new("https://storyboard.openstack.org/api/v1");
    /// }
    /// ```
    pub fn new(url: &str) -> Client {
        Client {
            uri: url.to_string(),
        }
    }

    /// Function that performs the request with the specified url.
    pub fn fetch_url<T: DeserializeOwned>(&self, url: &str)
                                      -> Result<T, Box<error::Error>> {
        let res = reqwest::get(url)?.json()?;
        Ok(res)
    }
}
