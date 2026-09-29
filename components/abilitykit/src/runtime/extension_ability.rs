//! Native extension abilities.
//!
//! An application implementing a native extension ability must export a function named
//! `OH_AbilityRuntime_OnNativeExtensionCreate` with the signature of
//! [`AbilityRuntime_Extension_CreateFunc`], which the system calls to instantiate it.

mod extension_ability_ffi;
pub use extension_ability_ffi::*;
