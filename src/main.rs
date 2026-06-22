pub mod api;
pub mod errors;
pub mod logging;
pub mod storage;
pub mod types;
pub mod worker;

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().json())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("Rustiq: Distributed Task Queue initialized.");
}
