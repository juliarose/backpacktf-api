use serde::{Deserialize, Serialize};
use crate::response;
use super::ErrorListing;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CreateListingsResult {
    pub success: Vec<response::listing::Listing>,
    pub error: Vec<ErrorListing>,
}