use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UpdateListing<T> {
    pub id: String,
    pub currencies: T,
    pub details: Option<String>,
}