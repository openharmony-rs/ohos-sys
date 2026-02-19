#[link(name = "ability_runtime")]
extern "C" {}

pub mod application_context;
mod common;
mod context;

pub use common::*;
pub use context::*;

#[cfg(feature = "api-17")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
pub mod start_options;
