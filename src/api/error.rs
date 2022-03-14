use reqwest;
use reqwest_middleware;
use anyhow;
use thiserror::Error;

pub const RESPONSE_UNSUCCESSFUL_MESSAGE: &str = "Empty response";

#[derive(Error, Debug)]
pub enum APIError {
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
    #[error("{}", .0)]
    Http(reqwest::StatusCode),
}

impl From<reqwest_middleware::Error> for APIError {
    fn from(error: reqwest_middleware::Error) -> APIError {
        match error {
            reqwest_middleware::Error::Reqwest(e) => {
                APIError::Reqwest(e)
            },
            reqwest_middleware::Error::Middleware(e) => {
                APIError::ReqwestMiddleware(e)
            },
        }
    }
}