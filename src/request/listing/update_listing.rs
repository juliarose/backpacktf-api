use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Body<T> {
    pub currencies: T,
    pub details: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UpdateListing<T> {
    pub id: String,
    pub body: Body<T>,
}