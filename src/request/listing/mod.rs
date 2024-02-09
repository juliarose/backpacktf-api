//! Contains structs for listing requests.

#[allow(clippy::module_inception)]
pub mod create_listing;
pub mod update_listing;

pub use create_listing::CreateListing;
pub use update_listing::UpdateListing;