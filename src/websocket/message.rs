//! A parsed message from the websocket.

use crate::response::listing::Listing;
use std::fmt;
use serde_json::value::RawValue;

/// A message from the websocket.
#[derive(Debug)]
pub enum Message {
    /// A listing was updated.
    ListingUpdate(Listing),
    /// A listing was deleted.
    ListingDelete(Listing),
    /// A listing from another app other than Team Fortress 2 was updated. Because of the way
    /// listings are deserialized, other apps are not supported by this crate. You can still
    /// deserialze them from the payload if needed.
    ListingUpdateOtherApp {
        /// The appid of the listing.
        appid: u32,
        /// The payload of the event.
        payload: Box<RawValue>,
    },
    /// A listing from another app other than Team Fortress 2 was deleted. Because of the way
    /// listings are deserialized, other apps are not supported by this crate. You can still
    /// deserialze them from the payload if needed.
    ListingDeleteOtherApp {
        /// The appid of the listing.
        appid: u32,
        /// The payload of the event.
        payload: Box<RawValue>,
    },
    /// The client was exceeded. The contained string contains more details.
    ClientLimitExceeded(String),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::ListingUpdate(listing) => write!(f, "ListingUpdate: {}", listing),
            Message::ListingDelete(listing) => write!(f, "ListingDelete: {}", listing),
            Message::ListingUpdateOtherApp { appid, payload } => {
                write!(f, "ListingUpdateOtherApp: appid={}, payload={}", appid, payload)
            }
            Message::ListingDeleteOtherApp { appid, payload } => {
                write!(f, "ListingDeleteOtherApp: appid={}, payload={}", appid, payload)
            }
            Message::ClientLimitExceeded(message) => write!(f, "ClientLimitExceeded: {}", message),
        }
    }
}