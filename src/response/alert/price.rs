//! Price for an alert.

use crate::currency_type::CurrencyType;
use crate::response::deserializers;
use serde::{Deserialize, Serialize};

/// Represents a price for an alert.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Price {
    /// The currency type.
    #[serde(deserialize_with = "deserializers::currency_type_enum_from_str")]
    pub currency: CurrencyType,
    /// The minimum value.
    pub min: Option<f32>,
    /// The maximum value.
    pub max: Option<f32>,
}