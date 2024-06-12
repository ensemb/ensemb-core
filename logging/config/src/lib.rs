#![deny(warnings)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub mod prelude {
    pub use super::LoggingConfig;
}

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(long)]
    pub env_verbose: Option<String>,
}
