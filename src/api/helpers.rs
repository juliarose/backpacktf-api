use serde::{Deserialize, de::DeserializeOwned};
use crate::error::Error;
use std::time::{Instant, Duration};
use reqwest::header::RETRY_AFTER;
use log::error;

/// Handles rate limits for requests that are split into chunks.
pub struct Cooldown<'a, T> {
    start_time: Instant,
    i: usize,
    chunk_i: usize,
    limit: usize,
    cooldown: u64,
    chunks: Vec<&'a [T]>,
}

impl<'a, T> Cooldown<'a, T> 
where
    T: Sized
{
    pub fn new(
        data: &'a [T],
    ) -> Self {
        let chunks: Vec<_> = data.chunks(100).collect();
        
        Self {
            start_time: Instant::now(),
            i: 0,
            chunk_i: 0,
            limit: 10,
            cooldown: 60,
            chunks,
        }
    }
    
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
    
    pub fn go_back(&mut self) {
        self.reset();
        
        if self.i > 0 {
            log::info!("Move chunk back from {} to {}", self.i, self.i - 1);
            self.chunk_i = 0;
            self.i -= 1;
        }
    }
    
    pub fn next(&mut self) -> Option<(&'a [T], Option<Duration>)> {
        self.i += 1;

        log::info!("Getting chunk {}", self.i);
        
        if let Some(chunk) = self.chunks.get(self.i - 1) {
            if self.i >= self.chunks.len() || self.chunk_i >= self.limit {
                self.chunk_i = 0;
                
                Some((chunk, None))
            } else {
                let elapsed = self.start_time.elapsed().as_secs() + 1;
                let wait = if elapsed < self.cooldown {
                    0
                } else {
                    elapsed - self.cooldown
                };
                
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
            
            Err(Error::Response("bad".into()))
        },
        500..=599 => {
            Err(Error::Http(response))
        },
        _ => {
            let body = &response
                .bytes()
                .await?;
            // Print the body
            // let text = std::str::from_utf8(&body).unwrap();
            // println!("BODY: {}", text);

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