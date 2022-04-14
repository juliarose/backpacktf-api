use std::{time::Duration,sync::Arc };
use crate::{
    SteamID,
    BackpackAPIBuilder, 
    error::Error,
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
use crate::tf2_price::traits::SerializeCurrencies;
use super::{api_response, helpers};
use async_std::task::sleep;
use serde::{Serialize, Deserialize};
use url::Url;
use reqwest::cookie::Jar;
use reqwest_middleware::ClientWithMiddleware;

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
    
    fn get_token(&self) -> Result<&str, Error> {
        if let Some(token) = &self.token {
            Ok(token)
        } else {
            Err(Error::MissingToken)
        }
    }
    
    fn get_key(&self) -> Result<&str, Error> {
        if let Some(key) = &self.key {
            Ok(key)
        } else {
            Err(Error::MissingKey)
        }
    }
    
    pub fn set_cookies(
        &self,
        cookies: &[String],
    ) {
        let uri = Self::HOSTNAME.parse::<Url>().unwrap();
        
        for cookie_str in cookies {
            self.cookies.add_cookie_str(cookie_str, &uri);
        }
    }
    
    pub async fn get_user(
        &self,
        steamid: &SteamID,
    ) -> Result<response::player::Player, Error> {
        let steamids: Vec<SteamID> = vec![*steamid];
        let players = self.get_users(&steamids).await?;
        
        if let Some(player) = players.get(steamid) {
            Ok(player.to_owned())
        } else {
            Err(Error::Response("No player with SteamID in response".into()))
        }
    }
    
    pub async fn get_users<'b>(
        &self,
        steamids: &'b [SteamID],
    ) -> Result<response::player::Players, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            key: &'a str,
            #[serde(serialize_with = "comma_delimited_steamids")]
            steamids: &'b [SteamID],
        }
        
        if steamids.is_empty() {
            return Err(Error::Parameter("No steamids given"));
        }
        
        let key = self.get_key()?;
        let uri = self.get_api_uri("/IGetUsers/v3");
        let response = self.client.get(uri)
            .query(&Params {
                key,
                steamids,
            })
            .send()
            .await?;
        let body: api_response::GetUsersResponseWrapper = helpers::parses_response(response).await?;
        
        if !body.response.success {
            Err(Error::Response(
                body.response.message
                    .unwrap_or_else(|| RESPONSE_UNSUCCESSFUL_MESSAGE.to_string())
            ))
        } else if let Some(players) = body.response.players {
            Ok(players)
        } else {
            Err(Error::Response("No players in response".into()))
        }
    }
    
    pub async fn get_all_alerts(
        &self,
        skip: u32,
    ) -> Result<Vec<response::alert::Alert>, Error> {
        let mut all = Vec::new();
        let mut limit = 100;
        let mut skip = skip;
        
        loop {
            let (mut alerts, cursor) = self.get_alerts(skip, limit).await?;
            
            all.append(&mut alerts);
            limit = cursor.limit;
            skip = cursor.skip + limit;
            
            if limit + skip >= cursor.total {
                // we done
                break;
            }
            
            // take a break
            sleep(Duration::from_secs(1)).await;
        }
        
        Ok(all)
    }
    
    pub async fn get_alerts(
        &self,
        skip: u32,
        limit: u32,
    ) -> Result<(Vec<response::alert::Alert>, response::cursor::Cursor), Error> {
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
        let body: api_response::GetAlertsResponse = helpers::parses_response(response).await?;
        
        Ok((body.alerts, body.cursor))
    }
    
    /// Creates an alert. If no price is given, creates a blanket alert.
    pub async fn create_alert(
        &self,
        item_name: &str,
        intent: &ListingIntent,
        price: Option<request::MinMax>,
    ) -> Result<response::alert::Alert, Error> {
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
        
        let uri = self.get_api_uri("/classifieds/alerts");
        let response = self.client.post(uri)
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
        let body: response::alert::Alert = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn delete_alert_by_name(
        &self,
        item_name: &str,
        intent: &ListingIntent,
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
    ) -> Result<response::alert::Alert, Error> {
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
        let body: response::alert::Alert = helpers::parses_response(response).await?;
            
        Ok(body)
    }

    pub async fn get_notification(
        &self,
        id: &str,
    ) -> Result<response::notification::Notification, Error> {
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
        let body: response::notification::Notification = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn delete_notification(
        &self,
        id: &str,
    ) -> Result<(), Error> {
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
    ) -> Result<(Vec<response::notification::Notification>, response::cursor::Cursor), Error> {
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
        let body: api_response::GetNotificationsResponse = helpers::parses_response(response).await?;
        
        Ok((body.notifications, body.cursor))
    }

    pub async fn get_unread_notifications(
        &self,
    ) -> Result<Vec<response::notification::Notification>, Error> {
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
        let notifications: Vec<response::notification::Notification> = helpers::parses_response(response).await?;
        
        Ok(notifications)
    }

    pub async fn mark_unread_notifications(
        &self,
    ) -> Result<(), Error> {
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
    ) -> Result<response::snapshot::Snapshot, Error> {
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
        let body: response::snapshot::Snapshot = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_inventory_values(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryValues, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/values", self.get_api_uri("/inventory"), u64::from(*steamid));
        let response = self.client.get(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryValues = helpers::parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn get_inventory_status(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryStatus, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/status", self.get_api_uri("/inventory"), u64::from(*steamid));
        let response = self.client.get(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryStatus = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn refresh_inventory(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryStatus, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/refresh", self.get_api_uri("/inventory"), u64::from(*steamid));
        let response = self.client.post(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::inventory::InventoryStatus = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn get_listings(
        &self,
        skip: u32,
        limit: u32,
    ) -> Result<(Vec<response::listing::Listing>, response::cursor::Cursor), Error> {
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
        let body: api_response::GetListingsResponse = helpers::parses_response(response).await?;
        
        Ok((body.listings, body.cursor))
    }
    
    pub async fn create_listings<'de, T>(
        &self,
        listings: Vec<request::CreateListing<T>>,
    ) -> Result<response::listing::create_listing::CreateListingsResult<T>, Error>
    where
        T: SerializeCurrencies<'de>
    {
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
        
        if listings.is_empty() {
            return Err(Error::Parameter("No listings given"));
        } else if listings.len() > 100 {
            return Err(Error::Parameter("Maximum of 100 listings allowed"));
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.post(uri)
            .query(&Params {
                token,
            })
            .json(&listings)
            .send()
            .await?;
        let body: Vec<CreateListingResponse> = helpers::parses_response(response).await?;
        let mut result = response::listing::create_listing::CreateListingsResult {
            success: Vec::new(),
            error: Vec::new(),
        };
        
        if body.len() != listings.len() {
            return Err(Error::Parameter("Results and query have different number of listings"));
        }
        
        for (response, query) in body.iter().zip(listings) {
            if let Some(error) = &response.error {
                // there should be a query at this index...
                result.error.push(response::listing::create_listing::ErrorListing {
                    message: error.message.to_owned(),
                    query,
                });
            } else if let Some(listing) = &response.result {
                result.success.push(listing.to_owned());
            } else {
                return Err(Error::Response("Object with missing field".into()));
            }
        }
        
        Ok(result)
    }

    pub async fn delete_listing(
        &self,
        id: &str,
    ) -> Result<(), Error> {
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
    
    pub async fn update_listing<'de, T>(
        &self,
        id: &str,
        details: Option<String>,
        currencies: &T,
    ) -> Result<response::listing::Listing, Error>
    where
        T: SerializeCurrencies<'de>
    {
        #[derive(Serialize, Debug)]
        struct JSONParams<'b, T>  {
            currencies: &'b T,
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
        let body: response::listing::Listing = helpers::parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn promote_listing(
        &self,
        id: &str,
    ) -> Result<response::listing::Listing, Error> {
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
        let body: response::listing::Listing = helpers::parses_response(response).await?;
        
        println!("{:?}", body);
        Ok(body)
    }
    
    pub async fn demote_listing(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = format!("{}/{}/demote", self.get_api_uri("/v2/classifieds/listings"), id);
        let _ = self.client.post(uri)
            .json(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn get_listing_batch_limit(
        &self,
    ) -> Result<response::listing::BatchLimit, Error> {
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
        let body: response::listing::BatchLimit = helpers::parses_response(response).await?;
        
        Ok(body)
    }
    
    pub async fn delete_listings(
        &self,
        listing_ids: &[String],
    ) -> Result<response::listing::delete_listing::DeleteListingsResult, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            listing_ids: &'b [String],
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
        let response: response::listing::delete_listing::DeleteListingsResult = helpers::parses_response(response).await?;
        
        Ok(response)
    }
    
    pub async fn create_listing<'de, T>(
        &self,
        listing: &request::CreateListing<T>,
    ) -> Result<response::listing::Listing, Error>
    where
        T: SerializeCurrencies<'de>
    {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b, T> {
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
            currencies: &'b T,
        }
        
        let token = self.get_token()?;
        let params: Params<T> = match listing {
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
        let body: response::listing::Listing = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn agent_pulse(
        &self,
        user_agent: &str,
    ) -> Result<response::agent::AgentStatus, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            user_agent: &'b str,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/agent/pulse");
        let response = self.client.post(uri)
            .query(&Params {
                token,
                user_agent,
            })
            .send()
            .await?;
        let body: response::agent::AgentStatus = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn agent_status(
        &self,
    ) -> Result<response::agent::AgentStatus, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/agent/status");
        let response = self.client.post(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: response::agent::AgentStatus = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    pub async fn stop_agent(
        &self,
    ) -> Result<(), Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/agent/stop");
        let _ = self.client.post(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        
        Ok(())
    }

    pub async fn classifieds_limits(
        &self,
    ) -> Result<response::classifieds_limits::ClassifiedsLimits, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/classifieds/limits");
        let response = self.client.post(uri)
            .query(&Params {
                token,
            })
            .send()
            .await?;
        let body: api_response::ClassifiedsLimitResponse = helpers::parses_response(response).await?;
        
        Ok(body.listings)
    }
}