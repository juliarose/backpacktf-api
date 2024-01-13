use serde::{Deserialize, Serialize};

/// Inventory values.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Values {
    /// The market value.
    pub market_value: f32,
    /// The backpack.tf value.
    pub value: f32,
}
