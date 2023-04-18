use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Debug, Deserialize_enum_str, Serialize_enum_str, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    None,
    Active,
    ListingInactive,
    HiddenByUser,
    NotEnoughCurrency,
    #[serde(other)]
    Other(String),
}

impl Default for Status {
    fn default() -> Self {
        Self::None
    }
}

