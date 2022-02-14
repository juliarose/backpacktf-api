use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Values {
    pub market_value: f32,
    pub value: f32,
}
