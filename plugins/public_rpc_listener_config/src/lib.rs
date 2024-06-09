#![deny(warnings)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use structopt::StructOpt;

pub mod prelude {
    pub use super::PublicRpcListenerConfig;
}

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
pub struct PublicRpcListenerConfig {
    /// Public API host for RPC Server, for example: "127.0.0.1".
    #[structopt(long = "public-rpc-api-host", env = "PUBLIC_RPC_API_HOST")]
    pub public_rpc_api_host: Option<IpAddr>,

    /// Public API port for RPC Server, for example: "8080".
    #[structopt(long = "public-rpc-api-port", env = "PUBLIC_RPC_API_PORT")]
    pub public_rpc_api_port: Option<u16>,
}
