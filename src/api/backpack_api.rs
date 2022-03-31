use std::sync::Arc;
use crate::{
    ListingIntent,
    currency_type::CurrencyType,
    request::{
        self,
        listing_serializers::option_buy_listing_item_into_params,
        serializers::{
            option_number_to_str,
            comma_delimited_steamids,
            listing_intent_enum_to_str,
            currency_type_enum_to_str,
        }
    },
    response,
};
use super::{
    APIError,
    api_response,
    helpers::parses_response,
};
use serde::{Serialize, Deserialize};
use url::Url;
use reqwest::cookie::Jar;
use reqwest_middleware::ClientWithMiddleware;
use steamid_ng::SteamID;
use crate::BackpackAPIBuilder;

const RESPONSE_UNSUCCESSFUL_MESSAGE: &str = "Empty response";

/// Interface for backpack.tf API endpoints.
pub struct BackpackAPI {
    key: Option<String>,
    token: Option<String>,
    cookies: Arc<Jar>,
    client: ClientWithMiddleware,
}

impl Default for BackpackAPI {
    
    fn default() -> Self {
        Self::builder().build()
    }
}

impl BackpackAPI {
    
    const HOSTNAME: &'static str = "backpack.tf";
    
    pub fn builder() -> BackpackAPIBuilder {
        BackpackAPIBuilder::default()
    }
    
    pub fn new(
        key: Option<String>,
        token: Option<String>,
        cookies: Arc<Jar>,
        client: ClientWithMiddleware
    ) -> Self {
        Self {
            key,
            token,
            cookies,
            client,
        }
    }
    
    fn get_uri(
        &self,
        pathname: &str,
    ) -> String {
        // https://backpack.tf/api/index.html#/webapi-users/800c12bc395d28d411e61a7aa5da1d9c
        format!("https://{}{}", Self::HOSTNAME, pathname)
    }

    fn get_api_uri(
        &self,
        endpoint: &str,
    ) -> String {
        format!("{}{}", self.get_uri("/api"), endpoint)
    }
    
    fn get_token(&self) -> Result<&str, APIError> {
        if let Some(token) = &self.token {
            Ok(token)
        } else {
            Err(APIError::MissingToken)
        }
    }
    
    fn get_key(&self) -> Result<&str, APIError> {
        if let Some(key) = &self.key {
            Ok(key)
        } else {
            Err(APIError::MissingKey)
        }
    }
    
    pub fn set_cookies(
        &self,
        cookies: &[String],
    ) {
        let url = Self::HOSTNAME.parse::<Url>().unwrap();
        
        for cookie_str in cookies {
            self.cookies.add_cookie_str(cookie_str, &url);
        }
    }
    
    pub async fn get_user(
        &self,
        steamid: &SteamID,
    ) -> Result<response::player::Player, APIError> {
        let steamids: Vec<SteamID> = vec![*steamid];
        let players = self.get_users(&steamids).await?;
        
        if let Some(player) = players.get(steamid) {
            Ok(player.to_owned())
        } else {
            Err(APIError::Response("No player with SteamID in response".into()))
        }
    }
    
    pub async fn get_users<'b>(
        &self,
        steamids: &'b [SteamID],
    ) -> Result<response::player::Players, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            key: &'a str,
            #[serde(serialize_with = "comma_delimited_steamids")]
            steamids: &'b [SteamID],
        }
        
        if steamids.len() == 0 {
            return Err(APIError::Parameter("No steamids given"));
        }
        
        let key = self.get_key()?;
        let response = self.client.get(self.get_api_uri("/IGetUsers/v3"))
            .query(&Params {
                key,
                steamids,
            })
            .send()
            .await?;
        let body: api_response::GetUsersResponseWrapper = parses_response(response).await?;
        
        if !body.response.success {
            return Err(APIError::Response(body.response.message.unwrap_or(RESPONSE_UNSUCCESSFUL_MESSAGE.to_string()).into()));
        } else if let Some(players) = body.response.players {
            Ok(players)
        } else {
            Err(APIError::Response("No players in response".into()))
        }
    }
    
    pub async fn get_alerts(
        &self,
        skip: u32,
        limit: u32,
    ) -> Result<(Vec<response::alert::Alert>, response::cursor::Cursor), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/classifieds/alerts");
        let response = self.client.get(uri)
            .query(&Params {
                token,
                limit,
                skip,
            })
            .send()
            .await?;
        let body: api_response::GetAlertsResponse = parses_response(response).await?;
        
        Ok((body.alerts, body.cursor))
    }
    
    /// Creates an alert. If no price is given, creates a blanket alert.
    pub async fn create_alert(
        &self,
        item_name: &str,
        intent: &ListingIntent,
        price: Option<request::MinMax>,
    ) -> Result<response::alert::Alert, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            item_name: &'b str,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: &'b ListingIntent,
            #[serde(serialize_with = "currency_type_enum_to_str")]
            currency: Option<CurrencyType>,
            min: Option<f32>,
            max: Option<f32>,
            blanket: Option<bool>,
        }
        
        let token = self.get_token()?;
        let mut currency: Option<CurrencyType> = None;
        let mut min: Option<f32> = None;
        let mut max: Option<f32> = None;
        // defaults to blanket
        let mut blanket: Option<bool> = Some(true);

        if let Some(values) = &price {
            currency = Some(values.currency.clone());
            min = Some(values.min);
            max = Some(values.max);
            blanket = None;
        }
        
        let response = self.client.post(self.get_api_uri("/classifieds/alerts"))
            .json(&Params {
                token,
                item_name,
                intent,
                currency,
                min,
                max,
                blanket,
            })
            .send()
            .await?;
        let body: response::alert::Alert = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn delete_alert_by_name(
        &self,
        item_name: &str,
        intent: &ListingIntent,
    ) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            item_name: &'b str,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: &'b ListingIntent,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/classifieds/alerts");
        let _ = self.client.delete(uri)
            .query(&Params {
                token,
                item_name,
                intent,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn delete_alert(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}", self.get_api_uri("/classifieds/alerts"), id);
        let _ = self.client.delete(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_alert(
        &self,
        id: &str,
    ) -> Result<response::alert::Alert, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}", self.get_api_uri("/classifieds/alerts"), id);
        let response = self.client.get(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::alert::Alert = parses_response(response).await?;
            
        Ok(body)
    }

    pub async fn get_notification(
        &self,
        id: &str,
    ) -> Result<response::notification::Notification, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let response = self.client.get(format!("{}/{}", self.get_api_uri("/notifications"), id))
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::notification::Notification = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn delete_notification(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }

        let token = self.get_token()?;
        let uri = format!("{}/{}", self.get_api_uri("/notifications"), id);
        self.client.delete(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_notifications(
        &self,
        skip: u32,
        limit: u32,
        unread: bool,
    ) -> Result<(Vec<response::notification::Notification>, response::cursor::Cursor), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
            unread: bool,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/notifications");
        let response = self.client.get(uri)
            .query(&Params {
                token,
                skip,
                limit,
                unread,
            })
            .send()
            .await?;
        let body: api_response::GetNotificationsResponse = parses_response(response).await?;
        
        Ok((body.notifications, body.cursor))
    }

    pub async fn get_unread_notifications(
        &self,
    ) -> Result<Vec<response::notification::Notification>, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/notifications/unread");
        let response = self.client.post(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let notifications: Vec<response::notification::Notification> = parses_response(response).await?;
        
        Ok(notifications)
    }

    pub async fn mark_unread_notifications(
        &self,
    ) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/notifications/unread");
        self.client.post(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_classifieds_snapshot(
        &self,
        sku: &str,
    ) -> Result<response::snapshot::Snapshot, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            sku: &'b str,
            appid: u32,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/classifieds/listings/snapshot");
        let response = self.client.get(uri)
            .query(&Params {
                token,
                appid: 440,
                sku,
            })
            .send()
            .await?;
        let body: response::snapshot::Snapshot = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_inventory_values(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryValues, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/values", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.get(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryValues = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn get_inventory_status(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/status", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.get(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn refresh_inventory(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/refresh", self.get_api_uri("/inventory"), u64::from(steamid.clone()));
        let response = self.client.post(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_listings(
        &self,
        skip: u32,
        limit: u32,
    ) -> Result<(Vec<response::listing::Listing>, response::cursor::Cursor), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
            skip: u32,
            limit: u32,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/listings");
        let response = self.client.get(uri)
            .query(&Params {
                token,
                skip,
                limit,
            })
            .send()
            .await?;
        let body: api_response::GetListingsResponse = parses_response(response).await?;
        
        Ok((body.listings, body.cursor))
    }
    
    pub async fn create_listings(
        &self,
        listings: &[request::CreateListing],
    ) -> Result<response::listing::create_listing::CreateListingsResult, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        #[derive(Deserialize, Debug)]
        struct CreateListingResponse {
            error: Option<ErrorMessage>,
            result: Option<response::listing::Listing>,
        }
        
        #[derive(Deserialize, Debug)]
        struct ErrorMessage {
            message: String,
        }
        
        if listings.len() == 0 {
            return Err(APIError::Parameter("No listings given"));
        } else if listings.len() > 100 {
            return Err(APIError::Parameter("Maximum of 100 listings allowed"));
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.post(uri)
            .query(&Params {
                token,
            })
            .json(listings)
            .send()
            .await?;
        let body: Vec<CreateListingResponse> = parses_response(response).await?;
        let mut result = response::listing::create_listing::CreateListingsResult {
            success: Vec::new(),
            error: Vec::new(),
        };
        
        if body.len() != listings.len() {
            return Err(APIError::Parameter("Results and query have different number of listings"));
        }
        
        for (i, response) in body.iter().enumerate() {
            if let Some(error) = &response.error {
                // there should be a query at this index...
                result.error.push(response::listing::create_listing::ErrorListing {
                    message: error.message.to_owned(),
                    // this is guaranteed based on the length comparison check above
                    // it will need to be cloned
                    query: listings[i].clone(),
                });
            } else if let Some(listing) = &response.result {
                result.success.push(listing.to_owned());
            } else {
                return Err(APIError::Response("Object with missing field".into()));
            }
        }
        
        Ok(result)
    }

    pub async fn delete_listing(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}", self.get_api_uri("/v2/classifieds/listings"), id);
        // This does not produce an output
        self.client.delete(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }
    
    pub async fn update_listing(
        &self,
        id: &str,
        details: Option<String>,
        currencies: &request::Currencies,
    ) -> Result<response::listing::Listing, APIError> {
        #[derive(Serialize, Debug)]
        struct JSONParams<'b> {
            currencies: &'b request::Currencies,
            details: Option<String>,
        }
        
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}", self.get_api_uri("/v2/classifieds/listings"), id);
        let response = self.client.patch(uri)
            .json(&JSONParams {
                currencies,
                details,
            })
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::listing::Listing = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn promote_listing(
        &self,
        id: &str,
    ) -> Result<response::listing::Listing, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/promote", self.get_api_uri("/v2/classifieds/listings"), id);
        let response = self.client.post(uri)
            .json(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::listing::Listing = parses_response(response).await?;
        
        println!("{:?}", body);
        Ok(body)
    }
    
    pub async fn demote_listing(
        &self,
        id: &str,
    ) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/demote", self.get_api_uri("/v2/classifieds/listings"), id);
        self.client.post(uri)
            .json(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_listing_batch_limit(
        &self,
    ) -> Result<response::listing::BatchLimit, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.get(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::listing::BatchLimit = parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn delete_listings(
        &self,
        listing_ids: &Vec<String>,
    ) -> Result<response::listing::delete_listing::DeleteListingsResult, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            listing_ids: &'b Vec<String>,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/classifieds/delete/v1");
        let response = self.client.delete(uri)
            .json(&Params {
                token,
                listing_ids,
            })
            .send()
            .await?;
        let response: response::listing::delete_listing::DeleteListingsResult = parses_response(response).await?;
        
        Ok(response)
    }
    
    pub async fn create_listing(
        &self,
        listing: &request::CreateListing,
    ) -> Result<response::listing::Listing, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            #[serde(serialize_with = "option_number_to_str", skip_serializing_if = "Option::is_none")]
            id: Option<u64>,
            #[serde(serialize_with = "option_buy_listing_item_into_params", skip_serializing_if = "Option::is_none")]
            item: Option<&'b request::BuyListingItem>,
            #[serde(serialize_with = "listing_intent_enum_to_str")]
            intent: ListingIntent,
            #[serde(skip_serializing_if = "Option::is_none")]
            details: &'b Option<String>,
            buyout: &'b bool,
            offers: &'b bool,
            currencies: &'b request::Currencies,
        }
        
        let token = self.get_token()?;
        let params: Params = match listing {
            request::CreateListing::Buy {
                item,
                currencies,
                details,
                buyout,
                offers,
            } => {
                Params {
                    token,
                    id: None,
                    item: Some(item),
                    intent: ListingIntent::Buy,
                    buyout,
                    offers,
                    details,
                    currencies,
                }
            },
            request::CreateListing::Sell {
                id,
                currencies,
                details,
                buyout,
                offers,
            } => {
                Params {
                    token,
                    id: Some(*id),
                    item: None,
                    intent: ListingIntent::Sell,
                    buyout,
                    offers,
                    details,
                    currencies,
                }
            },
        };
        let uri = self.get_api_uri("/v2/classifieds/listings");
        let response = self.client.post(uri)
            .json(&params)
            .send()
            .await?;
        let body: response::listing::Listing = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn agent_pulse(
        &self,
        user_agent: &str,
    ) -> Result<response::agent::AgentStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            user_agent: &'b str,
        }
        
        let token = self.get_token()?;
        let response = self.client.post(self.get_api_uri("/agent/pulse"))
            .query(&Params {
                token,
                user_agent,
            })
            .send()
            .await?;
        let body: response::agent::AgentStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn agent_status(
        &self,
    ) -> Result<response::agent::AgentStatus, APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let response = self.client.post(self.get_api_uri("/agent/status"))
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::agent::AgentStatus = parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn stop_agent(
        &self,
    ) -> Result<(), APIError> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        self.client.post(self.get_api_uri("/agent/stop"))
            .query(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }
}