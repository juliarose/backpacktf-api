use super::BuyListingItemAttribute;

pub trait Attributable {
    fn as_attributes(&self) -> Vec<BuyListingItemAttribute>;
}
