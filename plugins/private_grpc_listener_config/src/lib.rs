#![deny(warnings)]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub mod prelude {
    pub use super::PrivateGrpcListenerConfig;
}

#[derive(Debug, Clone, StructOpt, Serialize, Deserialize)]
pub struct PrivateGrpcListenerConfig {
    /// The path to the private protobuf socket.
    /// Can be either the path to Unix socket, e.g. "./pb.sock".
    /// Or address for TCP socket, e.g. "127.0.0.1:3000".
    #[structopt(env = "ORACLE_PRIVATE_GRPC_SOCKET_PATH", short, long)]
    pub private_grpc_socket_path: Option<String>,
}
