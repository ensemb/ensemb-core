use std::any::Any;

use crate::shared_state::SharedState;

use super::plugin_info::PluginInfo;

/// Trait, which describe basic plugin interface.
pub trait Plugin: PluginInfo + Any + Send {
    fn new(_name: Option<&str>, _shared_state: SharedState) -> Self
    where
        Self: Sized;
}
