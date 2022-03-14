use std::sync::Arc;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest::{header, cookie::CookieStore};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

const USER_AGENT_STRING: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

pub fn get_default_middleware<T>(cookie_store: Arc<T>) -> ClientWithMiddleware
where
    T: Sized + CookieStore + 'static
{
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let mut headers = header::HeaderMap::new();
    
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(USER_AGENT_STRING));
    
    let client = reqwest::ClientBuilder::new()
        .cookie_provider(cookie_store)
        .default_headers(headers)
        .build()
        .unwrap();
    
    ClientBuilder::new(client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}