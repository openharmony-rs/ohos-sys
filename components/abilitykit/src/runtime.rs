#[link(name = "ability_runtime")]
extern "C" {}

pub mod application_context;
mod common;
mod context_constant;

pub use common::*;
pub use context_constant::*;

#[cfg(feature = "api-26")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-26")))]
pub mod connect_options;

#[cfg(feature = "api-24")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-24")))]
pub mod context;

#[cfg(feature = "api-24")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-24")))]
pub mod extension_ability;

#[cfg(feature = "api-26")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-26")))]
pub mod modular_object_dispatcher;

#[cfg(feature = "api-26")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-26")))]
pub mod modular_object_extension_ability;

#[cfg(feature = "api-26")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-26")))]
pub mod modular_object_extension_context;

#[cfg(feature = "api-26")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-26")))]
pub mod modular_object_extension_manager;

#[cfg(feature = "api-26")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-26")))]
pub mod native_ability_wrapper;

#[cfg(feature = "api-17")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
pub mod start_options;
