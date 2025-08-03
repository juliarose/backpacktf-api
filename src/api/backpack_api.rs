use super::{api_response, helpers};
use crate::{SteamID, BackpackAPIBuilder, ListingIntent};
use crate::error::{Error, ParameterError};
use crate::currency_type::CurrencyType;
use crate::response;
use crate::request::{self, listing_serializers::option_buy_listing_item_into_params, serializers};
use std::borrow::Borrow;
use std::time::Duration;
use async_std::task::sleep;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use reqwest_middleware::ClientWithMiddleware;

const RESPONSE_UNSUCCESSFUL_MESSAGE: &str = "Empty response";
const APPID_TEAM_FORTRESS_2: u32 = 440;
const MAX_LISTINGS_REQUEST_LIMIT: usize = 100;
const MAX_ALERTS_REQUEST_LIMIT: usize = 100;

/// Interface for backpack.tf API endpoints.
#[derive(Debug, Clone)]
pub struct BackpackAPI {
    key: Option<String>,
    token: Option<String>,
    client: ClientWithMiddleware,
}

impl Default for BackpackAPI {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl BackpackAPI {
    const HOSTNAME: &'static str = "backpack.tf";
    
    /// Creates a new builder for a [`BackpackAPI`].
    pub fn builder() -> BackpackAPIBuilder {
        BackpackAPIBuilder::default()
    }
    
    /// Creates a new [`BackpackAPI`] with the given key and token.
    pub(crate) fn new(
        key: Option<String>,
        token: Option<String>,
        client: ClientWithMiddleware,
    ) -> Self {
        Self {
            key,
            token,
            client,
        }
    }
    
    /// Gets the URI for an API endpoint.
    fn get_api_uri(
        &self,
        endpoint: &str,
    ) -> String {
        format!("https://api.{}/api{}", Self::HOSTNAME, endpoint)
    }
    
    /// Gets the token for the API.
    fn get_token(&self) -> Result<&str, Error> {
        if let Some(token) = &self.token {
            Ok(token)
        } else {
            Err(ParameterError::MissingToken.into())
        }
    }
    
    /// Gets the key for the API.
    fn get_key(&self) -> Result<&str, Error> {
        if let Some(key) = &self.key {
            Ok(key)
        } else {
            Err(ParameterError::MissingKey.into())
        }
    }
    
    /// Sends a GET request.
    async fn get<T, D>(
        &self,
        uri: &str,
        query: &T,
    ) -> Result<D, Error>
    where
        T: Serialize,
        D: DeserializeOwned,
    {
        let uri = self.get_api_uri(uri);
        let request = self.client.get(uri)
            .query(query);
        let response = request
            .send()
            .await?;
        
        helpers::parses_response::<D>(response).await
    }
    
    /// Sends a DELETE request.
    async fn delete<T>(
        &self,
        uri: &str,
        query: &T,
    ) -> Result<(), Error>
    where
        T: Serialize,
    {
        let uri = self.get_api_uri(uri);
        let request = self.client.delete(uri)
            .query(query);
        let _ = request
            .send()
            .await?;
        
        Ok(())
    }
    
    /// Sends a POST request.
    async fn post<T, D>(
        &self,
        uri: &str,
        query: &T,
    ) -> Result<D, Error>
    where
        T: Serialize,
        D: DeserializeOwned,
    {
        let uri = self.get_api_uri(uri);
        let request = self.client.post(uri)
            .query(query);
        let response = request
            .send()
            .await?;
        
        helpers::parses_response::<D>(response).await
    }
    
    /// Sends a POST request and parses the JSON response body.
    async fn post_json<T, D>(
        &self,
        uri: &str,
        json: &T,
    ) -> Result<D, Error>
    where
        T: Serialize,
        D: DeserializeOwned,
    {
        let uri = self.get_api_uri(uri);
        let request = self.client.post(uri)
            .json(json);
        let response = request
            .send()
            .await?;
        
        helpers::parses_response::<D>(response).await
    }
    
    /// Gets details about a user.
    pub async fn get_user_v1<'b>(
        &self,
        steamid: &SteamID,
    ) -> Result<response::player::PlayerV1, Error> {
        let steamids: Vec<SteamID> = vec![*steamid];
        let mut players = self.get_users_v1(&steamids).await?;
        
        if let Some(player) = players.remove(steamid) {
            Ok(player)
        } else {
            Err(Error::Response("No player with SteamID in response".into()))
        }
    }
    
    /// Gets details about users.
    pub async fn get_users_v1<'b>(
        &self,
        steamids: &'b [SteamID],
    ) -> Result<response::player::PlayersV1, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            key: &'a str,
            #[serde(serialize_with = "serializers::comma_delimited_steamids")]
            steamids: &'b [SteamID],
        }
        
        if steamids.is_empty() {
            return Err(ParameterError::Empty {
                name: "steamids",
            }.into());
        }
        
        let key = self.get_key()?;
        let body: api_response::GetUsersV1Response = self.get(
            "/users/info/v1",
            &Params {
                key,
                steamids,
            }
        ).await?;
        
        Ok(body.users)
    }
    
    /// Gets details about a user including name, bans, trust scores, and inventory values.
    pub async fn get_user(
        &self,
        steamid: &SteamID,
    ) -> Result<response::player::Player, Error> {
        let steamids: Vec<SteamID> = vec![*steamid];
        let mut players = self.get_users(&steamids).await?;
        
        if let Some(player) = players.remove(steamid) {
            Ok(player)
        } else {
            Err(Error::Response("No player with SteamID in response".into()))
        }
    }
    
    /// Gets details about users including name, bans, trust scores, and inventory values.
    pub async fn get_users<'b>(
        &self,
        steamids: &'b [SteamID],
    ) -> Result<response::player::Players, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            key: &'a str,
            #[serde(serialize_with = "serializers::comma_delimited_steamids")]
            steamids: &'b [SteamID],
        }
        
        if steamids.is_empty() {
            return Err(ParameterError::Empty {
                name: "steamids",
            }.into());
        }
        
        let key = self.get_key()?;
        let body: api_response::GetUsersResponseWrapper = self.get(
            "/IGetUsers/v3",
            &Params {
                key,
                steamids,
            }
        ).await?;
        
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
    
    /// Gets a page of alerts along with a cursor for scrolling.
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
        let body: api_response::GetAlertsResponse = self.get(
            "/classifieds/alerts",
            &Params {
                token,
                limit,
                skip,
            },
        ).await?;
        
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
            #[serde(serialize_with = "serializers::listing_intent_enum_to_str")]
            intent: &'b ListingIntent,
            #[serde(serialize_with = "serializers::currency_type_enum_to_str")]
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
            currency = Some(values.currency);
            min = Some(values.min);
            max = Some(values.max);
            blanket = None;
        }
        
        let alert: response::alert::Alert = self.post_json(
            "/classifieds/alerts",
            &Params {
                token,
                item_name,
                intent,
                currency,
                min,
                max,
                blanket,
            },
        ).await?;
        
        Ok(alert)
    }

    /// Deletes an alert by its name.
    pub async fn delete_alert_by_name(
        &self,
        item_name: &str,
        intent: &ListingIntent,
    ) -> Result<(), Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b> {
            token: &'a str,
            item_name: &'b str,
            #[serde(serialize_with = "serializers::listing_intent_enum_to_str")]
            intent: &'b ListingIntent,
        }
        
        let token = self.get_token()?;
        
        self.delete(
            "/classifieds/alerts",
            &Params {
                token,
                item_name,
                intent,
            },
        ).await
    }

    /// Deletes an alert using its ID.
    pub async fn delete_alert(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        
        self.delete(
            &format!("/classifieds/alerts/{id}"),
            &Token {
                token,
            },
        ).await
    }

    /// Gets an alert.
    pub async fn get_alert(
        &self,
        id: &str,
    ) -> Result<response::alert::Alert, Error> {
        let token = self.get_token()?;
        let alert: response::alert::Alert = self.get(
            &format!("/classifieds/alerts/{id}"),
            &Token {
                token,
            }
        ).await?;
            
        Ok(alert)
    }

    /// Gets a notification.
    pub async fn get_notification(
        &self,
        id: &str,
    ) -> Result<response::notification::Notification, Error> {
        let token = self.get_token()?;
        let notification: response::notification::Notification = self.get(
            &format!("/notifications/{id}"),
            &Token {
                token,
            }
        ).await?;
        
        Ok(notification)
    }

    /// Deletes a notification.
    pub async fn delete_notification(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        
        self.delete(
            &format!("/notifications/{id}"),
            &Token {
                token,
            }
        ).await
    }

    /// Gets notifications along with a cursor for scrolling results.
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
        let body: api_response::GetNotificationsResponse = self.get(
            "/notifications",
            &Params {
                token,
                skip,
                limit,
                unread,
            },
        ).await?;
        
        Ok((body.notifications, body.cursor))
    }

    /// Gets unread notifications.
    pub async fn get_unread_notifications(
        &self,
    ) -> Result<Vec<response::notification::Notification>, Error> {
        let token = self.get_token()?;
        let notifications: Vec<response::notification::Notification> = self.post(
            "/notifications/unread",
            &Token {
                token,
            },
        ).await?;
        
        Ok(notifications)
    }
    
    /// Marks notifications as read.
    pub async fn mark_unread_notifications(
        &self,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        
        self.post(
            "/notifications/unread",
            &Token {
                token,
            },
        ).await
    }

    /// Gets a classifieds snapshot. SKU is the name of an item e.g. "Strange Pain Train".
    pub async fn get_snapshot(
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
        let snapshot: response::snapshot::Snapshot = self.get(
            "/classifieds/listings/snapshot",
            &Params {
                token,
                appid: APPID_TEAM_FORTRESS_2,
                sku,
            },
        ).await?;
        
        Ok(snapshot)
    }

    /// Gets the values of an inventory.
    pub async fn get_inventory_values(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryValues, Error> {
        let token = self.get_token()?;
        let inventory_values: response::inventory::InventoryValues = self.get(
            &format!("/inventory/{}/values", u64::from(*steamid)),
            &Token {
                token,
            },
        ).await?;
        
        Ok(inventory_values)
    }
    
    /// Gets the current state of an inventory.
    pub async fn get_inventory_status(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryStatus, Error> {
        let token = self.get_token()?;
        let inventory_status: response::inventory::InventoryStatus = self.get(
            &format!("/inventory/{}/status", u64::from(*steamid)),
            &Token {
                token,
            },
        ).await?;
        
        Ok(inventory_status)
    }

    /// Refreshes the state of an inventory.
    pub async fn refresh_inventory(
        &self,
        steamid: &SteamID,
    ) -> Result<response::inventory::InventoryStatus, Error> {
        let token = self.get_token()?;
        let inventory_status: response::inventory::InventoryStatus = self.post(
            &format!("/inventory/{}/refresh", u64::from(*steamid)),
            &Token {
                token,
            },
        ).await?;
        
        Ok(inventory_status)
    }
    
    /// Gets a page of listings from the archive along with the cursor for scrolling.
    pub async fn get_archived_listings(
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
        let body: api_response::GetListingsResponse = self.get(
            "/v2/classifieds/archive",
            &Params {
                token,
                skip,
                limit,
            },
        ).await?;
        
        Ok((body.listings, body.cursor))
    }
    
    /// Deletes a listing from the archive.
    pub async fn delete_archived_listing(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        
        self.delete(
            &format!("/v2/classifieds/archive/{id}"),
            &Token {
                token,
            },
        ).await
    }
    
    /// Deletes all listings from the archive.
    pub async fn delete_all_archived_listings(
        &self,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/archive");
        let _response = self.client.delete(uri)
            .json(&Token {
                token,
            })
            .send()
            .await?;
            
        // todo check the response
        
        Ok(())
    }
    
    /// Deletes listings from the archive. A limit of 100 listings is imposed. Currently does not 
    /// work.
    pub async fn delete_archived_listings<T>(
        &self,
        listing_ids: &[T],
    ) -> Result<u32, Error> 
    where
        T: Borrow<String> + Serialize,
    {
        #[derive(Serialize, Debug)]
        struct Params<'a, T> {
            listing_ids: &'a [T],
        }
        
        if listing_ids.is_empty() {
            return Err(ParameterError::Empty {
                name: "listing_ids",
            }.into());
        }
        
        if listing_ids.len() > MAX_LISTINGS_REQUEST_LIMIT {
            return Err(ParameterError::MaximumLengthExceeded {
                name: "listing_ids",
                max: MAX_LISTINGS_REQUEST_LIMIT,
            }.into());
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/archive/batch");
        let response = self.client.delete(uri)
            .json(&Params {
                listing_ids,
            })
            .query(&Token {
                token,
            })
            .send()
            .await?;
        let response: api_response::DeleteListingsResult = helpers::parses_response(response).await?;
        
        Ok(response.deleted)
    }
    
    /// Updates a listing from the archive. Note that any type can be used for the currencies 
    /// parameter as long as it implements all traits under [`SerializeCurrencies`].
    pub async fn update_archived_listing<T>(
        &self,
        id: &str,
        details: Option<String>,
        currencies: &T,
    ) -> Result<response::listing::update_listing::SuccessListing, Error>
    where
        T: Serialize
    {
        #[derive(Serialize, Debug)]
        struct JSONParams<'b, T>  {
            currencies: &'b T,
            details: Option<String>,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri(&format!("/v2/classifieds/archive/{id}"));
        let response = self.client.patch(uri)
            .json(&JSONParams {
                currencies,
                details,
            })
            .query(&Token {
                token,
            })
            .send()
            .await?;
        let body: response::listing::update_listing::SuccessListing = helpers::parses_response(response).await?;
        
        Ok(body)
    }

    /// Publishes a listing from the archive to the active pool.
    pub async fn publish_archived_listing(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        let uri = self.get_api_uri(&format!("/v2/classifieds/archive/{id}/publish"));
        let _response = self.client.post(uri)
            .query(&Token {
                token,
            })
            .send()
            .await?;
        
        // todo check the response
            
        Ok(())
    }

    /// Gets a listing.
    pub async fn get_listing(
        &self,
        id: &str,
    ) -> Result<response::listing::Listing, Error> {
        #[derive(Serialize, Debug)]
        struct Params<'a> {
            token: &'a str,
        }

        let token = self.get_token()?;
        let listing: response::listing::Listing = self.get(
            &format!("/v2/classifieds/listings/{id}"),
            &Params {
                token
            }
        ).await?;

        Ok(listing)
    }
    
    /// Gets a page of listings along with the cursor for scrolling.
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
        let body: api_response::GetListingsResponse = self.get(
            "/v2/classifieds/listings",
            &Params {
                token,
                skip,
                limit,
            },
        ).await?;
        
        Ok((body.listings, body.cursor))
    }
    
    /// Creates a listing.
    pub async fn create_listing<T>(
        &self,
        listing: &request::CreateListing<T>,
    ) -> Result<response::listing::Listing, Error>
    where
        T: Serialize
    {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b, T> {
            token: &'a str,
            #[serde(serialize_with = "serializers::option_number_to_str", skip_serializing_if = "Option::is_none")]
            id: Option<u64>,
            #[serde(serialize_with = "option_buy_listing_item_into_params", skip_serializing_if = "Option::is_none")]
            item: Option<&'b request::BuyListingItem>,
            #[serde(serialize_with = "serializers::listing_intent_enum_to_str")]
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
            } => Params {
                token,
                id: None,
                item: Some(item),
                intent: ListingIntent::Buy,
                buyout,
                offers,
                details,
                currencies,
            },
            request::CreateListing::Sell {
                id,
                currencies,
                details,
                buyout,
                offers,
            } => Params {
                token,
                id: Some(*id),
                item: None,
                intent: ListingIntent::Sell,
                buyout,
                offers,
                details,
                currencies,
            },
        };
        let listing: response::listing::Listing = self.post_json(
            "/v2/classifieds/listings",
            &params,
        ).await?;
        
        Ok(listing)
    }
    
    /// Creates listings. A limit of 100 listings is imposed. Note that any type can be used for 
    /// the currencies parameter as long as it implements all traits under [`SerializeCurrencies`].
    pub async fn create_listings<'a, T>(
        &self,
        listings: &'a [request::CreateListing<T>],
    ) -> Result<Vec<response::listing::create_listing::Result<'a, T>>, Error>
    where
        T: Serialize
    {
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
            return Err(ParameterError::Empty {
                name: "listings",
            }.into());
        }
        
        if listings.len() > MAX_LISTINGS_REQUEST_LIMIT {
            return Err(ParameterError::MaximumLengthExceeded {
                name: "listings",
                max: MAX_LISTINGS_REQUEST_LIMIT,
            }.into());
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.post(uri)
            .query(&Token {
                token,
            })
            .json(&listings)
            .send()
            .await?;
        let body: Vec<CreateListingResponse> = helpers::parses_response(response).await?;
        
        if body.len() != listings.len() {
            return Err(Error::Response("Results and query have different number of listings".into()));
        }
        
        let mut results = Vec::with_capacity(body.len());
        
        for (response, query) in body.into_iter().zip(listings) {
            let result = response.result;
            let error = response.error;
            
            if let Some(error) = error {
                // there should be a query at this index...
                results.push(Err(response::listing::create_listing::ErrorListing {
                    message: error.message,
                    query,
                }));
            } else if let Some(listing) = result {
                results.push(Ok(listing));
            } else {
                return Err(Error::Response("Object with missing field".into()));
            }
        }
        
        Ok(results)
    }

    /// Deletes a listing.
    pub async fn delete_listing(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        
        self.delete(
            &format!("/v2/classifieds/listings/{id}"),
            &Token {
                token,
            },
        ).await
    }
    
    /// Deletes listings. A limit of 100 listings is imposed.
    pub async fn delete_listings<T>(
        &self,
        listing_ids: &[T],
    ) -> Result<u32, Error>
    where
        T: Borrow<String> + Serialize,
    {
        #[derive(Serialize, Debug)]
        struct Params<'a, 'b, T> {
            token: &'a str,
            listing_ids: &'b [T],
        }
        
        if listing_ids.is_empty() {
            return Err(ParameterError::Empty {
                name: "listing_ids",
            }.into());
        }
        
        if listing_ids.len() > MAX_LISTINGS_REQUEST_LIMIT {
            return Err(ParameterError::MaximumLengthExceeded {
                name: "listing_ids",
                max: MAX_LISTINGS_REQUEST_LIMIT,
            }.into());
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
        let response: api_response::DeleteListingsResult = helpers::parses_response(response).await?;
        
        Ok(response.deleted)
    }
    
    /// Updates a listing. Note that any type can be used for the currencies parameter as long as 
    /// it implements all traits under [`SerializeCurrencies`].
    pub async fn update_listing<T>(
        &self,
        id: &str,
        details: Option<String>,
        currencies: &T,
    ) -> Result<response::listing::update_listing::SuccessListing, Error>
    where
        T: Serialize
    {
        #[derive(Serialize, Debug)]
        struct JSONParams<'b, T>  {
            currencies: &'b T,
            details: Option<String>,
        }
        
        let token = self.get_token()?;
        let uri = self.get_api_uri(&format!("/v2/classifieds/listings/{id}"));
        let response = self.client.patch(uri)
            .json(&JSONParams {
                currencies,
                details,
            })
            .query(&Token {
                token,
            })
            .send()
            .await?;
        let body: response::listing::update_listing::SuccessListing = helpers::parses_response(response).await?;
        
        Ok(body)
    }
    
    /// Updates listings. A limit of 100 listings is imposed. Note that any type can be used for 
    /// the currencies parameter as long as it implements all traits under [`SerializeCurrencies`].
    pub async fn update_listings<'a, T>(
        &self,
        listings: &'a [request::UpdateListing<T>],
    ) -> Result<Vec<response::listing::update_listing::Result<'a, T>>, Error>
    where
        T: Serialize
    {
        #[derive(Deserialize, Debug)]
        struct ErrorResult {
            index: usize,
            message: String,
        }
        
        #[derive(Deserialize, Debug)]
        struct UpdateListingsResponse {
            updated: Vec<response::listing::update_listing::SuccessListing>,
            errors: Vec<ErrorResult>,
        }
        
        #[derive(Serialize, Debug)]
        pub struct Body<'a, T> {
            currencies: &'a T,
            details: &'a Option<String>,
        }
        
        #[derive(Serialize, Debug)]
        pub struct Listing<'a, T> {
            id: &'a str,
            body: Body<'a, T>,
        }
        
        if listings.is_empty() {
            return Err(ParameterError::Empty {
                name: "listings",
            }.into());
        }
        
        if listings.len() > MAX_LISTINGS_REQUEST_LIMIT {
            return Err(ParameterError::MaximumLengthExceeded {
                name: "listings",
                max: MAX_LISTINGS_REQUEST_LIMIT,
            }.into());
        }
        
        let mapped = listings
            .iter()
            .map(|update| Listing {
                id: &update.id,
                body: Body {
                    currencies: &update.currencies,
                    details: &update.details,
                },
            })
            .collect::<Vec<_>>();
        let token = self.get_token()?;
        let uri = self.get_api_uri("/v2/classifieds/listings/batch");
        let response = self.client.patch(uri)
            .json(&mapped)
            .query(&Token {
                token,
            })
            .send()
            .await?;
        let body: UpdateListingsResponse = helpers::parses_response(response).await?;
        
        if body.updated.len() + body.errors.len() != listings.len() {
            return Err(Error::Response("Results and query have different number of listings".into()));
        }
        
        let mut results = body.updated
            .into_iter()
            .map(Ok)
            .collect::<Vec<_>>();
        
        for error in body.errors {
            if let Some(query) = listings.get(error.index) {
                results.push(Err(response::listing::update_listing::ErrorListing {
                    message: error.message,
                    query,
                }))
            } else {
                // probably shouldn't ever happen but who knows
                return Err(Error::Response(format!("Missing index `{}`: {}", error.index, error.message)));
            }
        }
        
        Ok(results)
    }
    
    /// Sets a listing to promoted.
    pub async fn promote_listing(
        &self,
        id: &str,
    ) -> Result<response::listing::Listing, Error> {
        let token = self.get_token()?;
        let listing: response::listing::Listing = self.post_json(
            &format!("/v2/classifieds/listings/{id}/promote"),
            &Token {
                token,
            },
        ).await?;
        
        Ok(listing)
    }
    
    /// Demotes a listing to promoted listing.
    pub async fn demote_listing(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        
        self.post_json(
            &format!("/v2/classifieds/listings/{id}/demote"),
            &Token {
                token,
            },
        ).await
    }
    
    /// Gets limits for batch requests.
    pub async fn get_listing_batch_limit(
        &self,
    ) -> Result<u32, Error> {
        let token = self.get_token()?;
        let batch_limit: api_response::BatchLimit = self.get(
            "/v2/classifieds/listings/batch",
            &Token {
                token,
            },
        ).await?;
        
        Ok(batch_limit.op_limit)
    }
    
    /// Sends a heartbeat.
    pub async fn agent_pulse(
        &self,
    ) -> Result<response::agent::AgentStatus, Error> {
        let token = self.get_token()?;
        let agent_status: response::agent::AgentStatus = self.post(
            "/agent/pulse",
            &Token {
                token,
            },
        ).await?;
        
        Ok(agent_status)
    }
    
    /// Gets current status of user agent.
    pub async fn agent_status(
        &self,
    ) -> Result<response::agent::AgentStatus, Error> {
        let token = self.get_token()?;
        let agent_status: response::agent::AgentStatus = self.post(
            "/agent/status",
            &Token {
                token,
            },
        ).await?;
        
        Ok(agent_status)
    }
    
    /// Stops user agent.
    pub async fn stop_agent(
        &self,
    ) -> Result<(), Error> {
        let token = self.get_token()?;
        
        self.post(
            "/agent/stop",
            &Token {
                token,
            },
        ).await
    }
    
    /// Gets your classifieds limits.
    pub async fn classifieds_limits(
        &self,
    ) -> Result<response::classifieds_limits::ClassifiedsLimits, Error> {
        let token = self.get_token()?;
        let body: api_response::ClassifiedsLimitResponse = self.get(
            "/classifieds/limits",
            &Token {
                token,
            },
        ).await?;
        
        Ok(body.listings)
    }
    
    /// Gets all alerts. This is a convenience method which scrolls against the responses
    /// in [get_alerts](BackpackAPI::get_alerts) until all alerts are obtained. If an error 
    /// occurs, execution will cease and an error will be added to the return value.
    pub async fn get_all_alerts(
        &self,
    ) -> (Vec<response::alert::Alert>, Option<Error>) {
        let mut all = Vec::new();
        let mut limit = MAX_ALERTS_REQUEST_LIMIT as u32;
        let mut skip = 0;
        
        loop {
            match self.get_alerts(skip, limit).await {
                Ok((mut alerts, cursor)) => {
                    all.append(&mut alerts);
                    limit = cursor.limit;
                    skip = cursor.skip + limit;
                    
                    if skip >= cursor.total {
                        // we done
                        break;
                    }
                    
                    sleep(Duration::from_secs(4)).await;
                    continue;
                },
                Err(error) => {
                    if let Some(duration) = helpers::retryable_duration(&error) {
                        sleep(duration).await;
                        continue;
                    }
                    
                    return (all, Some(error))
                },
            }
        }
        
        (all, None)
    }
    
    /// Gets all archived listings. This is a convenience method which scrolls against the 
    /// responses in [get_listings](BackpackAPI::get_archived_listings) until all listings are 
    /// obtained. If an error occurs, execution will cease and an error will be added to the 
    /// return value.
    pub async fn get_all_archived_listings(
        &self,
    ) -> (Vec<response::listing::Listing>, Option<Error>) {
        let mut all = Vec::new();
        let mut limit = MAX_LISTINGS_REQUEST_LIMIT as u32;
        let mut skip = 0;
        
        loop {
            match self.get_archived_listings(skip, limit).await {
                Ok((mut listings, cursor)) => {
                    all.append(&mut listings);
                    limit = cursor.limit;
                    skip = cursor.skip + limit;
                    
                    if skip >= cursor.total {
                        // we done
                        break;
                    }
                    
                    sleep(Duration::from_secs(4)).await;
                    continue;
                },
                Err(error) => {
                    if let Some(duration) = helpers::retryable_duration(&error) {
                        sleep(duration).await;
                        continue;
                    }
                    
                    return (all, Some(error))
                },
            }
        }
        
        (all, None)
    }
    
    /// Gets all listings. This is a convenience method which scrolls against the responses
    /// in [get_listings](BackpackAPI::get_listings) until all listings are obtained. If an 
    /// error occurs, execution will cease and an error will be added to the return value.
    pub async fn get_all_listings(
        &self,
    ) -> (Vec<response::listing::Listing>, Option<Error>) {
        let mut all = Vec::new();
        let mut limit = MAX_LISTINGS_REQUEST_LIMIT as u32;
        let mut skip = 0;
        
        loop {
            match self.get_listings(skip, limit).await {
                Ok((mut listings, cursor)) => {
                    all.append(&mut listings);
                    limit = cursor.limit;
                    skip = cursor.skip + limit;
                    
                    if skip >= cursor.total {
                        // we done
                        break;
                    }
                    
                    sleep(Duration::from_secs(4)).await;
                    continue;
                },
                Err(error) => {
                    if let Some(duration) = helpers::retryable_duration(&error) {
                        sleep(duration).await;
                        continue;
                    }
                    
                    return (all, Some(error))
                },
            }
        }
        
        (all, None)
    }
    
    /// Gets all listings and archived listings. This is a convenience method which combines the 
    /// results from [get_all_listings](BackpackAPI::get_all_listings) and 
    /// [get_all_archived_listings](BackpackAPI::get_all_archived_listings)
    pub async fn get_all_listings_and_archived(
        &self,
    ) -> (Vec<response::listing::Listing>, Option<Error>) {
        let (
            mut listings,
            listings_error,
        ) = self.get_all_listings().await;
        
        if let Some(error) = listings_error {
            return (listings, Some(error));
        }
        
        let (
            mut archived_listings,
            archived_listings_error,
        ) = self.get_all_archived_listings().await;
        
        listings.append(&mut archived_listings);
        (listings, archived_listings_error)
    }
    
    /// Bulk creates any number of listings. This is a convenience method which handles mass 
    /// creation of listings that need to be split into chunks and are rate limited to a certain 
    /// number of requests per minute. If an error occurs, execution will cease and an error will 
    /// be added to the return value. Note that any type can be used for the currencies parameter 
    /// as long as it implements all traits under [`SerializeCurrencies`].
    pub async fn create_listings_chunked<'a, T>(
        &self,
        listings: &'a [request::CreateListing<T>],
    ) -> (Vec<response::listing::create_listing::Result<'a, T>>, Option<Error>)
    where
        T: Serialize
    {
        let mut chunked = helpers::Cooldown::new(listings);
        let mut all = Vec::new();
        
        while let Some((listings, duration)) = chunked.next() {
            match self.create_listings(listings).await {
                Ok(mut more_created) => {
                    all.append(&mut more_created);
                    
                    if let Some(duration) = duration {
                        sleep(duration).await;
                    }
                },
                Err(error) => {
                    if let Some(duration) = helpers::retryable_duration(&error) {
                        sleep(duration).await;
                        chunked.go_back();
                        continue;
                    }
                    
                    return (all, Some(error))
                },
            }
        }
        
        (all, None)
    }
    
    /// Bulk updates any number of listings. This is a convenience method which handles mass 
    /// updating of listings that need to be split into chunks and are rate limited to a certain 
    /// number of requests per minute. If an error occurs, execution will cease and an error will 
    /// be added to the return value. Note that any type can be used for the currencies parameter 
    /// as long as it implements all traits under [`SerializeCurrencies`].
    pub async fn update_listings_chunked<'a, T>(
        &self,
        listings: &'a [request::UpdateListing<T>],
    ) -> (Vec<response::listing::update_listing::Result<'a, T>>, Option<Error>)
    where
        T: Serialize
    {
        let mut chunked = helpers::Cooldown::new(listings);
        let mut all = Vec::new();
        
        while let Some((listings, duration)) = chunked.next() {
            match self.update_listings(listings).await {
                Ok(mut more_updated) => {
                    all.append(&mut more_updated);
                    
                    if let Some(duration) = duration {
                        sleep(duration).await;
                    }
                },
                Err(error) => {
                    if let Some(duration) = helpers::retryable_duration(&error) {
                        sleep(duration).await;
                        chunked.go_back();
                        continue;
                    }
                    
                    return (all, Some(error))
                },
            }
        }
        
        (all, None)
    }
    
    /// Bulk deletes any number of listings. This is a convenience method which handles mass 
    /// deletion of listings that need to be split into chunks and are rate limited to a certain 
    /// number of requests per minute. If an error occurs, execution will cease and an error will 
    /// be added to the return value.
    pub async fn delete_listings_chunked<T>(
        &self,
        listing_ids: &[T],
    ) -> (u32, Option<Error>) 
    where
        T: Borrow<String> + Serialize,
    {
        let mut chunked = helpers::Cooldown::new(listing_ids);
        let mut all = 0;
        
        while let Some((listing_ids, duration)) = chunked.next() {
            match self.delete_listings(listing_ids).await {
                Ok(more_deleted) => {
                    all += more_deleted;
                    
                    if let Some(duration) = duration {
                        sleep(duration).await;
                    }
                },
                Err(error) => {
                    if let Some(duration) = helpers::retryable_duration(&error) {
                        sleep(duration).await;
                        chunked.go_back();
                        continue;
                    }
                    
                    return (all, Some(error))
                },
            }
        }
        
        (all, None)
    }
    
    /// Bulk deletes any number of archived listings. This is a convenience method which handles 
    /// mass deletion of archived listings that need to be split into chunks and are rate
    /// limited to a certain number of requests per minute. If an error occurs, execution will 
    /// cease and an error will be added to the return value.
    pub async fn delete_archived_listings_chunked<T>(
        &self,
        listing_ids: &[T],
    ) -> (u32, Option<Error>)
    where
        T: Borrow<String> + Serialize,
    {
        let mut chunked = helpers::Cooldown::new(listing_ids);
        let mut all = 0;
        
        while let Some((listing_ids, duration)) = chunked.next() {
            match self.delete_archived_listings(listing_ids).await {
                Ok(more_deleted) => {
                    all += more_deleted;
                    
                    if let Some(duration) = duration {
                        sleep(duration).await;
                    }
                },
                Err(error) => {
                    if let Some(duration) = helpers::retryable_duration(&error) {
                        sleep(duration).await;
                        chunked.go_back();
                        continue;
                    }
                    
                    return (all, Some(error))
                },
            }
        }
        
        (all, None)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Token<'a> {
    token: &'a str,
}