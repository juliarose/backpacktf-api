
use std::sync::Arc;
use reqwest_middleware::ClientWithMiddleware;
use reqwest::cookie::Jar;
use crate::BackpackAPI;
use super::middleware::get_default_middleware;

pub struct BackpackAPIBuilder {
    key: Option<String>,
    token: Option<String>,
    cookies: Option<Arc<Jar>>,
    client: Option<ClientWithMiddleware>,
}

impl Default for BackpackAPIBuilder {
    
    fn default() -> Self {
        Self::new()
    }
} 

impl BackpackAPIBuilder {
    
    pub fn new() -> Self {
        Self {
            key: None,
            token: None,
            cookies: None,
            client: None,
        }
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.into());
        self
    }
    
    pub fn middleware(mut self, client: ClientWithMiddleware) -> Self {
        self.client = Some(client);
        self
    }

    pub fn cookies(mut self, cookies: Arc<Jar>) -> Self {
        self.cookies = Some(cookies);
        self
    }
    
    pub fn build(self) -> BackpackAPI {
        let cookies = self.cookies.unwrap_or_else(|| Arc::new(Jar::default()));
        let client = self.client.unwrap_or_else(|| get_default_middleware(Arc::clone(&cookies)));
        
        BackpackAPI::new(self.key, self.token, cookies, client)
    }
}