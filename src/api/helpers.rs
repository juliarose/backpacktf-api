use serde::{Deserialize, de::DeserializeOwned};
use crate::error::Error;
use std::time::{Instant, Duration};
use reqwest::{header::RETRY_AFTER, StatusCode};
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
            self.chunk_i = 0;
            self.i -= 1;
        }
    }
    
    pub fn next(&mut self) -> Option<(&'a [T], Option<Duration>)> {
        if let Some(chunk) = self.chunks.get(self.i) {
            self.i += 1;
            self.chunk_i += 1;
            
            // we can skip the wait
            if 
                // if we have reached the end
                self.i == self.chunks.len() ||
                // or the current chunk index is under the limit
                self.chunk_i <= self.limit
            {
                Some((chunk, None))
            } else {
                let elapsed = self.start_time.elapsed().as_secs() + 1;
                let wait = if elapsed > self.cooldown {
                    0
                } else {
                    self.cooldown - elapsed
                };
                
                self.start_time = Instant::now();
                self.chunk_i = 0;
                
                Some((chunk, Some(Duration::from_secs(wait))))
            }
        } else {
            None
        }
    }
}

fn get_retry_seconds(response: &reqwest::Response) -> Option<u64> {
    if let Some(header) = response.headers().get(RETRY_AFTER) {
        if let Ok(retry_after) = header.to_str() {
            if let Ok(seconds) = retry_after.parse::<u64>() {
                return Some(seconds);
            }
        }
    }
    
    None
}

/// Sensible wait durations for retrying requests.
pub fn retryable_duration(response: &reqwest::Response) -> Option<Duration> {
    match response.status() {
        StatusCode::BAD_GATEWAY => return Some(Duration::from_secs(5)),
        StatusCode::TOO_MANY_REQUESTS => if let Some(seconds) = get_retry_seconds(response) {
            return Some(Duration::from_secs(seconds));
        },
        _ => {},
    }
    
    None
}

pub async fn parses_response<D>(response: reqwest::Response) -> Result<D, Error>
where
    D: DeserializeOwned
{
    #[derive(Deserialize, Debug)]
    struct ErrorResponse {
        message: String,
    }
    
    let status = response.status();
    
    match status.as_u16() {
        100..=199 |
        300..=599 => Err(Error::Http(response)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next() {
        let vec = (0..10000).into_iter().collect::<Vec<_>>();
        let mut cooldown = Cooldown::new(&vec);
        
        for _i in 0..9 {
            cooldown.next();
        }
        
        let (_, duration) = cooldown.next().unwrap();
        
        // on the 9th iteration there should be no duration
        assert!(duration.is_none());
        
        let (_, duration) = cooldown.next().unwrap();
        
        // on the 10th iteration there should be a cooldown
        assert!(duration.is_some());
        
        let (_, duration) = cooldown.next().unwrap();
        
        // it resets, there should now be no duration
        assert!(duration.is_none());
    }
}