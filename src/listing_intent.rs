use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum ListingIntent {
    Buy = 0,
    Sell = 1,
}