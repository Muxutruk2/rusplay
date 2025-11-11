use anyhow::Context;
use chrono::Utc;
use futures::stream::{self, StreamExt};
use rusplay::{RugplayClient, models::RewardStatus};
use serde::Deserialize;
use std::{fs::File, io::Read, time::Duration};
use tokio::time::sleep;
use tracing::{Instrument, debug, error, info};
use tracing_subscriber::EnvFilter;

#[derive(Deserialize, Debug)]
struct TokensConfig {
    pub tokens: Vec<UserCreds>,
}

#[derive(Deserialize, Debug)]
struct UserCreds {
    pub name: String,
    pub api_key: String,
    pub cookie: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut file = File::open("tokens.toml")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .context("Could not read file")?;

    let tokens = toml::from_str::<TokensConfig>(&contents).context("Could not parse tokens")?;

    stream::iter(tokens.tokens)
        .for_each_concurrent(None, |u| {
            let span = tracing::info_span!("collector", user = %u.name);
            async move {
                info!("Spawning task: {}", u.name);
                let client = RugplayClient::new(u.api_key, u.cookie, None)
                    .expect("Could not create Rugplay Client");
                claim_loop(client).instrument(span).await;
            }
        })
        .await;

    Ok(())
}

fn format_wait_time(ms: u64) -> String {
    let total_seconds = ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;

    match (hours, minutes) {
        (0, 0) => "0m".to_string(),
        (0, m) => format!("{m}m"),
        (h, 0) => format!("{h}h"),
        (h, m) => format!("{h}h {m}m"),
    }
}

async fn try_claim(client: &RugplayClient) -> anyhow::Result<RewardStatus> {
    let claim_info = client
        .get_claim_info()
        .await
        .context("Could not get claim info")?;

    if claim_info.can_claim {
        debug!("Can claim!");
        return client
            .claim_reward()
            .await
            .context("Failed to claim reward");
    }

    debug!(
        "Cannot claim yet. Waiting {}...",
        format_wait_time(claim_info.time_remaining)
    );

    let wait_time = claim_info.time_remaining + 5000;
    sleep(Duration::from_millis(wait_time)).await;

    client
        .claim_reward()
        .await
        .context("Failed to claim after waiting: {e:?}")
}

async fn claim_loop(client: RugplayClient) {
    loop {
        match try_claim(&client).await {
            Ok(r) => {
                debug!(
                    "Successfully claimed reward! Won ${:.2}, New balance: ${:.2}, Login streak: {}",
                    r.reward_amount, r.new_balance, r.login_streak
                );

                let sleep_duration =
                    parse_next_claim_time(&r.next_claim_time).unwrap_or_else(|| {
                        info!("Rugplay did not tell the next claim time; defaulting to ~12h");
                        Duration::from_secs(60 * 60 * 12 + 60)
                    });

                debug!(
                    "Will sleep {}",
                    format_wait_time(sleep_duration.as_millis() as u64)
                );
                sleep(sleep_duration).await;
            }
            Err(e) => {
                error!("Could not claim reward: {e:?}");
                // Optional: backoff on error to avoid hammering server
                sleep(Duration::from_secs(60)).await;
            }
        }
    }
}

fn parse_next_claim_time(next_claim_time: &Option<String>) -> Option<Duration> {
    let next_claim_str = next_claim_time.as_ref()?;
    let next_claim_time = chrono::DateTime::parse_from_rfc3339(next_claim_str).ok()?;
    let next_claim_time = next_claim_time.with_timezone(&Utc);
    let now = Utc::now();
    let wait_duration = next_claim_time.signed_duration_since(now);
    if wait_duration.num_milliseconds() > 0 {
        Some(Duration::from_millis(
            wait_duration.num_milliseconds() as u64
        ))
    } else {
        None
    }
}
