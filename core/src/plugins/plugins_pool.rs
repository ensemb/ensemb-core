use std::collections::BTreeMap;
use std::fmt::Debug;

use super::plugin::Plugin;

#[derive(Debug)]
pub struct PluginsPool<T: Plugin + ?Sized>(pub BTreeMap<String, Box<T>>);

impl<T: Plugin + ?Sized> Default for PluginsPool<T> {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

#[allow(clippy::borrowed_box)]
impl<T: Plugin + ?Sized> PluginsPool<T> {
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Box<T>> {
        self.0.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Box<T>> {
        self.0.get_mut(name)
    }

    pub fn insert(&mut self, plugin_name: &str, plugin: Box<T>) {
        self.0.insert(plugin_name.to_string(), plugin);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Box<T>)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Box<T>)> {
        self.0.iter_mut()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
