//! Listing value.

use serde::{Serialize, Deserialize};

/// Value.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    /// The raw value.
    pub raw: f32,
    /// The short value.
    pub short: String,
    /// The long value.
    pub long: String,
}