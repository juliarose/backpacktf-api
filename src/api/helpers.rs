use serde::{Deserialize, de::DeserializeOwned};
use crate::error::Error;

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
        300..=399 => {
            Err(Error::Http(*status))
        },
        400..=499 => {
            Err(Error::Http(*status))
        },
        500..=599 => {
            Err(Error::Http(*status))
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
                        println!("{}", String::from_utf8_lossy(body));
                        
                        Err(parse_error.into())
                    }
                }
            }
        }
    }
}