use backpacktf_api::websocket::{Error, connect};

#[tokio::main]
async fn main() {
    match connect().await {
        Ok(mut websocket) => {
             while let Some((_, message)) = websocket.recv().await {
                println!("{message:?}");
            }
        },
        // Server responded with an HTTP error
        Err(Error::Http(response)) => {
            let status = response.status();
            
            if let Some(text) = response.into_body() {
                println!("HTTP Error: {}", String::from_utf8_lossy(&text));
            } else {
                println!("HTTP Error: {status}");
            }
        },
        Err(error) => {
            println!("Error connecting to websocket: {:?}", error);
        },
    }
}
