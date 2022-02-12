use crate::enums::{
    ListingIntent,
    CurrencyType,
    ListingItemAttributeValue
};
use crate::query::{
    CreateListingQuery,
    CreateAlertQuery,
    GetNotificationsQuery,
    SellListingQuery,
    BuyListingQuery,
    BuyListingItemQuery,
    BuyListingItemAttributeQuery
};
use crate::structs::{
    Players,
    Player,
    Snapshots,
    Cursor,
    Alert,
    Alerts,
    Listing,
    Listings,
    Notification,
    Notifications,
    AgentPulse,
    MinMax,
    InventoryValues,
    InventoryStatus,
    ListingItemAttribute,
    ServerTime,
    Attributes
};
use crate::helpers::{get_default_middleware, parses_response};
use crate::serializers::{
    option_number_to_str,
    comma_delimited_steamids
};
use crate::deserializers::{
    bool_from_int
};
use crate::api_serializers::{
    currencies_into_form,
    option_buy_listing_query_into_params,
    listing_intent_enum_to_str
};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use chrono::serde::{ts_seconds_option};
use api_utils::error::{APIError, RESPONSE_UNSUCCESSFUL_MESSAGE};
use steamid_ng::SteamID;
use tf2_price::Currencies;
use reqwest_middleware::{ClientWithMiddleware};

pub struct BackpackAPI {
    key: String,
    token: String,
    client: ClientWithMiddleware,
}

impl BackpackAPI {
    
    pub fn new(key: String, token: String) -> Self {
        Self {
            key,
            token,
            client: get_default_middleware(),
        }
    }
    
    fn get_uri(&self, pathname: &str) -> String {
        // https://backpack.tf/api/index.html#/webapi-users/800c12bc395d28d411e61a7aa5da1d9c
        format!("https://backpack.tf{}", pathname)
    }

    fn get_api_uri(&self, endpoint: &str) -> String {
        format!("{}{}", self.get_uri("/api"), endpoint)
    }
    
    pub async fn get_user(&self, steamid: &SteamID) -> Result<Player, APIError> {
        let steamids: Vec<SteamID> = vec![*steamid];
        let players = self.get_users(&steamids).await?;
        
        if let Some(player) = players.get(steamid) {
            Ok(player.to_owned())
        } else {
            Err(APIError::ResponseError("No player with SteamID in response".into()))
        }
    }
    
    pub async fn get_users<'a, 'b>(&self, steamids: &'b [SteamID]) -> Result<Players, APIError> {
        #[derive(Serialize, Debug)]
        struct GetUsersParams<'a, 'b> {
            key: &'a str,
            #[serde(serialize_with = "comma_delimited_steamids")]
            steamids: &'b [SteamID],
        }
        
        if steamids.len() == 0 {
            return Err(APIError::ParameterError("No steamids given"));
        }
        
        let response = self.client.get(self.get_api_uri("/IGetUsers/v3"))
            .query(&GetUsersParams {
                key: &self.key,
                steamids,
            })
            .send()
            .await?;
        let body: GetUsersResponseWrapper = parses_response(response).await?;
        
        if !body.response.success {
            return Err(APIError::ResponseError(body.response.message.unwrap_or(RESPONSE_UNSUCCESSFUL_MESSAGE.to_string()).into()));
        } else if let Some(players) = body.response.players {
            Ok(players)
        } else {
            Err(APIError::ResponseError("No players in response".into()))
        }
    }
    
    pub async fn get_alerts<'a>(&self, skip: u32) -> Result<Alerts, APIError> {
        #[derive(Serialize, Debug)]
        struct GetAlertsParams<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
        }
        
        let uri = self.get_api_uri("/classifieds/alerts");
        let response = self.client.get(uri)
            .query(&GetAlertsParams {
                token: &self.token,
                limit: 500,
                skip,
            })
            .send()
            .await?;
        let body: Alerts = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn create_alert<'a>(&self, query: CreateAlertQuery) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct CreateAlertParams<'a> {
            token: &'a str,
            item_name: String,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: ListingIntent,
            currency: Option<CurrencyType>,
            min: Option<u32>,
            max: Option<u32>,
            blanket: Option<bool>,
        }

        let mut currency: Option<CurrencyType> = None;
        let mut min: Option<u32> = None;
        let mut max: Option<u32> = None;
        // defaults to blanket
        let mut blanket: Option<bool> = Some(true);

        if let Some(values) = query.values {
            currency = Some(values.currency);
            min = Some(values.min);
            max = Some(values.max);
            blanket = None;
        }

        self.client.post(self.get_api_uri("/classifieds/alerts"))
            .query(&CreateAlertParams {
                token: &self.token,
                item_name: query.item_name,
                intent: query.intent,
                currency,
                min,
                max,
                blanket,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn delete_alert_by_name<'a, 'b>(&self, item_name: &'b str, intent: &'b ListingIntent) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct DeleteAlertParams<'a, 'b> {
            token: &'a str,
            item_name: &'b str,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: &'b ListingIntent,
        }
        
        self.client.delete(self.get_api_uri("/classifieds/alerts"))
            .query(&DeleteAlertParams {
                token: &self.token,
                item_name,
                intent,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn delete_alert(&self, id: &str) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct DeleteAlertParams<'a> {
            token: &'a str,
        }

        self.client.delete(format!("{}/{}", self.get_api_uri("/classifieds/alerts"), id))
            .query(&DeleteAlertParams {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_alert<'a>(&self, id: &str) -> Result<Alert, APIError> {
        #[derive(Serialize, Debug)]
        struct GetAlertParams<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}", self.get_api_uri("/classifieds/alerts"), id);
        let response = self.client.get(uri)
            .query(&GetAlertParams {
                token: &self.token,
            })
            .send()
            .await?;
        let body: Alert = parses_response(response).await?;
            
        Ok(body)
    }

    pub async fn get_notification<'a>(&self, id: &str) -> Result<Notification, APIError> {
        #[derive(Serialize, Debug)]
        struct GetNotificationParams<'a> {
            token: &'a str,
        }
        
        let response = self.client.get(format!("{}/{}", self.get_api_uri("/notifications"), id))
            .query(&GetNotificationParams {
                token: &self.token,
            })
            .send()
            .await?;
        let body: Notification = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn delete_notification<'a>(&self, id: &str) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct DeleteNotificationParams<'a> {
            token: &'a str,
        }

        let uri = format!("{}/{}", self.get_api_uri("/notifications"), id);
        
        self.client.delete(uri)
            .query(&DeleteNotificationParams {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_notifications<'a>(&self, query: GetNotificationsQuery) -> Result<Notifications, APIError> {
        #[derive(Serialize, Debug)]
        struct GetNotificationsParams<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
            unread: bool,
        }
        
        let uri = self.get_api_uri("/notifications");
        let response = self.client.get(uri)
            .query(&GetNotificationsParams {
                token: &self.token,
                skip: query.skip,
                limit: query.limit,
                unread: query.unread,
            })
            .send()
            .await?;
        let body: Notifications = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_unread_notifications<'a>(&self) -> Result<Notifications, APIError> {
        #[derive(Serialize, Debug)]
        struct GetUnreadNotificationsParams<'a> {
            token: &'a str,
        }
        
        let uri = self.get_api_uri("/notifications/unread");
        let response = self.client.post(uri)
            .query(&GetUnreadNotificationsParams {
                token: &self.token,
            })
            .send()
            .await?;
        let body: Notifications = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn mark_unread_notifications<'a>(&'a self) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct MarkUnreadNotificationsParams<'a> {
            token: &'a str,
        }
        
        let uri = self.get_api_uri("/notifications/unread");
        let body = self.client.post(uri)
            .query(&MarkUnreadNotificationsParams {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_classifieds_snapshot<'a, 'b>(&self, sku: &'b str) -> Result<Snapshots, APIError> {
        #[derive(Serialize, Debug)]
        struct GetClassifiedsSnapshotParams<'a, 'b> {
            token: &'a str,
            sku: &'b str,
            appid: u32,
        }
        
        let uri = self.get_api_uri("/classifieds/listings/snapshot");
        let response = self.client.get(uri)
            .query(&GetClassifiedsSnapshotParams {
                token: &self.token,
                appid: 440,
                sku,
            })
            .send()
            .await?;
        let body: Snapshots = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_inventory_values<'a>(&self, steamid: &SteamID) -> Result<InventoryValues, APIError> {
        #[derive(Serialize, Debug)]
        struct GetInventoryValuesParams<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/values", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.get(uri)
            .query(&GetInventoryValuesParams {
                token: &self.token,
            })
            .send()
            .await?;
        let body: InventoryValues = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn get_inventory_status<'a>(&self, steamid: &SteamID) -> Result<InventoryStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct GetInventoryStatusParams<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/status", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.get(uri)
            .query(&GetInventoryStatusParams {
                token: &self.token,
            })
            .send()
            .await?;
        let body: InventoryStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn refresh_inventory<'a>(&self, steamid: &SteamID) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct RefreshInventoryParams<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/refresh", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let body = self.client.post(uri)
            .query(&RefreshInventoryParams {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_listings<'a>(&self, skip: u32, limit: u32) -> Result<Listings, APIError> {
        #[derive(Serialize, Debug)]
        struct GetListingsParams<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
        }
        
        let uri = self.get_api_uri("/v2/classifieds/listings");
        let response = self.client.get(uri)
            .query(&GetListingsParams {
                token: &self.token,
                skip: skip,
                limit: limit,
            })
            .send()
            .await?;
        let body: Listings = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn create_listings<'a>(&self, query: Vec<CreateListingQuery>) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct CreateListingsParams<'a> {
            token: &'a str,
            listings: Vec<CreateListingQuery>,
        }

        let params = CreateListingsParams {
            token: &self.token,
            listings: query,
        };

        println!("{}", serde_json::to_string(&params).unwrap());

        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.post(uri)
            .json(&params)
            .send()
            .await?
            .text()
            .await?;
        
        // println!("{}", response);
        Ok(())
    }

    pub async fn delete_listing<'a, 'b>(&self, &'b str) -> Result<(), APIError> {



        Ok(())
    }

    pub async fn create_listing<'a>(&self, query: CreateListingQuery) -> Result<Listing, APIError> {
        #[derive(Serialize, Debug)]
        struct CreateListingParams<'a> {
            token: &'a str,
            #[serde(serialize_with = "option_number_to_str", skip_serializing_if = "Option::is_none")]
            id: Option<u64>,
            #[serde(serialize_with = "option_buy_listing_query_into_params", skip_serializing_if = "Option::is_none")]
            item: Option<BuyListingItemQuery>,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: ListingIntent,
            #[serde(skip_serializing_if = "Option::is_none")]
            details: Option<String>,
            buyout: bool,
            offers: bool,
            #[serde(serialize_with = "currencies_into_form")]
            currencies: Currencies,
        }
        
        let params: CreateListingParams = match query {
            CreateListingQuery::Buy(listing) => {
                CreateListingParams {
                    token: &self.token,
                    id: None,
                    item: Some(listing.item),
                    intent: ListingIntent::Buy,
                    buyout: listing.buyout,
                    offers: listing.offers,
                    details: listing.details,
                    currencies: listing.currencies,
                }
            },
            CreateListingQuery::Sell(listing) => {
                CreateListingParams {
                    token: &self.token,
                    id: Some(listing.id),
                    item: None,
                    intent: ListingIntent::Sell,
                    buyout: listing.buyout,
                    offers: listing.offers,
                    details: listing.details,
                    currencies: listing.currencies,
                }
            },
        };
        let uri = self.get_api_uri("/v2/classifieds/listings");
        let response = self.client.post(uri)
            .json(&params)
            .send()
            .await?;
        let body: Listing = parses_response(response).await?;
        
        println!("{}", serde_json::to_string(&params).unwrap());
        
        Ok(body)
    }

    pub async fn agent_pulse<'a, 'b>(&self, user_agent: &'b str) -> Result<AgentPulse, APIError> {
        #[derive(Serialize, Debug)]
        struct AgentPulseParams<'a, 'b> {
            token: &'a str,
            user_agent: &'b str,
        }
        
        let response = self.client.post(self.get_api_uri("/agent/pulse"))
            .query(&AgentPulseParams {
                token: &self.token,
                user_agent,
            })
            .send()
            .await?;
        let body: AgentPulse = parses_response(response).await?;
        
        Ok(body)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CreateListingResponse {
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetUsersResponse {
    #[serde(deserialize_with = "bool_from_int")]
    success: bool,
    message: Option<String>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    current_time: Option<ServerTime>,
    players: Option<Players>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetUsersResponseWrapper {
    response: GetUsersResponse,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;
    use serde::de::{DeserializeOwned};

    fn read_file(filename: &str) -> std::io::Result<String> {
        let rootdir = env!("CARGO_MANIFEST_DIR");
        let filepath = Path::new(rootdir).join(format!("tests/json/{}", filename));
        
        fs::read_to_string(filepath)
    }
    
    fn read_and_parse_file<D>(filename: &str) -> Result<D, &str>
    where
        D: DeserializeOwned
    {
        let contents = tests::read_file(filename)
            .expect("Something went wrong reading the file");
        let response: D = serde_json::from_str(&contents).unwrap();
        
        Ok(response)
    }
    
    #[test]
    fn parses_get_classifieds_snapshot() {
        let response: Snapshots = tests::read_and_parse_file("get_classifieds_snapshot.json").unwrap();
        let listing = response.listings.iter().next().unwrap();
        
        assert_eq!(180, listing.currencies.keys);
    }
    
    #[test]
    fn parses_get_users() {
        let response: GetUsersResponseWrapper = tests::read_and_parse_file("get_users.json").unwrap();
        let players = response.response.players.unwrap();
        let steamid = SteamID::from(76561198080179568);
        let player = players.get(&steamid).unwrap();
        
        assert_eq!(52, player.backpack_tf_trust.r#for);
    }
    
    #[test]
    fn parses_get_alerts() {
        let response: Alerts = tests::read_and_parse_file("get_alerts.json").unwrap();
        let alert = response.alerts.first().unwrap();

        assert_eq!("Purple Energy Danger".to_string(), alert.item_name);    
    }

    #[test]
    fn parses_get_notifications() {
        let response: Notifications = tests::read_and_parse_file("get_notifications.json").unwrap();
        let particle = response.notifications.first().as_ref().unwrap().bundle.listing.as_ref().unwrap().item.particle.as_ref().unwrap();

        assert_eq!("Purple Energy".to_string(), particle.name); 
    }

    #[test]
    fn parses_get_listings() {
        let response: Listings = tests::read_and_parse_file("get_listings.json").unwrap();
        let listing = response.listings.first().unwrap();
        
        assert_eq!("Lucky Cat Hat".to_string(), listing.item.name); 
    }
}