//! Module for connecting and reading from the backpack.tf websocket. Uses tokio asynchronous
//! framework.

mod message;
mod handlers;

pub use message::Message;
pub use tungstenite::Error;

use handlers::read_events;

use tokio::sync::mpsc;
use tokio_tungstenite::{tungstenite, connect_async};
use tungstenite::client::IntoClientRequest;

/// The websocket message receiver.
pub type Receiver = mpsc::Receiver<(String, Message)>;

const CONNECT_ADDR: &str = "wss://ws.backpack.tf/events";

/// Connects to the websocket.
/// 
/// Dropping the receiver closes the connection.
pub async fn connect() -> Result<mpsc::Receiver<(String, Message)>, tungstenite::Error> {
    // The address to connect to.
    let request = CONNECT_ADDR
        .into_client_request()?;
    let (stream, _) = connect_async(request.clone()).await?;
    let (sender, read) = mpsc::channel::<(String, Message)>(100);
    
    tokio::spawn(read_events(stream, sender));
    
    Ok(read)
}
