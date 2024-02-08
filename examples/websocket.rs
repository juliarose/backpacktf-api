use backpacktf_api::websocket::{Error, connect};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut websocket = connect().await?;

    while let Some((id, message)) = websocket.recv().await {
        println!("{id}: {message:?}");
    }

    Ok(())
}
