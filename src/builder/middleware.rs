use std::sync::Arc;
use reqwest::header;
use reqwest::cookie::CookieStore;

use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

pub fn get_default_client<T>(
    cookie_store: Arc<T>,
    user_agent: &'static str,
) -> ClientWithMiddleware
where
    T: Sized + CookieStore + 'static
{
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let mut headers = header::HeaderMap::new();
    
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(user_agent));
    
    let client = reqwest::ClientBuilder::new()
        .cookie_provider(cookie_store)
        .default_headers(headers)
        .connection_verbose(false)
        .build()
        .unwrap();
    
    ClientBuilder::new(client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}