#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
mod rdb_transaction_ffi;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub use rdb_transaction_ffi::*;
