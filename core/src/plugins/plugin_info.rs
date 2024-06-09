/// Supertrait for common plugin information.
pub trait PluginInfo {
    fn name(&self) -> String;
    fn version() -> &'static str
    where
        Self: Sized;
    fn canonical_name() -> &'static str
    where
        Self: Sized;
}
