#![deny(warnings)]
#![deny(unused_crate_dependencies)]
#![forbid(unsafe_code)]

pub mod prelude {
    pub use super::errors::*;
    pub use super::plugins::prelude::*;
    pub use super::shared_state::*;
    pub use super::utils::*;
}

mod errors;
mod plugins;
mod shared_state;
mod utils;
