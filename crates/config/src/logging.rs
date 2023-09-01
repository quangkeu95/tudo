use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

pub use tracing as __tracing;
pub use tracing::{debug, error, info, instrument, trace, warn};

/// Initializes a tracing Subscriber for logging
#[allow(dead_code)]
pub fn init_tracing_subscriber() {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer())
        .init()
}
