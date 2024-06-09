#![deny(warnings)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use structopt::StructOpt;

pub mod prelude {
    pub use super::PrivateRpcListenerConfig;
}

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
pub struct PrivateRpcListenerConfig {
    /// Private API host for RPC Server, for example: "127.0.0.1".
    #[structopt(long = "private-rpc-api-host", env = "PRIVATE_RPC_API_HOST")]
    pub private_rpc_api_host: Option<IpAddr>,

    /// Private API port for RPC Server, for example: "3000".
    #[structopt(long = "private-rpc-api-port", env = "PRIVATE_RPC_API_PORT")]
    pub private_rpc_api_port: Option<u16>,
}
