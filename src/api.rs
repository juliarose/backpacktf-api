use serde::{Serialize, Deserialize};
use crate::{
    ListingIntent,
    currency_type::CurrencyType,
    time::ServerTime,
    request::listing::option_buy_listing_item_into_params,
    response::deserializers::bool_from_int,
    request,
    response
};
use crate::request::serializers::{
    option_number_to_str,
    comma_delimited_steamids,
    currencies_into_form,
    listing_intent_enum_to_str
};
use chrono::serde::ts_seconds_option;
use api_utils::{
    get_default_middleware,
    parses_response,
    error::APIError,
    error::RESPONSE_UNSUCCESSFUL_MESSAGE
};
use steamid_ng::SteamID;
use tf2_price::Currencies;
use reqwest_middleware::ClientWithMiddleware;

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
    
    pub async fn get_user(&self, steamid: &SteamID) -> Result<response::player::Player, APIError> {
        let steamids: Vec<SteamID> = vec![*steamid];
        let players = self.get_users(&steamids).await?;
        
        if let Some(player) = players.get(steamid) {
            Ok(player.to_owned())
        } else {
            Err(APIError::ResponseError("No player with SteamID in response".into()))
        }
    }
    
    pub async fn get_users<'a, 'b>(&self, steamids: &'b [SteamID]) -> Result<response::player::Players, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            key: &'a str,
            #[serde(serialize_with = "comma_delimited_steamids")]
            steamids: &'b [SteamID],
        }
        
        if steamids.len() == 0 {
            return Err(APIError::ParameterError("No steamids given"));
        }
        
        let response = self.client.get(self.get_api_uri("/IGetUsers/v3"))
            .query(&Params {
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
    
    pub async fn get_alerts<'a>(&self, skip: u32) -> Result<(Vec<response::alert::Alert>, response::cursor::Cursor), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
        }
        
        let uri = self.get_api_uri("/classifieds/alerts");
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
                limit: 500,
                skip,
            })
            .send()
            .await?;
        let body: GetAlertsResponse = parses_response(response).await?;
        
        Ok((body.alerts, body.cursor))
    }

    pub async fn create_alert<'a>(&self, query: &request::alert::CreateAlert) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            item_name: &'b String,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: &'b ListingIntent,
            currency: &'b Option<CurrencyType>,
            min: Option<u32>,
            max: Option<u32>,
            blanket: Option<bool>,
        }

        let mut currency: Option<CurrencyType> = None;
        let mut min: Option<u32> = None;
        let mut max: Option<u32> = None;
        // defaults to blanket
        let mut blanket: Option<bool> = Some(true);

        if let Some(values) = &query.values {
            currency = Some(values.currency.clone());
            min = Some(values.min);
            max = Some(values.max);
            blanket = None;
        }

        self.client.post(self.get_api_uri("/classifieds/alerts"))
            .query(&Params {
                token: &self.token,
                item_name: &query.item_name,
                intent: &query.intent,
                currency: &currency,
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
        struct Params<'a, 'b> {
            token: &'a str,
            item_name: &'b str,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: &'b ListingIntent,
        }
        
        self.client.delete(self.get_api_uri("/classifieds/alerts"))
            .query(&Params {
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
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}", self.get_api_uri("/classifieds/alerts"), id);
        self.client.delete(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_alert<'a>(&self, id: &str) -> Result<response::alert::Alert, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}", self.get_api_uri("/classifieds/alerts"), id);
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: response::alert::Alert = parses_response(response).await?;
            
        Ok(body)
    }

    pub async fn get_notification<'a>(&self, id: &str) -> Result<response::notification::Notification, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let response = self.client.get(format!("{}/{}", self.get_api_uri("/notifications"), id))
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: response::notification::Notification = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn delete_notification<'a>(&self, id: &str) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }

        let uri = format!("{}/{}", self.get_api_uri("/notifications"), id);
        self.client.delete(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_notifications<'a>(&self, query: &request::notification::GetNotification) -> Result<(Vec<response::notification::Notification>, response::cursor::Cursor), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
            unread: bool,
        }
        
        let uri = self.get_api_uri("/notifications");
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
                skip: query.skip,
                limit: query.limit,
                unread: query.unread,
            })
            .send()
            .await?;
        let body: GetNotificationsResponse = parses_response(response).await?;
        
        Ok((body.notifications, body.cursor))
    }

    pub async fn get_unread_notifications<'a>(&self) -> Result<(Vec<response::notification::Notification>, response::cursor::Cursor), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = self.get_api_uri("/notifications/unread");
        let response = self.client.post(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: GetNotificationsResponse = parses_response(response).await?;
        
        Ok((body.notifications, body.cursor))
    }

    pub async fn mark_unread_notifications<'a>(&'a self) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = self.get_api_uri("/notifications/unread");
        self.client.post(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_classifieds_snapshot<'a, 'b>(&self, sku: &'b str) -> Result<response::snapshot::Snapshot, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            sku: &'b str,
            appid: u32,
        }
        
        let uri = self.get_api_uri("/classifieds/listings/snapshot");
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
                appid: 440,
                sku,
            })
            .send()
            .await?;
        let body: response::snapshot::Snapshot = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_inventory_values<'a>(&self, steamid: &SteamID) -> Result<response::inventory::InventoryValues, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/values", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryValues = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn get_inventory_status<'a>(&self, steamid: &SteamID) -> Result<response::inventory::InventoryStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/status", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn refresh_inventory<'a>(&self, steamid: &SteamID) -> Result<response::inventory::InventoryStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/refresh", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.post(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_listings<'a>(&self, skip: u32, limit: u32) -> Result<(Vec<response::listing::Listing>, response::cursor::Cursor), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
        }
        
        let uri = self.get_api_uri("/v2/classifieds/listings");
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
                skip: skip,
                limit: limit,
            })
            .send()
            .await?;
        let body: GetListingsResponse = parses_response(response).await?;
        
        Ok((body.listings, body.cursor))
    }
    
    pub async fn create_listings<'a>(&self, query: &Vec<request::listing::CreateListing>) -> Result<CreateListingResult, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        if query.len() == 0 {
            return Err(APIError::ParameterError("No listings given"));
        } else if query.len() > 100 {
            return Err(APIError::ParameterError("Maximum of 100 listings allowed"));
        }
        
        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.post(uri)
            .query(&Params {
                token: &self.token,
            })
            .json(query)
            .send()
            .await?;
        let body: Vec<CreateListingResponse> = parses_response(response).await?;
        let mut result = CreateListingResult {
            success: Vec::new(),
            error: Vec::new(),
        };
        
        println!("{:?}", body);
        
        if body.len() != query.len() {
            return Err(APIError::ParameterError("Results and query have different number of listings"));
        }
        
        for (i, item) in body.iter().enumerate() {
            // take owned values from item
            let CreateListingResponse {
                error: error_option,
                result: listing_option,
            } = item.to_owned();
            
            if let Some(error) = error_option {
                // there should be a query at this index...
                result.error.push(ErrorListing {
                    message: error.message,
                    // this is guaranteed based on the length comparison check above
                    // it will need to be cloned
                    query: query[i].clone(),
                });
            } else if let Some(listing) = listing_option {
                result.success.push(listing);
            } else {
                return Err(APIError::ResponseError("Object with missing field".into()));
            }
        }
        
        Ok(result)
    }

    pub async fn delete_listing<'a, 'b>(&self, id: &'b str) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}", self.get_api_uri("/v2/classifieds/listings"), id);
        self.client.delete(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }
    
    pub async fn update_listing<'a, 'b>(&self, id: &'b str, query: &request::listing::UpdateListing) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            currencies: &'b Currencies,
            details: &'b Option<String>,
        }
        
        // todo implement response and change to v2 when available
        let uri = format!("{}/{}", self.get_api_uri("/v2/classifieds/listings"), id);
        self.client.patch(uri)
            .json(&Params {
                token: &self.token,
                currencies: &query.currencies,
                details: &query.details,
            })
            .send()
            .await?;
        
        Ok(())
    }
    
    pub async fn promote_listing<'a, 'b>(&self, id: &'b str) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/promote", self.get_api_uri("/v2/classifieds/listings"), id);
        self.client.post(uri)
            .json(&Params {
                token: &self.token
            })
            .send()
            .await?;
        
        Ok(())
    }
    
    pub async fn demote_listing<'a, 'b>(&self, id: &'b str) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = format!("{}/{}/demote", self.get_api_uri("/v2/classifieds/listings"), id);
        self.client.post(uri)
            .json(&Params {
                token: &self.token
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_listing_batch_limit<'a>(&self) -> Result<response::listing::BatchLimit, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.get(uri)
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: response::listing::BatchLimit = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn delete_listings<'a, 'b>(&self, listing_ids: &Vec<&'b str>) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b, 'c> {
            token: &'a str,
            listing_ids: &'c Vec<&'b str>,
        }
        
        // todo implement response and change to v2 when available
        let uri = self.get_api_uri("/classifieds/delete/v1");
        self.client.delete(uri)
            .json(&Params {
                token: &self.token,
                listing_ids,
            })
            .send()
            .await?;
        
        Ok(())
    }
    
    pub async fn create_listing<'a>(&self, query: &request::listing::CreateListing) -> Result<response::listing::Listing, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            #[serde(serialize_with = "option_number_to_str", skip_serializing_if = "Option::is_none")]
            id: Option<u64>,
            #[serde(serialize_with = "option_buy_listing_item_into_params", skip_serializing_if = "Option::is_none")]
            item: Option<&'b request::listing::BuyListingItem>,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: ListingIntent,
            #[serde(skip_serializing_if = "Option::is_none")]
            details: &'b Option<String>,
            buyout: bool,
            offers: bool,
            #[serde(serialize_with = "currencies_into_form")]
            currencies: &'b Currencies,
        }
        
        let params: Params = match query {
            request::listing::CreateListing::Buy(listing) => {
                Params {
                    token: &self.token,
                    id: None,
                    item: Some(&listing.item),
                    intent: ListingIntent::Buy,
                    buyout: listing.buyout,
                    offers: listing.offers,
                    details: &listing.details,
                    currencies: &listing.currencies,
                }
            },
            request::listing::CreateListing::Sell(listing) => {
                Params {
                    token: &self.token,
                    id: Some(listing.id),
                    item: None,
                    intent: ListingIntent::Sell,
                    buyout: listing.buyout,
                    offers: listing.offers,
                    details: &listing.details,
                    currencies: &listing.currencies,
                }
            },
        };
        let uri = self.get_api_uri("/v2/classifieds/listings");
        let response = self.client.post(uri)
            .json(&params)
            .send()
            .await?;
        let body: response::listing::Listing = parses_response(response).await?;
        
        println!("{}", serde_json::to_string(&params).unwrap());
        
        Ok(body)
    }

    pub async fn agent_pulse<'a>(&self, user_agent: &str) -> Result<response::agent::AgentStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            user_agent: &'b str,
        }
        
        let response = self.client.post(self.get_api_uri("/agent/pulse"))
            .query(&Params {
                token: &self.token,
                user_agent,
            })
            .send()
            .await?;
        let body: response::agent::AgentStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn agent_status<'a>(&self) -> Result<response::agent::AgentStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let response = self.client.post(self.get_api_uri("/agent/status"))
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        let body: response::agent::AgentStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn stop_agent<'a>(&self) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        self.client.post(self.get_api_uri("/agent/stop"))
            .query(&Params {
                token: &self.token,
            })
            .send()
            .await?;
        
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorListing {
    pub message: String,
    pub query: request::listing::CreateListing,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateListingResult {
    pub success: Vec<response::listing::Listing>,
    pub error: Vec<ErrorListing>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateListingResponse {
    pub error: Option<ErrorMessage>,
    pub result: Option<response::listing::Listing>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetUsersResponse {
    #[serde(deserialize_with = "bool_from_int")]
    success: bool,
    message: Option<String>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    current_time: Option<ServerTime>,
    players: Option<response::player::Players>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetUsersResponseWrapper {
    response: GetUsersResponse,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetNotificationsResponse {
    #[serde(rename(deserialize = "results"))]
    pub notifications: Vec<response::notification::Notification>,
    pub cursor: response::cursor::Cursor,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetListingsResponse {
    #[serde(rename(deserialize = "results"))]
    pub listings: Vec<response::listing::Listing>,
    pub cursor: response::cursor::Cursor,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GetAlertsResponse {
    #[serde(rename(deserialize = "results"))]
    pub alerts: Vec<response::alert::Alert>,
    pub cursor: response::cursor::Cursor,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;
    use serde::de::{DeserializeOwned};
    use tf2_enums::{Quality};

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
        let response: response::snapshot::Snapshot = tests::read_and_parse_file("get_classifieds_snapshot.json").unwrap();
        let listing = response.listings.iter().next().unwrap();
        
        assert_eq!(180, listing.currencies.keys);
    }
    
    #[test]
    fn parses_get_classifieds_snapshot_quality() {
        let response: response::snapshot::Snapshot = tests::read_and_parse_file("get_classifieds_snapshot.json").unwrap();
        let listing = response.listings.iter().next().unwrap();
        
        assert_eq!(Quality::Unusual, listing.item.quality);
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
        let response: GetAlertsResponse = tests::read_and_parse_file("get_alerts.json").unwrap();
        let alert = response.alerts.first().unwrap();

        assert_eq!("Purple Energy Danger".to_string(), alert.item_name);    
    }

    #[test]
    fn parses_get_notifications() {
        let response: GetNotificationsResponse = tests::read_and_parse_file("get_notifications.json").unwrap();
        let particle = response.notifications.first().as_ref().unwrap().bundle.listing.as_ref().unwrap().item.particle.as_ref().unwrap();

        assert_eq!("Purple Energy".to_string(), particle.name); 
    }

    #[test]
    fn parses_get_listings() {
        let response: GetListingsResponse = tests::read_and_parse_file("get_listings.json").unwrap();
        let listing = response.listings.first().unwrap();
        
        assert_eq!("Lucky Cat Hat".to_string(), listing.item.name); 
    }
}