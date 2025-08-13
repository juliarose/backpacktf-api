//! Handlers for reading messages.

use std::string;

use super::Message;
use crate::response::listing::Listing;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::value::RawValue;

const APPID_TEAM_FORTRESS_2: u32 = 440;

/// An error from an event.
#[derive(thiserror::Error, Debug)]
enum EventError<'a> {
    /// An error was encountered sending a message.
    #[error("{}", .0)]
    Send(#[from] SendError<(String, Message)>),
    /// An error was encountered deserializing a message.
    #[error("{}", .0)]
    Serde(serde_json::Error, &'a RawValue),
    /// This should be unreachable.
    #[error("Unreachable code reached")]
    Unreachable,
}

/// The type of event from the websocket.
#[derive(Deserialize, Debug)]
enum EventType {
    /// A listing was updated.
    #[serde(rename = "listing-update")]
    ListingUpdate,
    /// A listing was deleted.
    #[serde(rename = "listing-delete")]
    ListingDelete,
    /// Client limit was exceeded.
    #[serde(rename = "client-limit-exceeded")]
    ClientLimitExceeded,
}

/// An event from the websocket.
#[derive(Deserialize, Debug)]
struct EventMessage<'a> {
    /// The event ID.
    id: String,
    /// The type of event.
    event: EventType,
    /// The payload of the event.
    #[serde(borrow)]
    payload: &'a RawValue,
}

/// A listing from another app other than Team Fortress 2. This reads only the appid from the 
/// payload.
#[derive(Debug, Deserialize)]
struct AppType {
    appid: u32,
}

/// A payload which includes a message was received.
#[derive(Debug, Deserialize)]
struct StringMessage {
    message: String,
}

/// Intermediary for converting raw messages into [`Message`].
struct Event {
    id: String,
    message: Message,
}

impl<'a> TryFrom<EventMessage<'a>> for Event {
    type Error = EventError<'a>;
    
    fn try_from(message: EventMessage<'a>) -> Result<Self, Self::Error> {
        use EventType::*;
        
        match message.event {
            ListingUpdate |
            ListingDelete => {
                match serde_json::from_str::<Listing>(message.payload.get()) {
                    Ok(listing) => {
                        match message.event {
                            EventType::ListingUpdate => Ok(Event {
                                id: message.id,
                                message: Message::ListingUpdate(listing)
                            }),
                            EventType::ListingDelete => Ok(Event {
                                id: message.id,
                                message: Message::ListingDelete(listing)
                            }),
                            _ => Err(EventError::Unreachable),
                        }
                    },
                    Err(error) => {
                        if let Ok(AppType { appid }) = serde_json::from_str::<AppType>(message.payload.get()) {
                            if appid != APPID_TEAM_FORTRESS_2 {
                                // move the payload into a box
                                let payload = message.payload.to_owned();
                                
                                return match message.event {
                                    EventType::ListingUpdate => Ok(Event {
                                        id: message.id,
                                        message: Message::ListingUpdateOtherApp {
                                            appid,
                                            payload,
                                        },
                                    }),
                                    EventType::ListingDelete => Ok(Event {
                                        id: message.id,
                                        message: Message::ListingDeleteOtherApp {
                                            appid,
                                            payload,
                                        },
                                    }),
                                    // Shouldn't be reachable
                                    _ => Err(EventError::Unreachable),
                                };
                            }
                        }
                        
                        return Err(EventError::Serde(error, message.payload));
                    },
                }
            },
            EventType::ClientLimitExceeded => {
                // move the payload into a box
                if let Ok(string_message) = serde_json::from_str::<StringMessage>(message.payload.get()) {
                    return Ok(Event {
                        id: message.id,
                        message: Message::ClientLimitExceeded(string_message.message),
                    });
                }
                
                Ok(Event {
                    id: message.id,
                    message: Message::ClientLimitExceeded(message.payload.get().to_owned()),
                })
            },
        }
    }
}

pub async fn read_events(
    mut stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    sender: mpsc::Sender<(String, Message)>,
) {
    while let Some(message) = stream.next().await {
        match message {
            Ok(WsMessage::Text(bytes)) => {
                if bytes.is_empty() {
                    // the message is empty... just ignore it
                    continue;
                }
                
                match serde_json::from_slice::<Vec<EventMessage>>(bytes.as_ref()) {
                    Ok(messages) => {
                        for message in messages {
                            // parse and send the message to the sender, capturing any errors in the message
                            match on_event(message, &sender).await {
                                Ok(_) => {},
                                Err(EventError::Serde(error, payload)) => {
                                    log::debug!("Error deserializing event payload: {error}\n\n{payload}");
                                },
                                // This means the channel is closed, so we can stop reading messages.
                                Err(EventError::Send(_)) => break,
                                Err(EventError::Unreachable) => continue,
                            }
                        }
                    },
                    Err(error) => {
                        // If we encounter an error deserializing the event, log it.
                        if let Ok(message) = std::str::from_utf8(bytes.as_ref()) {
                            log::debug!("Error deserializing event: {error} {message}");
                        } else {
                            log::debug!("Error deserializing event: {error}; Invalid utf8 string: {bytes:?}");
                        }
                    },
                }
            },
            // We don't expect binary messages, but if we do, log it.
            Ok(WsMessage::Binary(_)) => log::debug!("Received unexpected binary message"),
            // If we receive a close frame, we can stop reading messages.
            Ok(WsMessage::Close(frame)) => {
                log::debug!("Connection closed: {:?}", frame);
                break;
            },
            Ok(WsMessage::Frame(frame)) => log::debug!("Frame received: {}", frame),
            // Ping/pongs are handled automatically by the library, so we can ignore them.
            Ok(WsMessage::Ping(_)) |
            Ok(WsMessage::Pong(_)) => {},
            Err(error) => {
                // dropped?
                log::debug!("Connection dropped: {}", error);
                break;
            },
        }
    }
}

/// Handles an event.
async fn on_event<'a>(
    message: EventMessage<'a>,
    sender: &mpsc::Sender<(String, Message)>,
) -> Result<(), EventError<'a>> {
    let Event { id, message } = Event::try_from(message)?;
    
    sender.send((id, message)).await?;
    Ok(())
}
