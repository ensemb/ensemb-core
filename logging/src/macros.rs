#[macro_export]
macro_rules! prepare {
    ($fmt:literal) => {{
        format!("{}", $fmt)
    }};
    ($fmt:literal, $($arg:expr),* $(,)?) => {{
        format!("{}", format_args!($fmt, $($arg),*))
    }};
}

#[macro_export]
macro_rules! log_plugin_prefix {
    ($self:ty, $state:expr) => {{
        let name: String = {
            let name = (*$state.name).clone();
            let canonical_name = <$self as PluginInfo>::canonical_name().to_string();
            if name.eq(&canonical_name) {
                name
            } else {
                format!("{} -> {}", &canonical_name, &name)
            }
        };
        format!("{} [{}]", &name, <$self as PluginInfo>::version())
    }};
}

#[macro_export]
macro_rules! p_log {
    ($state:expr, $level:tt, $fmt:literal) => {
        log!($level, "{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt));
    };
    ($self:ty, $state:expr, $level:tt, $fmt:literal) => {
        log!($level, "{} {}", log_plugin_prefix!($self, $state), prepare!($fmt));
    };
    ($state:expr, $level:tt, $fmt:literal, $($arg:expr),* $(,)*) => {
        log!($level, "{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt, $($arg),*));
    };
    ($self:ty, $state:expr, $level:tt, $fmt:literal, $($arg:expr),* $(,)*) => {
        log!($level, "{} {}", log_plugin_prefix!($self, $state), prepare!($fmt, $($arg),*));
    };
}

#[macro_export]
macro_rules! p_error {
    ($state:expr, $fmt:literal) => {
        error!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt));
    };
    ($self:ty, $state:expr, $fmt:literal) => {
        error!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt));
    };
    ($state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        error!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt, $($arg),*));
    };
    ($self:ty, $state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        error!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt, $($arg),*));
    };
}

#[macro_export]
macro_rules! p_warn {
    ($state:expr, $fmt:literal) => {
        warn!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt));
    };
    ($self:ty, $state:expr, $fmt:literal) => {
        warn!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt));
    };
    ($state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        warn!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt, $($arg),*));
    };
    ($self:ty, $state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        warn!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt, $($arg),*));
    };
}

#[macro_export]
macro_rules! p_info {
    ($state:expr, $fmt:literal) => {
        info!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt));
    };
    ($self:ty, $state:expr, $fmt:literal) => {
        info!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt));
    };
    ($state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        info!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt, $($arg),*));
    };
    ($self:ty, $state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        info!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt, $($arg),*));
    };
}

#[macro_export]
macro_rules! p_debug {
    ($state:expr, $fmt:literal) => {
        debug!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt));
    };
    ($self:ty, $state:expr, $fmt:literal) => {
        debug!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt));
    };
    ($state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        debug!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt, $($arg),*));
    };
    ($self:ty, $state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        debug!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt, $($arg),*));
    };
}

#[macro_export]
macro_rules! p_trace {
    ($state:expr, $fmt:literal) => {
        trace!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt));
    };
    ($self:ty, $state:expr, $fmt:literal) => {
        trace!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt));
    };
    ($state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        trace!("{} {}", log_plugin_prefix!(Self, $state), prepare!($fmt, $($arg),*));
    };
    ($self:ty, $state:expr, $fmt:literal, $($arg:expr),* $(,)*) => {
        trace!("{} {}", log_plugin_prefix!($self, $state), prepare!($fmt, $($arg),*));
    };
}
