use std::any::Any;

use crate::shared_state::SharedState;
use structopt::StructOpt;

use super::plugin_info::PluginInfo;

/// Trait, which describe basic plugin interface.
pub trait Plugin: PluginInfo + Any + Send + Sync {
    fn new<C>(_name: Option<&str>, _shared_state: SharedState<C>) -> Self
    where
        Self: Sized, C: StructOpt + Clone + Send +  Sync;
}
