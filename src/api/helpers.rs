use serde::{Deserialize, de::DeserializeOwned};
use crate::error::Error;
use std::time::{Instant, Duration};
use reqwest::header::RETRY_AFTER;
use log::error;

/// Handles rate limits for requests that are split into chunks.
pub struct Cooldown<'a, T> {
    start_time: Instant,
    i: usize,
    limit: usize,
    cooldown: u64,
    chunks: std::iter::Peekable<core::slice::Chunks<'a, T>>,
}

impl<'a, T> Cooldown<'a, T> 
where
    T: Sized
{
    
    pub fn new(
        data: &'a [T],
    ) -> Self {
        let chunks = data.chunks(100).peekable();
        
        Self {
            start_time: Instant::now(),
            i: 0,
            limit: 10,
            cooldown: 60,
            chunks,
        }
    }
    
    pub fn reset(&mut self) {
        self.i = 0;
    }
    
    pub fn next(&mut self) -> Option<(&'a [T], Option<Duration>)> {
        self.i += 1;
        
        if let Some(chunk) = self.chunks.next() {
            if self.chunks.peek().is_none() || self.i % self.limit != 0  {
                Some((chunk, None))
            } else {
                let elapsed = self.start_time.elapsed().as_secs() + 1;
                let wait = if elapsed >= self.cooldown {
                    0
                } else {
                    elapsed - self.cooldown
                };
                
                self.start_time = Instant::now();
                
                Some((chunk, Some(Duration::from_secs(wait))))
            }
        } else {
            None
        }
    }
}

pub async fn parses_response<D>(response: reqwest::Response) -> Result<D, Error>
where
    D: DeserializeOwned
{
    #[derive(Deserialize, Debug)]
    struct ErrorResponse {
        message: String,
    }
    
    let status = &response.status();
    
    match status.as_u16() {
        100..=199 => {
            Err(Error::Http(response))
        },
        300..=399 => {
            Err(Error::Http(response))
        },
        400..=499 => {
            if status.as_u16() == 429 {
                if let Some(retry_after_header) = &response.headers().get(RETRY_AFTER) {
                    let parsed = retry_after_header.to_str()
                        .map(|retry_after| retry_after.parse::<u64>());
                    
                    if let Ok(Ok(retry_after)) = parsed {
                        return Err(Error::TooManyRequests(retry_after));
                    }
                }
            }
            
            Err(Error::Http(response))
        },
        500..=599 => {
            Err(Error::Http(response))
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
                        Err(Error::Response(error_body.message))
                    } else {
                        error!("Error parsing body `{}`: {}", parse_error, String::from_utf8_lossy(body));
                        
                        Err(parse_error.into())
                    }
                }
            }
        }
    }
}