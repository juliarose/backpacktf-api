use serde::{Deserialize, de::DeserializeOwned};
use super::APIError;

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
                        Err(APIError::Response(error_body.message))
                    } else {
                        println!("{}", String::from_utf8_lossy(body));
                        
                        Err(parse_error.into())
                    }
                }
            }
        }
    }
}