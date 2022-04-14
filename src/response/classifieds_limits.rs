use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ClassifiedsLimits {
    pub promotion_slots_available: u32,
    pub used: u32,
    pub total: u32,
    pub baseline: u32,
    pub donation_bonus: u32,
    pub gifted_premium_months_bonus: u32,
    pub multiplier: u32,
    pub twitter_follower_bonus: u32,
    pub accepted_suggestion_bonus: u32,
    pub mvp_donation_bonus: u32,
    pub group_membership_bonus: u32,
}