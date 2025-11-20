#![deny(clippy::all)]
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopCoinsResponse {
    pub coins: Vec<CoinSummary>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinSummary {
    pub symbol: String,
    pub name: String,
    pub icon: Option<String>,
    pub price: f64,
    pub change24h: f64,
    pub market_cap: f64,
    pub volume24h: f64,
}

// ---- /market ----
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketResponse {
    pub coins: Vec<MarketCoin>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCoin {
    pub symbol: String,
    pub name: String,
    pub icon: Option<String>,
    pub current_price: f64,
    pub market_cap: f64,
    pub volume24h: f64,
    pub change24h: f64,
    pub created_at: String,
    pub creator_name: Option<String>,
}

// ---- /coin/{symbol} ----
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinDetailsResponse {
    pub coin: CoinDetail,
    pub candlestick_data: Vec<Candle>,
    pub volume_data: Vec<VolumePoint>,
    pub timeframe: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinDetail {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub current_price: f64,
    pub market_cap: f64,
    pub volume24h: f64,
    pub change24h: f64,
    pub circulating_supply: f64,
    pub initial_supply: f64,
    pub creator_name: Option<String>,
    pub creator_username: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub time: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumePoint {
    pub time: u64,
    pub volume: f64,
}

// ---- /holders/{symbol} ----
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldersResponse {
    pub coin_symbol: String,
    pub total_holders: u64,
    pub circulating_supply: f64,
    pub pool_info: PoolInfo,
    pub holders: Vec<Holder>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolInfo {
    pub coin_amount: f64,
    pub base_currency_amount: f64,
    pub current_price: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holder {
    pub rank: u32,
    pub user_id: u64,
    pub username: String,
    pub name: String,
    pub image: String,
    pub quantity: f64,
    pub percentage: f64,
    pub liquidation_value: f64,
}

// ---- /hopium ----
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HopiumResponse {
    pub questions: Vec<HopiumQuestion>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HopiumQuestion {
    pub id: u64,
    pub question: String,
    pub status: HopiumStatus,
    pub resolution_date: Option<String>,
    pub total_amount: f64,
    pub yes_amount: f64,
    pub no_amount: f64,
    pub yes_percentage: f64,
    pub no_percentage: f64,
    pub created_at: String,
    pub creator: HopiumCreator,
}

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub enum HopiumStatus {
    ACTIVE,
    RESOLVED,
    CANCELLED,
}

impl TryFrom<&str> for HopiumFilter {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ACTIVE" => Ok(Self::ACTIVE),
            "RESOLVED" => Ok(Self::RESOLVED),
            "CANCELLED" => Ok(Self::CANCELLED),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
pub enum HopiumFilter {
    ACTIVE,
    RESOLVED,
    CANCELLED,
    ALL,
}

impl Display for HopiumFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HopiumFilter::ACTIVE => write!(f, "ACTIVE"),
            HopiumFilter::RESOLVED => write!(f, "RESOLVED"),
            HopiumFilter::CANCELLED => write!(f, "CANCELLED"),
            HopiumFilter::ALL => write!(f, "ALL"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HopiumCreator {
    pub id: u64,
    pub name: String,
    pub username: String,
    pub image: String,
}

// ---- /hopium/{id} ----
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HopiumDetailsResponse {
    pub question: HopiumQuestionDetails,
    pub probability_history: Vec<ProbabilityPoint>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HopiumQuestionDetails {
    pub id: u64,
    pub question: String,
    pub status: String,
    pub creator: HopiumCreator,
    pub total_amount: f64,
    pub yes_amount: f64,
    pub no_amount: f64,
    pub yes_percentage: f64,
    pub no_percentage: f64,
    pub created_at: String,
    pub recent_bets: Option<Vec<HopiumBet>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HopiumBet {
    pub id: u64,
    pub side: bool,
    pub amount: f64,
    pub created_at: String,
    pub user: HopiumCreator,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbabilityPoint {
    pub time: u64,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimInfoResponse {
    pub base_reward: u32,
    pub can_claim: bool,
    pub last_reward_claim: Option<String>,
    pub login_streak: u32,
    pub next_claim_time: Option<String>,
    pub prestige_bonus: u32,
    pub prestige_level: u32,
    pub reward_amount: u32,
    pub time_remaining: u64,
    pub total_rewards_claimed: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimInfo {
    pub can_claim: bool,
    pub reward_amount: u32,
    pub base_reward: u32,
    pub prestige_bonus: u32,
    pub prestige_level: u32,
    pub time_remaining: u64,
    pub next_claim_time: Option<String>,
    pub total_rewards_claimed: u32,
    pub last_reward_claim: Option<String>,
    pub login_streak: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardStatus {
    pub success: bool,
    pub reward_amount: u32,
    pub base_reward: u32,
    pub prestige_bonus: u32,
    pub prestige_level: u32,
    pub new_balance: f64,
    pub total_rewards_claimed: u32,
    pub login_streak: u32,
    pub next_claim_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CoinTradeType {
    BUY,
    SELL,
}

#[derive(Debug, Serialize)]
pub struct TradeRequest {
    pub amount: u32,
    pub r#type: CoinTradeType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeResponse {
    pub success: bool,
    pub r#type: CoinTradeType,
    pub coins_bought: Option<f64>,
    pub coins_sold: Option<f64>,
    pub total_cost: Option<u32>,
    pub new_price: f64,
    pub price_impact: f64,
    pub new_balance: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeType {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTradeResponse {
    pub trades: Vec<Trade>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub amount: f64,
    pub coin_icon: Option<String>,
    pub coin_name: String,
    pub coin_symbol: String,
    pub price: f64,
    pub timestamp: u64,
    pub total_value: f64,
    pub r#type: TradeType,
    pub user_id: String,
    pub user_image: Option<String>,
    pub username: String,
}
