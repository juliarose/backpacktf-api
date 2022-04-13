use serde::{Deserialize, Serialize};
use crate::response;
use super::ErrorListing;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CreateListingsResult<T> {
    pub success: Vec<response::listing::Listing>,
    pub error: Vec<ErrorListing<T>>,
}