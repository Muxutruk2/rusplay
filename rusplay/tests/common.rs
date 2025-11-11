use rusplay::RugplayClient;
use std::sync::Once;
use tracing_subscriber::{EnvFilter, fmt};

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
        fmt()
            .without_time()
            .with_env_filter(env_filter)
            .with_test_writer() // ensures logs appear in `cargo test` output
            .init();
    });
}

pub fn test_client() -> Option<RugplayClient> {
    init_tracing();

    let token = std::env::var("RUGPLAY_TOKEN").ok()?;
    let cookie = std::env::var("RUGPLAY_COOKIE").ok();
    RugplayClient::new(token, cookie, None).ok()
}
