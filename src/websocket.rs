use crate::response::listing::Listing;
use tokio_tungstenite::{tungstenite, connect_async};
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::value::RawValue;
use tokio::sync::mpsc;

#[derive(Deserialize, Debug)]
enum EventType {
    #[serde(rename = "listing-update")]
    ListingUpdate,
    #[serde(rename = "listing-delete")]
    ListingDelete,
}

#[derive(Deserialize, Debug)]
struct EventMessage<'a> {
    event: EventType,
    #[serde(borrow)]
    payload: &'a RawValue,
}

#[derive(Debug)]
pub enum Message {
    ListingUpdate(Listing),
    ListingDelete(Listing),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}", .0)]
    Url(#[from] url::ParseError),
    #[error("{}", .0)]
    Connect(#[from] tungstenite::Error),
}

pub async fn connect() -> Result<mpsc::Receiver<Message>, Error> {
    let connect_addr = "wss://ws.backpack.tf/events";
    let url = url::Url::parse(connect_addr)?;
    let (ws_stream, _) = connect_async(url).await?;
    let (write, read) = mpsc::channel::<Message>(100);
    let (_ws_write, mut ws_read) = ws_stream.split();
    
    tokio::spawn(async move {
        while let Some(message) = ws_read.next().await {
            match message {
                Ok(message) => {
                    let data = message.into_data();
                    let bytes = data.as_slice();
                    
                    match serde_json::from_slice::<EventMessage>(bytes) {
                        Ok(message) => {
                            if let Err(error) = on_event(&message, &write).await {
                                match error {
                                    EventError::Serde(error) => {
                                        log::debug!(
                                            "Error deserializing event payload: {} {}",
                                            error,
                                            message.payload,
                                        );
                                    },
                                    // connection likely dropped
                                    EventError::Send(_) => {
                                        break;
                                    },
                                }
                            }
                        },
                        Err(error) => {
                            if bytes.is_empty() {
                                // the message is empty...
                                continue;
                            } else if let Ok(message) = std::str::from_utf8(bytes) {
                                log::debug!(
                                    "Error deserializing event: {} {}",
                                    error,
                                    message,
                                );
                            } else {
                                log::debug!(
                                    "Error deserializing event: {}; Invalid utf8 string: {:?}",
                                    error,
                                    bytes,
                                );
                            }
                        },
                    }
                },
                Err(error) => {
                    // dropped?
                    log::debug!(
                        "Connection dropped: {:?}",
                        error,
                    );
                    break;
                },
            }
        }
        
        drop(write);
    });
    
    Ok(read)
}

#[derive(thiserror::Error, Debug)]
enum EventError {
    #[error("{}", .0)]
    Send(#[from] tokio::sync::mpsc::error::SendError<Message>),
    #[error("{}", .0)]
    Serde(#[from] serde_json::Error),
}

async fn on_event<'a>(
    message: &EventMessage<'a>,
    write: &mpsc::Sender<Message>,
) -> Result<(), EventError> {
    match message.event {
        EventType::ListingUpdate | EventType::ListingDelete => {
            let listing = serde_json::from_str::<Listing>(message.payload.get())?;
            let event = match message.event {
                EventType::ListingUpdate => Message::ListingUpdate(listing),
                EventType::ListingDelete => Message::ListingDelete(listing),
            };
            
            write.send(event).await?;
            
            Ok(())
        },
    }
}