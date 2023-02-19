/// Any range of errors encountered when making requests.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Token is missing.
    #[error("Missing token")]
    MissingToken,
    /// Key is missing.
    #[error("Missing key")]
    MissingKey,
    /// An input parameter is missing or invalid.
    #[error("Invalid parameter: {}", .0)]
    Parameter(&'static str),
    /// An error was encountered making a request.
    #[error("Request error: {}", .0)]
    Reqwest(#[from] reqwest::Error),
    #[error("Request middleware error: {}", .0)]
    /// An error was encountered within the request middleware.
    ReqwestMiddleware(anyhow::Error),
    #[error("Error parsing response: {}", .0)]
    /// An error was encountered parsing a JSON response body.
    Parse(#[from] serde_json::Error),
    #[error("{}", .0.status())]
    /// An error was encountered on response. This is usually a response with an HTTP code other 
    /// than 200. Check the status code of the response for more information.
    Http(reqwest::Response),
    /// Unexpected response. Check the message for more details.
    #[error("Unexpected response: {}", .0)]
    Response(String),
}

impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Self {
        match error {
            reqwest_middleware::Error::Reqwest(e) => {
                Self::Reqwest(e)
            },
            reqwest_middleware::Error::Middleware(e) => {
                Self::ReqwestMiddleware(e)
            },
        }
    }
}