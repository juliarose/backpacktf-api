//! Create listing.

use crate::{request, response};
use serde::{Deserialize, Serialize};

/// An error occurred when creating a listing.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ErrorListing<T> {
    /// The message in the response.
    pub message: String,
    /// The query used to create the listing.
    pub query: request::CreateListing<T>,
}

/// The result of creating a listing.
pub type Result<T> = std::result::Result<response::listing::Listing, ErrorListing<T>>;