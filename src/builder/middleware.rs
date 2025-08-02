use std::sync::Arc;
use reqwest::{Client, ClientBuilder};
use reqwest::header;
use reqwest::cookie::CookieStore;

pub fn get_default_client<T>(
    cookie_store: Arc<T>,
    user_agent: &'static str,
) -> Client
where
    T: Sized + CookieStore + 'static
{
    let mut headers = header::HeaderMap::new();
    
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(user_agent));
    
    ClientBuilder::new()
        .cookie_provider(cookie_store)
        .default_headers(headers)
        .connection_verbose(false)
        .build()
        .unwrap()
}