//! Provides APIs to manage child processes.
//!
//! SystemCapability: SystemCapability.Ability.AbilityRuntime.Core

#[link(name = "child_process")]
extern "C" {}

mod childprocess_ffi;
pub use childprocess_ffi::*;
