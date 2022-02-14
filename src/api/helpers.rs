use std::sync::Arc;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest::{header, cookie::CookieStore};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::{Deserialize, de::DeserializeOwned};
use super::APIError;

pub fn get_default_middleware<T>(cookie_store: Arc<T>, user_agent_string: &'static str) -> ClientWithMiddleware
where
    T: CookieStore + 'static
{
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let mut headers = header::HeaderMap::new();
    
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(user_agent_string));
    
    let client = reqwest::ClientBuilder::new()
        .cookie_provider(cookie_store)
        .default_headers(headers)
        .build()
        .unwrap();
    
    ClientBuilder::new(client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}

pub async fn parses_response<D>(response: reqwest::Response) -> Result<D, APIError>
where
    D: DeserializeOwned
{
    #[derive(Deserialize, Debug)]
    struct ErrorResponse {
        message: String,
    }
    
    let status = &response.status();
    
    match status.as_u16() {
        300..=399 => {
            Err(APIError::Http(*status))
        },
        400..=499 => {
            Err(APIError::Http(*status))
        },
        500..=599 => {
            Err(APIError::Http(*status))
        },
        _ => {
            let body = &response
                .bytes()
                .await?;

            match serde_json::from_slice::<D>(body) {
                Ok(body) => Ok(body),
                Err(parse_error) => {
                    // unexpected response
                    if let Ok(error_body) = serde_json::from_slice::<ErrorResponse>(body) { 
                        Err(APIError::Response(error_body.message.into()))
                    } else {
                        Err(parse_error.into())
                    }
                }
            }
        }
    }
}