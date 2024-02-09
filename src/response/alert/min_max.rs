//! Sets minimum and maximum values.

use crate::currency_type::CurrencyType;
use crate::response::deserializers;
use serde::{Deserialize, Serialize};

/// Represents a minimum and maximum value.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MinMax {
    /// The currency type.
    #[serde(deserialize_with = "deserializers::currency_type_enum_from_str")]
    pub currency: CurrencyType,
    /// The minimum value.
    pub min: f32,
    /// The maximum value.
    pub max: f32,
}