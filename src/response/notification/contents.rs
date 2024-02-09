//! Notificaiton contents.

use serde::{Serialize, Deserialize};

/// The contents of a notification.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    /// The subject of the notification.
    pub subject: String,
    /// The message of the notification.
    pub message: String,
    /// The URL of the notification.
    pub url: String,
}