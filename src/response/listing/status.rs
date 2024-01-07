use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

/// The status of a listing.
#[derive(Debug, Deserialize_enum_str, Serialize_enum_str, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    /// No specific status.
    #[default]
    None,
    /// The listing is active.
    Active,
    /// The listing is inactive.
    ListingInactive,
    /// The listing is hidden by the user.
    HiddenByUser,
    /// The listing is hidden because the user does not have enough currency.
    NotEnoughCurrency,
    /// Another reason (check the string for more information).
    #[serde(other)]
    Other(String),
}