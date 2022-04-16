use serde::{Deserialize, Serialize};
use crate::{request, response};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ErrorListing<T> {
    pub message: String,
    pub query: request::CreateListing<T>,
}

pub type Result<T> = std::result::Result<response::listing::Listing, ErrorListing<T>>;