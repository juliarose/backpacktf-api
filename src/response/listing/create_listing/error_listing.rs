use serde::{Deserialize, Serialize};
use crate::request;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ErrorListing {
    pub message: String,
    pub query: request::listing::create_listing::CreateListing,
}