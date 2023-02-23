use crate::currency_type::CurrencyType;
use crate::response::deserializers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Price {
    #[serde(deserialize_with = "deserializers::currency_type_enum_from_str")]
    pub currency: CurrencyType,
    pub min: Option<f32>,
    pub max: Option<f32>,
}