use std::fmt;
use reqwest;
use reqwest_middleware;
use anyhow;
use thiserror::Error;

pub const RESPONSE_UNSUCCESSFUL_MESSAGE: &str = "Empty response";

#[derive(Debug, Error)]
pub enum APIError {
    Parameter(&'static str),
    Response(String),
    Reqwest(reqwest::Error),
    ReqwestMiddleware(anyhow::Error),
    Status(reqwest::StatusCode),
    Parse(serde_json::Error),
    Http(reqwest::StatusCode),
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::Parameter(s) => write!(f, "{}", s),
            APIError::Response(s) => write!(f, "{}", s),
            APIError::Reqwest(e) => write!(f, "{}", e),
            APIError::ReqwestMiddleware(e) => write!(f, "{}", e),
            APIError::Status(e) => write!(f, "{}", e),
            APIError::Parse(e) => write!(f, "{}", e),
            APIError::Http(e) => write!(f, "{}", e),
        }
    }
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

impl From<serde_json::Error> for APIError {
    fn from(error: serde_json::Error) -> APIError {
        APIError::Parse(error)
    }
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> APIError {
        APIError::Reqwest(error)
    }
}