use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Initializes the global tracing subscriber with JSON formatting.
pub fn init_logging() {
    let _ = tracing_subscriber::registry()
        .with(fmt::layer().json())
        .with(EnvFilter::from_default_env())
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

}
