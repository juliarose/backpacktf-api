use serde::{Deserialize, Serialize};

/// Parameters for updating a listing.
/// 
/// Currencies can be any type that can be serialized and deserialized.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UpdateListing<T> {
    /// The listing ID.
    pub id: String,
    /// The currencies. In practice, any type that can be serialized can be supplied.
    pub currencies: T,
    /// The message of the listing.
    pub details: Option<String>,
}