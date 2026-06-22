pub mod api;
pub mod errors;
pub mod logging;
pub mod storage;
pub mod types;
pub mod worker;

fn main() {
    logging::init_logging();

    tracing::info!("Rustiq: Distributed Task Queue initialized.");
}
