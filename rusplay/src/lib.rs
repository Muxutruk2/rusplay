use reqwest::{Client as HttpClient, Method, Url, cookie::Jar};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;

pub mod models;
use models::*;

/// An asyncronous Rugplay user instance
#[derive(Clone)]
pub struct RugplayClient {
    base_url: String,
    token: String,
    http: HttpClient,
}

/// Errors that the API can generate
#[derive(Debug, Error)]
pub enum RugplayError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to deserialize JSON: {source}\nResponse text: {text}")]
    Deserialize {
        source: serde_json::Error,
        text: String,
    },

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("No cookie provided")]
    NoCookie,
}

pub type Result<T> = std::result::Result<T, RugplayError>;

#[derive(Debug, Error)]
pub enum ClientCreateError {
    #[error("Invalid URL")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Could not build HTTP Client")]
    ReqwestError(#[from] reqwest::Error),
}

impl RugplayClient {
    /// Create a new Client with a token and an optional cookie
    ///
    /// The cookie is necessary for running non-official API calls
    pub fn new(
        token: impl Into<String>,
        cookie: Option<String>,
        url: Option<&str>,
    ) -> std::result::Result<Self, ClientCreateError> {
        let jar = Arc::new(Jar::default());
        let base_url: Url = Url::from_str(url.unwrap_or("https://rugplay.com/api/v1"))?;
        // Insert the auth cookie
        if let Some(cookie) = cookie {
            let cookie_value = cookie;
            jar.add_cookie_str(&cookie_value, &base_url);
        }

        let http = HttpClient::builder().cookie_provider(jar).build()?;

        Ok(Self {
            base_url: url.unwrap_or("https://rugplay.com/api/v1").into(),
            token: token.into(),
            http,
        })
    }

    async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let req = self
            .http
            .request(Method::GET, &url)
            .bearer_auth(&self.token)
            .query(params.unwrap_or_default());

        let resp = req.send().await?.error_for_status()?;
        let text = resp.text().await?;

        match serde_json::from_str(&text) {
            Ok(data) => Ok(data),
            Err(e) => Err(RugplayError::Deserialize { source: e, text }),
        }
    }

    async fn post<T: for<'de> Deserialize<'de>, J: Serialize>(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
        json: Option<J>,
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut req = self
            .http
            .request(Method::POST, &url)
            .bearer_auth(&self.token)
            .query(params.unwrap_or_default());

        if let Some(body) = json {
            req = req.json(&body);
        }

        let resp = req.send().await?.error_for_status()?;

        let text = resp.text().await?;

        match serde_json::from_str(&text) {
            Ok(data) => Ok(data),
            Err(e) => Err(RugplayError::Deserialize { source: e, text }),
        }
    }

    // ---- ENDPOINTS ----
    /// Returns the top 50 coins by market cap.
    pub async fn get_top_coins(&self) -> Result<TopCoinsResponse> {
        self.get("top", None).await
    }

    /// Returns paginated market data with filtering and sorting options.
    ///
    /// ## Query Parameters
    /// - search - Search by coin name or symbol
    /// - sortBy - Sort field: marketCap, currentPrice, change24h, volume24h, createdAt (default: marketCap)
    /// - sortOrder - Sort order: asc, desc (default: desc)
    /// - priceFilter - Price range: all, under1, 1to10, 10to100, over100 (default: all)
    /// - changeFilter - Change filter: all, gainers, losers, hot, wild (default: all)
    /// - page - Page number (default: 1)
    /// - limit - Items per page, max 100 (default: 12)
    pub async fn get_market(&self, params: &[(&str, &str)]) -> Result<MarketResponse> {
        self.get("market", Some(params)).await
    }

    /// Returns detailed information about a specific coin including price history.
    /// symbol - Coin symbol (e.g., "TEST")
    // timeframe - Optional. Chart timeframe: 1m, 5m, 15m, 1h, 4h, 1d (default: 1m)
    pub async fn get_coin_details(
        &self,
        symbol: &str,
        timeframe: Option<&str>,
    ) -> Result<CoinDetailsResponse> {
        let endpoint = format!("coin/{}", symbol);
        let params = timeframe.map(|t| [("timeframe", t)]).unwrap_or_default();
        self.get(
            &endpoint,
            if timeframe.is_some() {
                Some(&params)
            } else {
                None
            },
        )
        .await
    }

    /// Returns the top 50 holders of a specific coin.
    ///
    /// ## Arguments:
    /// - symbol - Coin symbol (e.g., "TEST")
    /// - limit - Number of holders to return, max 200 (default: 50)
    pub async fn get_holders(&self, symbol: &str, limit: Option<u32>) -> Result<HoldersResponse> {
        let endpoint = format!("holders/{}", symbol);
        if let Some(ref limit) = limit {
            let params = &[("limit", &limit.to_string()[..])];
            self.get(&endpoint, Some(params)).await
        } else {
            self.get(&endpoint, None).await
        }
    }

    /// Returns prediction market questions with pagination and filtering options.
    ///
    /// ## Arguments:
    /// - hopium_status - Which types of questions to search
    /// - limit - Number of holders to return, max 100 (default: 50)
    /// - page - Page number (default: 1)
    pub async fn get_hopium(
        &self,
        hopium_status: HopiumFilter,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<HopiumResponse> {
        let params = &[
            ("limit", &limit.unwrap_or(20).to_string()[..]),
            ("status", &hopium_status.to_string()),
            ("page", &page.unwrap_or(1).to_string()),
        ];
        self.get("hopium", Some(params)).await
    }

    /// Returns detailed information about a specific prediction market question including recent bets and probability history.
    ///
    /// ## Arguments
    ///
    /// - question_id - Hopium question to get
    pub async fn get_hopium_details(&self, question_id: u32) -> Result<HopiumDetailsResponse> {
        let endpoint = format!("hopium/{question_id}");

        self.get(&endpoint, None).await
    }

    // ---- Unofficial API ----
    pub async fn get_claim_info(&self) -> Result<ClaimInfo> {
        self.get("../rewards/claim", None).await
    }

    pub async fn claim_reward(&self) -> Result<RewardStatus> {
        self.post::<RewardStatus, ()>("../rewards/claim", None, None)
            .await
    }

    pub async fn trade(
        &self,
        coin: &str,
        trade_type: TradeType,
        amount: u32,
    ) -> Result<TradeResponse> {
        let trade_request = TradeRequest {
            amount,
            r#type: trade_type,
        };

        let endpoint = format!("../coin/{coin}/trade");

        self.post::<TradeResponse, TradeRequest>(&endpoint, None, Some(trade_request))
            .await
    }
}
