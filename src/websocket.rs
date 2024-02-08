//! Module for connecting and reading from the backpack.tf websocket.

use crate::response::listing::Listing;
use tokio_tungstenite::{tungstenite, connect_async};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use tokio::sync::mpsc;
use http::uri::Uri;
use http::request::Request;

const APPID_TEAM_FORTRESS_2: u32 = 440;

/// The type of event from the websocket.
#[derive(Deserialize, Debug)]
enum EventType {
    /// A listing was updated.
    #[serde(rename = "listing-update")]
    ListingUpdate,
    /// A listing was deleted.
    #[serde(rename = "listing-delete")]
    ListingDelete,
}

/// An event from the websocket.
#[derive(Deserialize, Debug)]
struct EventMessage<'a> {
    /// The event ID.
    id: &'a str,
    /// The type of event.
    event: EventType,
    /// The payload of the event.
    #[serde(borrow)]
    payload: &'a RawValue,
}

/// A message from the websocket.
#[derive(Debug)]
pub enum Message {
    /// A listing was updated.
    ListingUpdate(Listing),
    /// A listing was deleted.
    ListingDelete(Listing),
    /// A listing from another app other than Team Fortress 2 was updated.
    ListingUpdateOtherApp {
        /// The appid of the listing.
        appid: u32,
        /// The payload of the event.
        payload: Box<RawValue>,
    },
    /// A listing from another app other than Team Fortress 2 was deleted.
    ListingDeleteOtherApp {
        /// The appid of the listing.
        appid: u32,
        /// The payload of the event.
        payload: Box<RawValue>,
    },
}

/// An error from the websocket.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error was encountered parsing a URL.
    #[error("{}", .0)]
    Url(#[from] http::uri::InvalidUri),
    /// The parsed URL does not contain a hostname.
    #[error("Parsed URL does not contain hostname")]
    UrlNoHostName,
    /// An error was encountered parsing a request.
    #[error("Error parsing request parameters: {}", .0)]
    RequestParse(#[from] http::Error),
    /// An error was encountered connecting to the websocket.
    #[error("{}", .0)]
    Connect(#[from] tungstenite::Error),
}

#[derive(Debug, Deserialize, Serialize)]
struct AppType {
    appid: u32,
}

/// Generate a random key for the `Sec-WebSocket-Key` header.
pub fn generate_key() -> String {
    // a base64-encoded (see Section 4 of [RFC4648]) value that,
    // when decoded, is 16 bytes in length (RFC 6455)
    let r: [u8; 16] = rand::random();
    data_encoding::BASE64.encode(&r)
}

/// Connect to the websocket.
pub async fn connect() -> Result<mpsc::Receiver<(String, Message)>, Error> {
    // Build our request for connecting to the websocket
    let request = {
        // The address to connect to.
        let connect_addr = "wss://ws.backpack.tf/events";
        let uri = connect_addr.parse::<Uri>()?;
        let authority = uri.authority()
            .ok_or(Error::UrlNoHostName)?.as_str();
        let host = authority
            .find('@')
            .map(|idx| authority.split_at(idx + 1).1)
            .unwrap_or_else(|| authority);
        // Add the headers to the request
        let request = Request::builder()
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            // Required header to connect to the websocket server
            // https://www.rfc-editor.org/rfc/rfc6455#page-57
            .header("Sec-WebSocket-Key", generate_key())
            .uri(uri)
            .body(())?;

        request
    };
    let (ws_stream, _) = connect_async(request).await?;
    let (write, read) = mpsc::channel::<(String, Message)>(100);
    let (_ws_write, mut ws_read) = ws_stream.split();

    tokio::spawn(async move {
        while let Some(message) = ws_read.next().await {
            match message {
                Ok(message) => {
                    let data = message.into_data();
                    let bytes = data.as_slice();

                    match serde_json::from_slice::<Vec<EventMessage>>(bytes) {
                        Ok(messages) => {
                            for message in messages {
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
                            }
                        },
                        Err(error) => if bytes.is_empty() {
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

/// An error from an event.
#[derive(thiserror::Error, Debug)]
enum EventError {
    #[error("{}", .0)]
    Send(#[from] tokio::sync::mpsc::error::SendError<(String, Message)>),
    #[error("{}", .0)]
    Serde(#[from] serde_json::Error),
}

/// Handle an event.
async fn on_event<'a>(
    message: &EventMessage<'a>,
    write: &mpsc::Sender<(String, Message)>,
) -> Result<(), EventError> {
    match message.event {
        EventType::ListingUpdate |
        EventType::ListingDelete => {
            match serde_json::from_str::<Listing>(message.payload.get()) {
                Ok(listing) => {
                    write.send((message.id.to_owned(), match message.event {
                        EventType::ListingUpdate => Message::ListingUpdate(listing),
                        EventType::ListingDelete => Message::ListingDelete(listing),
                    })).await?;
                },
                Err(error) => if let Ok(AppType { appid }) = serde_json::from_str::<AppType>(message.payload.get()) {
                    if appid != APPID_TEAM_FORTRESS_2 {
                        let payload = message.payload.to_owned();

                        write.send((message.id.to_owned(), match message.event {
                            EventType::ListingUpdate => Message::ListingUpdateOtherApp {
                                appid,
                                payload,
                            },
                            EventType::ListingDelete => Message::ListingDeleteOtherApp {
                                appid,
                                payload,
                            },
                        })).await?;
                    } else {
                        return Err(error.into());
                    }
                } else {
                    return Err(error.into());
                },
            }

            Ok(())
        },
    }
}
