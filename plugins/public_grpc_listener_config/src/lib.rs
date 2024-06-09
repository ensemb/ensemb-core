#![deny(warnings)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub mod prelude {
    pub use super::PublicGrpcListenerConfig;
}

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
pub struct PublicGrpcListenerConfig {
    /// The path to the public protobuf socket.
    /// Can be either the path to Unix socket, e.g. "./pb.sock".
    /// Or address for TCP socket, e.g. "127.0.0.1:8080".
    #[structopt(env = "ORACLE_PUBLIC_GRPC_SOCKET_PATH", short, long)]
    pub public_grpc_socket_path: Option<String>,
}
