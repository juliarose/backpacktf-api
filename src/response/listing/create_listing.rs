//! Create listing.

use crate::{request, response};

/// An error occurred when creating a listing.
#[derive(PartialEq, Clone, Debug)]
pub struct ErrorListing<'a, T> {
    /// The message in the response.
    pub message: String,
    /// The query used to create the listing.
    pub query: &'a request::CreateListing<T>,
}

/// The result of creating a listing.
pub type Result<'a, T> = std::result::Result<response::listing::Listing, ErrorListing<'a, T>>;