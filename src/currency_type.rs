use strum_macros::EnumString;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, EnumString)]
pub enum CurrencyType {
    #[strum(serialize = "keys")]
    Keys,
    #[strum(serialize = "metal")]
    Metal,
}