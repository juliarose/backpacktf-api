/// A cursor for scrolling responses.

use serde::{Serialize, Deserialize};

/// Cursor for scrolling response/
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Cursor {
    /// How many items to skip.
    pub skip: u32,
    /// Limit of items to return.
    pub limit: u32,
    /// Total number of items.
    pub total: u32,
}