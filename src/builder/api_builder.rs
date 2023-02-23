use super::middleware::get_default_middleware;
use crate::BackpackAPI;
use std::sync::Arc;
use reqwest_middleware::ClientWithMiddleware;
use reqwest::cookie::Jar;

const USER_AGENT_STRING: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36";

#[derive(Debug, Clone)]
pub struct BackpackAPIBuilder {
    key: Option<String>,
    token: Option<String>,
    client: Option<ClientWithMiddleware>,
    user_agent: &'static str,
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
            client: None,
            user_agent: USER_AGENT_STRING,
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
    
    pub fn client(mut self, client: ClientWithMiddleware) -> Self {
        self.client = Some(client);
        self
    }
    
    pub fn user_agent(mut self, user_agent: &'static str) -> Self {
        self.user_agent = user_agent;
        self
    }
    
    pub fn build(self) -> BackpackAPI {
        let cookies = Arc::new(Jar::default());
        let client = self.client.unwrap_or_else(|| {
            get_default_middleware(
                Arc::clone(&cookies),
                self.user_agent,
            )
        });
        
        BackpackAPI::new(
            self.key,
            self.token,
            client,
        )
    }
}