#![deny(warnings)]
#![forbid(unsafe_code)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub mod prelude {
    pub use super::UnixListenerConfig;
}

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
pub struct UnixListenerConfig {
    /// The path to the unix socket.
    #[structopt(env = "ORACLE_UNIX_SOCKET_PATH", short, long, parse(from_os_str))]
    pub unix_socket_path: Option<PathBuf>,
}
