//! Classifieds limits.

use serde::{Serialize, Deserialize};

/// The limits of the classifieds.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassifiedsLimits {
    /// The number of promotion slots available.
    pub promotion_slots_available: u32,
    /// The number of slots used.
    pub used: u32,
    /// The total number of slots that can be used.
    pub total: u32,
    /// The baseline number of slots.
    pub baseline: u32,
    /// The number of slots given from donations.
    pub donation_bonus: u32,
    /// The number of slots given from premium gifts.
    pub gifted_premium_months_bonus: u32,
    /// The listings multiplier.
    pub multiplier: u32,
    /// The number of slots given from following backpack.tf on Twitter.
    pub twitter_follower_bonus: u32,
    /// The number of slots given from accepted price suggestions.
    pub accepted_suggestion_bonus: u32,
    /// The number of slots given from donation status.
    pub mvp_donation_bonus: u32,
    /// The number of slots given from being a member of the backpack.tf group on Steam.
    pub group_membership_bonus: u32,
}
