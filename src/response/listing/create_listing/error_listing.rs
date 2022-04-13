use serde::{Deserialize, Serialize};
use crate::request;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ErrorListing<T> {
    pub message: String,
    pub query: request::CreateListing<T>,
}