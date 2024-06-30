use std::sync::{Arc, atomic::AtomicBool};

use serde::Serialize;
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::plugins::prelude::*;
use crate::utils::calculate_current_executable_hash;

pub enum PluginType {
    Workers,
}

#[derive(Default)]
pub struct PluginsContext {
    pub workers: PluginsPool<dyn Worker>,
}

impl PluginsContext {
    /// # Panics
    ///
    /// Will panic if there is not plugin named by `plugin_name`
    #[must_use]
    pub fn get_plugin<P>(&self, plugin_type: &PluginType, plugin_name: Option<&str>) -> &P
    where
        P: Plugin + Worker,
    {
        let plugin_name = plugin_name
            .unwrap_or_else(|| P::canonical_name())
            .to_string();
        let plugin = match plugin_type {
            PluginType::Workers => self.workers.0.get(&plugin_name).unwrap(),
        };
        plugin.downcast_ref::<P>().unwrap()
    }
}

/// Shared state of application.
/// This structure contains all data, which should be shared between plugins.
#[derive(Default, Clone)]
pub struct SharedState {
    pub name: Arc<String>,
    pub version: Arc<String>,
    pub current_executable_hash: Arc<String>,
    pub config: Arc<serde_json::Value>,
    pub shutdown_requested: Arc<AtomicBool>,
    pub plugins: Arc<RwLock<PluginsContext>>,
}

impl SharedState {
    #[must_use]
    pub fn from_config<C>(name: &str, version: &str, config: C) -> Self
    where
        C: Serialize,
    {
        let current_executable_hash = if cfg!(not(debug_assertions)) {
            calculate_current_executable_hash()
        } else {
            String::default()
        };

        Self {
            name: Arc::new(name.to_string()),
            version: Arc::new(version.to_string()),
            current_executable_hash: Arc::new(current_executable_hash.clone()),
            config: Arc::new(serde_json::to_value(config).unwrap()),
            shutdown_requested: Arc::new(AtomicBool::new(false)),
            plugins: Arc::new(RwLock::new(PluginsContext::default())),
        }
    }

    /// # Panics
    ///
    /// Will panic if there is not plugin named by `plugin_name`
    pub async fn set_worker_config<C>(&self, plugin_name: &str, config: C)
    where
        C: Serialize,
    {
        let mut plugins_collection = self.plugins.write().await;
        let plugin = plugins_collection.workers.0.get_mut(plugin_name).unwrap();
        plugin
            .set_config(serde_json::to_value(config).unwrap())
            .await;
    }

    pub async fn plugins_collection(&self) -> RwLockReadGuard<'_, PluginsContext> {
        self.plugins.read().await
    }
}
