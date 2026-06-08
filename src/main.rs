pub mod types;
pub mod errors;
pub mod storage;
pub mod api;
pub mod worker;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().json())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("Rustiq: Distributed Task Queue initialized.");
}

