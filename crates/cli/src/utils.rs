use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

/// The version message for the current program, like
/// `tudo 0.1.0 (f01b232bc 2022-01-22T23:28:39.493201+00:00)`
pub(crate) const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

/// Initializes a tracing Subscriber for logging
#[allow(dead_code)]
pub fn tracing_subscriber() {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer())
        .init()
}

/// Disables terminal colours if either:
/// - Running windows and the terminal does not support colour codes.
/// - Colour has been disabled by some environment variable.
/// - We are running inside a test
pub fn enable_terminal_colors() {
    let is_windows = cfg!(windows);
    let env_colour_disabled = std::env::var("NO_COLOR").is_ok();
    if is_windows || env_colour_disabled {
        owo_colors::set_override(false);
    }
}
