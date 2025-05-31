//! Defines the application context APIs.
//!
//! SystemCapability: SystemCapability.Ability.AbilityRuntime.Core

#[link(name = "ability_runtime")]
extern "C" {}

mod application_context_ffi;
pub use application_context_ffi::*;
