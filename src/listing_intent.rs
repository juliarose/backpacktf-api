use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ListingIntent {
    Buy = 0,
    Sell = 1,
}