#![deny(clippy::all)]
use httpmock::prelude::*;
use rusplay::RugplayClient;

#[tokio::test]
async fn test_get_top_coins_mocked() {
    let server = MockServer::start_async().await;

    // Mock the /top endpoint
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/top");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
            {
              "coins": [
                {
                  "symbol": "TEST",
                  "name": "Test",
                  "icon": "coins/test.webp",
                  "price": 76.52377103,
                  "change24h": 7652377003.1039,
                  "marketCap": 76523771031.04,
                  "volume24h": 13744958.18
                }
              ]
            }
            "#,
                );
        })
        .await;

    let client = RugplayClient::new("mock_token", None, Some(&server.base_url()))
        .expect("Failed to create client");

    let result = client.get_top_coins().await.expect("API call failed");
    assert_eq!(result.coins[0].symbol, "TEST");
    mock.assert();
}

#[tokio::test]
async fn test_get_market_mocked() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/market").query_param("limit", "5");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
            {
              "coins": [
                {
                  "symbol": "TEST",
                  "name": "Test",
                  "icon": "coins/test.webp",
                  "currentPrice": 76.52377103,
                  "marketCap": 76523771031.04,
                  "volume24h": 13744958.18,
                  "change24h": 7652377003.1039,
                  "createdAt": "2025-06-24T16:18:51.278Z",
                  "creatorName": "FaceDev"
                }
              ],
              "total": 150,
              "page": 1,
              "limit": 12,
              "totalPages": 13
            }
            "#,
                );
        })
        .await;

    let client = RugplayClient::new("mock_token", None, Some(&server.base_url()))
        .expect("Failed to create client");

    let params = &[("limit", "5")];
    let response = client.get_market(params).await.unwrap();
    assert_eq!(response.coins[0].symbol, "TEST");
    mock.assert();
}

#[tokio::test]
async fn test_get_coin_details_mocked() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/coin/TEST");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
            {
              "coin": {
                "id": 2668,
                "name": "Test",
                "symbol": "TEST",
                "icon": "coins/test.webp",
                "currentPrice": 76.70938996,
                "marketCap": 76709389959.04,
                "volume24h": 13764558.38,
                "change24h": 7670938895.9045,
                "poolCoinAmount": 114176.23963001,
                "poolBaseCurrencyAmount": 8758389.68983547,
                "circulatingSupply": 1000000000,
                "initialSupply": 1000000000,
                "isListed": true,
                "createdAt": "2025-06-24T16:18:51.278Z",
                "creatorId": 1,
                "creatorName": "FaceDev",
                "creatorUsername": "facedev",
                "creatorBio": "the one and only",
                "creatorImage": "avatars/1.jpg"
              },
              "candlestickData": [
                {
                  "time": 1750805760,
                  "open": 74.96948181,
                  "high": 74.96948181,
                  "low": 74.96948181,
                  "close": 74.96948181
                }
              ],
              "volumeData": [
                {
                  "time": 1750805760,
                  "volume": 1234.56
                }
              ],
              "timeframe": "1m"
            }
            "#,
                );
        })
        .await;

    let client = RugplayClient::new("mock_token", None, Some(&server.base_url()))
        .expect("Failed to create client");

    let response = client.get_coin_details("TEST", Some("1m")).await.unwrap();
    assert_eq!(response.coin.symbol, "TEST");
    mock.assert();
}

#[tokio::test]
async fn test_get_holders_mocked() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/holders/TEST");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
            {
              "coinSymbol": "TEST",
              "totalHolders": 50,
              "circulatingSupply": 1000000000,
              "poolInfo": {
                "coinAmount": 114176.23963001,
                "baseCurrencyAmount": 8758389.68983547,
                "currentPrice": 76.70938996
              },
              "holders": [
                {
                  "rank": 1,
                  "userId": 1,
                  "username": "facedev",
                  "name": "FaceDev",
                  "image": "avatars/1.jpg",
                  "quantity": 999883146.4679264,
                  "percentage": 99.98831464679265,
                  "liquidationValue": 4368219.41924125
                }
              ]
            }
            "#,
                );
        })
        .await;

    let client = RugplayClient::new("mock_token", None, Some(&server.base_url()))
        .expect("Failed to create client");

    let response = client.get_holders("TEST", Some(10)).await.unwrap();
    assert_eq!(response.coin_symbol, "TEST");
    assert_eq!(response.holders[0].username, "facedev");
    mock.assert();
}

#[tokio::test]
async fn test_get_hopium_mocked() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/hopium");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
            {
              "questions": [
                {
                  "id": 101,
                  "question": "will elon musk tweet about rugplay?",
                  "status": "ACTIVE",
                  "resolutionDate": "2025-07-25T10:39:19.612Z",
                  "totalAmount": 4007.76,
                  "yesAmount": 3634.65,
                  "noAmount": 373.11,
                  "yesPercentage": 90.69,
                  "noPercentage": 9.31,
                  "createdAt": "2025-06-25T10:39:19.613Z",
                  "resolvedAt": null,
                  "requiresWebSearch": true,
                  "aiResolution": null,
                  "creator": {
                    "id": 3873,
                    "name": "Eliaz",
                    "username": "eluskulus",
                    "image": "avatars/102644133851219200932.png"
                  },
                  "userBets": null
                }
              ],
              "total": 150,
              "page": 1,
              "limit": 20,
              "totalPages": 8
            }
            "#,
                );
        })
        .await;

    let client = RugplayClient::new("mock_token", None, Some(&server.base_url()))
        .expect("Failed to create client");

    use rusplay::models::HopiumFilter;
    let response = client
        .get_hopium(HopiumFilter::ALL, Some(5), Some(1))
        .await
        .unwrap();

    assert_eq!(response.questions[0].id, 101);
    assert_eq!(response.questions[0].creator.username, "eluskulus");
    mock.assert();
}

#[tokio::test]
async fn test_get_hopium_details_mocked() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/hopium/101");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
            {
              "question": {
                "id": 101,
                "question": "will elon musk tweet about rugplay?",
                "status": "ACTIVE",
                "resolutionDate": "2025-07-25T10:39:19.612Z",
                "totalAmount": 4007.76,
                "yesAmount": 3634.65,
                "noAmount": 373.11,
                "yesPercentage": 90.69,
                "noPercentage": 9.31,
                "createdAt": "2025-06-25T10:39:19.613Z",
                "resolvedAt": null,
                "requiresWebSearch": true,
                "aiResolution": null,
                "creator": {
                  "id": 3873,
                  "name": "Eliaz",
                  "username": "eluskulus",
                  "image": "avatars/102644133851219200932.png"
                },
                "userBets": null,
                "recentBets": [
                  {
                    "id": 8066,
                    "side": true,
                    "amount": 3.84,
                    "createdAt": "2025-06-25T14:59:54.201Z",
                    "user": {
                      "id": 5332,
                      "name": "Spam email inhaler",
                      "username": "sunny_tiger7616",
                      "image": "avatars/111376429189149628011.webp"
                    }
                  }
                ]
              },
              "probabilityHistory": [
                { "time": 1750805760, "value": 50.0 },
                { "time": 1750805820, "value": 65.2 }
              ]
            }
            "#,
                );
        })
        .await;

    let client = RugplayClient::new("mock_token", None, Some(&server.base_url()))
        .expect("Failed to create client");

    let response = client.get_hopium_details(101).await.unwrap();
    assert_eq!(response.question.id, 101);
    assert_eq!(response.question.creator.username, "eluskulus");
    assert_eq!(
        response.question.recent_bets.unwrap()[0].user.username,
        "sunny_tiger7616"
    );
    mock.assert();
}
