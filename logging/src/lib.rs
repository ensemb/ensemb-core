#![deny(warnings)]
#![forbid(unsafe_code)]

use std::str::FromStr;

use dome_red_logging_config::prelude::*;

pub mod macros;

pub mod prelude {
    pub use tracing::{debug, error, info, trace, warn};

    pub use super::*;
    pub use super::init_logging;
}

pub fn init_logging(logging_config: &LoggingConfig) {
    // Initialize tracing logger.
    let mut verbose_level = logging_config.env_verbose.clone().unwrap_or_default();
    if logging_config.verbose != 0 {
        verbose_level = logging_config.verbose.to_string();
    }
    let verbose_level = tracing::Level::from_str(&verbose_level).unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(verbose_level)
        .init();

    setup_panics_hook();
}

/// Setup logging of panics to both stdout and logs.
fn setup_panics_hook() {
    let original_hook = std::panic::take_hook();
    let log_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        original_hook(info);
        log_hook(info);
    }));
}
