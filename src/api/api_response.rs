use serde::{Serialize, Deserialize};
use crate::{
    time::ServerTime,
    response,
    response::deserializers::bool_from_int,
};
use chrono::serde::ts_seconds_option;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersResponse {
    #[serde(deserialize_with = "bool_from_int")]
    pub success: bool,
    pub message: Option<String>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub current_time: Option<ServerTime>,
    pub players: Option<response::player::Players>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersResponseWrapper {
    pub response: GetUsersResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetNotificationsResponse {
    #[serde(rename(deserialize = "results"))]
    pub notifications: Vec<response::notification::Notification>,
    pub cursor: response::cursor::Cursor,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetListingsResponse {
    #[serde(rename(deserialize = "results"))]
    pub listings: Vec<response::listing::Listing>,
    pub cursor: response::cursor::Cursor,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAlertsResponse {
    #[serde(rename(deserialize = "results"))]
    pub alerts: Vec<response::alert::Alert>,
    pub cursor: response::cursor::Cursor,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::response;
    use steamid_ng::SteamID;
    use tf2_enum::Paint;

    #[test]
    fn parses_delete_listings() {
        let response: response::listing::delete_listing::DeleteListingsResult = serde_json::from_str(include_str!("fixtures/delete_listings.json")).unwrap();
        let deleted = response.deleted;
        
        assert_eq!(deleted, 5);
    }
    
    #[test]
    fn parses_get_users() {
        let response: GetUsersResponseWrapper = serde_json::from_str(include_str!("fixtures/get_users.json")).unwrap();
        let players = response.response.players.unwrap();
        let steamid = SteamID::from(76561198080179568);
        let player = players.get(&steamid).unwrap();
        
        assert_eq!(52, player.backpack_tf_trust.r#for);
    }
    
    #[test]
    fn parses_get_alerts() {
        let response: GetAlertsResponse = serde_json::from_str(include_str!("fixtures/get_alerts.json")).unwrap();
        let alert = response.alerts.first().unwrap();

        assert_eq!("Purple Energy Danger".to_string(), alert.item_name);    
    }

    #[test]
    fn parses_get_notifications() {
        let response: GetNotificationsResponse = serde_json::from_str(include_str!("fixtures/get_notifications.json")).unwrap();
        let particle = response.notifications.first().as_ref().unwrap().bundle.listing.as_ref().unwrap().item.particle.as_ref().unwrap();

        assert_eq!("Purple Energy".to_string(), particle.name); 
    }

    #[test]
    fn parses_get_listings() {
        let response: GetListingsResponse = serde_json::from_str(include_str!("fixtures/get_listings.json")).unwrap();
        let listing = response.listings.iter().find(|listing| listing.id == "440_7764221391").unwrap();
        
        assert_eq!("Lucky Cat Hat", &listing.item.name); 
        assert_eq!(Paint::PinkAsHell, *listing.item.paint.as_ref().unwrap()); 
    }
}