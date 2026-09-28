//! C names that do not survive in the generated bindings.
//!
//! Renamed types and prefix-stripped enumerators only have their Rust name in
//! the output, and the success enumerator of a result enum is not emitted at
//! all. Record them while generating, so that [`crate::doc_links`] can resolve
//! doc links written against the C names.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Debug)]
pub(crate) struct Enumerator {
    /// Name of the C enum, as bindgen reports it.
    pub(crate) c_enum: String,
    /// Name of the Rust constant or variant generated for the enumerator.
    pub(crate) rust_name: String,
    pub(crate) is_zero: bool,
}

#[derive(Debug, Default)]
pub(crate) struct CNames {
    /// C type name -> Rust type name, for types that were renamed.
    pub(crate) type_renames: HashMap<String, String>,
    /// C enumerator name -> every enumerator of that name that bindgen saw.
    pub(crate) enumerators: HashMap<String, Vec<Enumerator>>,
}

pub(crate) static C_NAMES: LazyLock<Mutex<CNames>> = LazyLock::new(Default::default);

pub(crate) fn record_type_rename(c_name: &str, rust_name: &str) {
    // Some callbacks rename types to a path in order to refer to the item in
    // another module. Only the module that defines the item is of interest.
    if rust_name.contains("::") {
        return;
    }
    C_NAMES
        .lock()
        .unwrap()
        .type_renames
        .insert(c_name.to_string(), rust_name.to_string());
}

pub(crate) fn record_enumerator(c_enum: &str, c_name: &str, rust_name: &str, is_zero: bool) {
    let mut c_names = C_NAMES.lock().unwrap();
    let enumerators = c_names.enumerators.entry(c_name.to_string()).or_default();
    if enumerators.iter().any(|e| e.c_enum == c_enum) {
        return;
    }
    enumerators.push(Enumerator {
        c_enum: c_enum.to_string(),
        rust_name: rust_name.to_string(),
        is_zero,
    });
}
