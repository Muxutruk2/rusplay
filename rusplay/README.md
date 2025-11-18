# Rusplay Library

🚀 Memory safe, blazingly fast, asynchronous Rugplay library written in Rust™

## Usage

To use the library in a Rust project, run `cargo add rusplay`

You can read the documentation here:

- [Docs](https://docs.rs/rusplay/latest/rusplay/)

Example of a simple program


```rust
use rusplay::{RugplayClient, RugplayError, models::TopCoinsResponse};

#[tokio::main]
async fn main() -> Result<(), RugplayError> {
    let client = RugplayClient::new(
        "rgpl_...",
        None,
    );

    let top_coins: TopCoinsResponse = client.get_top_coins().await?;

    for coin in top_coins.coins {
        println!(
            "{} (*{}) ${} 24h Change: {} Market Cap: {} 24h Volume: {}",
            coin.name, coin.symbol, coin.price, coin.change24h, coin.market_cap, coin.volume24h
        );
    }

    Ok(())
}
```

## Implementation Roadmap

### Official API (intended enpoints)
- [x] Get Top Coins
- [x] Get Market Data
- [x] Get Coin Details
- [x] Get Coin Holders
- [x] Get Prediction Markets (Hopium)
- [x] Get Prediction Market Details

### Unofficial API (non-documented. Requires cookies)
- [x] Claim rewards
- [ ] Recent trades
- [ ] Recent trades (using websockets)
- [ ] Get Coin Comments
- [ ] Portfolio (total/summary)
- [x] Trade
- [ ] Bet on Hopium
- [ ] Get Hopium info

