//! Currency types.

use strum_macros::EnumString;
use serde::{Serialize, Deserialize};

/// The type of currencies. This is mostly used for alerts.
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Copy, Debug, EnumString)]
pub enum CurrencyType {
    /// Keys.
    #[strum(serialize = "keys")]
    Keys,
    /// Metal.
    #[strum(serialize = "metal")]
    Metal,
}