use serde::{Deserialize, Serialize};
use crate::currency_type::CurrencyType;
use crate::response::deserializers::currency_type_enum_from_str;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MinMax {
    #[serde(deserialize_with = "currency_type_enum_from_str")]
    pub currency: CurrencyType,
    pub min: f32,
    pub max: f32,
}