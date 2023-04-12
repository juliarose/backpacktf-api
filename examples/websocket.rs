use backpacktf_api::websocket::{Error, connect};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut websocket = connect().await?;
    
    while let Some(message) = websocket.recv().await {
        println!("{message:?}");
    }
    
    Ok(())
}