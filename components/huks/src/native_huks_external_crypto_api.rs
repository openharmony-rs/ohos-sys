mod native_huks_external_crypto_api_ffi;
pub use native_huks_external_crypto_api_ffi::*;

#[link(name = "huks_external_crypto.z")]
unsafe extern "C" {}
