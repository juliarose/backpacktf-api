use serde::{Deserialize, Serialize};
pub use crate::response::alert::MinMax;
pub use crate::ListingIntent;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct CreateAlert {
    pub item_name: String,
    pub intent: ListingIntent,
    pub values: Option<MinMax>,
}