extern crate reqwest;

/// A type that represents an error from the API.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    /// The debuginfo available.
    pub debuginfo: Option<String>,
    /// The failure code.
    pub faultcode: Option<String>,
    /// The failure description.
    pub faultstring: Option<String>,
}


#[derive(Debug)]
pub enum Error {
    ReqwestErr(reqwest::Error),
    OtherError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestErr(error)
    }
}
