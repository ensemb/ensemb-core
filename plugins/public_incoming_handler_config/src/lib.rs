#![deny(warnings)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub mod prelude {
    pub use super::PublicIncomingHandlerConfig;
}

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
pub struct PublicIncomingHandlerConfig {}
