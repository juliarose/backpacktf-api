#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Missing token")]
    MissingToken,
    #[error("Missing key")]
    MissingKey,
    #[error("Invalid parameter: {}", .0)]
    Parameter(&'static str),
    #[error("Unexpected response: {}", .0)]
    Response(String),
    #[error("Request error: {}", .0)]
    Reqwest(#[from] reqwest::Error),
    #[error("Request middleware error: {}", .0)]
    ReqwestMiddleware(anyhow::Error),
    #[error("Error parsing response: {}", .0)]
    Parse(#[from] serde_json::Error),
    #[error("{}", .0.status())]
    Http(reqwest::Response),
    #[error("Too many requests. Retry after {}s", .0)]
    TooManyRequests(u64),
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