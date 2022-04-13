use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Eq, PartialEq, Clone, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ListingIntent {
    Buy = 0,
    Sell = 1,
}