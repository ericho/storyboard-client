extern crate reqwest;

use std::env;
use serde::de::DeserializeOwned;

use Client;
use ApiError;
use ApiResult;
use Error;

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
        if let Ok(proxy) = get_proxy() {
            Client {
                uri: url.to_string(),
                client: reqwest::Client::builder()
                    .proxy(proxy)
                    .build().unwrap(),
            }
        } else {
            Client {
                uri: url.to_string(),
                client: reqwest::Client::new(),
            }
        }
    }

    /// Function that performs the request with the specified url.
    pub fn fetch_url<T: DeserializeOwned>(&self, url: &str)
                                      -> Result<T, Error> {
        let res: ApiResult<T, ApiError> = self.client.get(url).send()?.json()?;
        match res {
            ApiResult::Ok(v) => Ok(v),
            ApiResult::Err(_) => Err(Error::OtherError),
        }
    }
}

fn get_proxy() -> Result<reqwest::Proxy, Error> {
    let proxy = env::var("https_proxy")
        .or_else(|_| env::var("HTTPS_PROXY")).ok();
    match proxy {
        Some(n) => {
            let p = reqwest::Proxy::https(&n)?;
            Ok(p)
        },
        None => {
            Err(Error::OtherError)
        }
    }
}