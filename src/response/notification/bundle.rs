//! Bundle.

use crate::response::listing::Listing;
use serde::{Serialize, Deserialize};

/// A bundle.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// The bundle of listings.
    pub listing: Option<Listing>,
}