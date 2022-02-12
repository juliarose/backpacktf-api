use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Values {
    pub market_value: f32,
    pub value: f32,
}
