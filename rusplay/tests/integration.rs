#![deny(clippy::all)]
mod common;
use std::time::Duration;

use common::test_client;
use rusplay::models::CoinTradeType;

#[tokio::test]
#[ignore]
async fn test_get_top_coins_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let result = client.get_top_coins().await;
    assert!(result.is_ok(), "API call failed: {:#?}", result);
    let response = result.unwrap();
    assert!(!response.coins.is_empty(), "Expected non-empty coin list");
    println!("First coin: {:?}", response.coins[0]);
}

#[tokio::test]
#[ignore]
async fn test_get_market_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let params = &[
        ("limit", "5"),
        ("sortBy", "marketCap"),
        ("sortOrder", "desc"),
    ];
    let result = client.get_market(params).await;
    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    assert!(!response.coins.is_empty(), "Expected non-empty market data");
    println!("Market response sample: {:?}", response.coins.first());
}

#[tokio::test]
#[ignore]
async fn test_get_coin_details_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let result = client.get_coin_details("BTC", Some("1h")).await;
    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    assert_eq!(response.coin.symbol, "BTC", "Expected BTC coin details");
    println!("Coin details: {:?}", response);
}

#[tokio::test]
#[ignore]
async fn test_get_holders_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let result = client.get_holders("BTC", Some(10)).await;
    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    assert!(
        !response.holders.is_empty(),
        "Expected non-empty holder list"
    );
    println!("Top holder: {:?}", response.holders.first());
}

#[tokio::test]
#[ignore]
async fn test_get_hopium_live() {
    use rusplay::models::HopiumFilter;
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let result = client.get_hopium(HopiumFilter::ALL, Some(5), Some(1)).await;
    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    assert!(
        !response.questions.is_empty(),
        "Expected non-empty hopium questions"
    );
    println!("First hopium question: {:?}", response.questions.first());
}

#[tokio::test]
#[ignore]
async fn test_get_hopium_details_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    // Pick a known question ID or get from `get_hopium`
    let question_id = 1;
    let result = client.get_hopium_details(question_id).await;
    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    assert_eq!(
        response.question.id as u32, question_id,
        "Expected matching hopium question ID"
    );
    println!("Hopium question details: {:?}", response);
}

#[tokio::test]
#[ignore]
async fn test_get_claim_info_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let result = client.get_claim_info().await;
    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    println!("Claim info: {:?}", response);
}

#[tokio::test]
#[ignore]
async fn test_claim_reward_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let result = client.claim_reward().await;
    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    println!("Claim reward response: {:?}", response);
}

#[tokio::test]
#[ignore]
async fn test_trade_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");
    let buy = client.trade("BTC", CoinTradeType::BUY, 1).await;
    assert!(buy.is_ok(), "API call failed: {:?}", buy);
    let buy_response = buy.unwrap();
    println!("Buy response: {:?}", buy_response);

    std::thread::sleep(Duration::from_secs(1));

    let sell = client.trade("BTC", CoinTradeType::SELL, 1).await;
    assert!(sell.is_ok(), "API call failed: {:?}", sell);
    let sell_response = sell.unwrap();
    println!("Claim reward response: {:?}", sell_response);
}

#[tokio::test]
#[ignore]
async fn test_recent_trades_live() {
    let client = test_client().expect("Missing credentials: RUGPLAY_COOKIE or RUGPLAY_TOKEN");

    let result = client.get_recent_trades(100).await;

    assert!(result.is_ok(), "API call failed: {:?}", result);
    let response = result.unwrap();
    println!(
        "Got {} trades. First trade: {:?}",
        response.trades.len(),
        response.trades.get(0).unwrap()
    );
}
