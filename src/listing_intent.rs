use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// The intent of a listing.
#[derive(Serialize_repr, Deserialize_repr, Hash, Eq, PartialEq, Clone, Copy, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum ListingIntent {
    Buy = 0,
    Sell = 1,
}