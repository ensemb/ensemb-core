use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use serde_json::Value;
use tokio::task::JoinHandle;

use crate::errors::Result;

use super::plugin::Plugin;

#[async_trait]
pub trait Worker: Plugin + DowncastSync {
    async fn set_config(&self, _plugin_config: Value);
    async fn entrypoint(&self) -> Result<JoinHandle<Result<()>>>;
}
impl_downcast!(sync Worker);
