//! Bindings to the OpenHarmony relational database APIs
//!
//! See also the [official OpenHarmony RDB documentation][rdb-docs] and the
//! [C API documentation][rdb-capi-docs].
//!
//! [rdb-docs]: https://docs.openharmony.cn/pages/v6.0/en/application-dev/database/native-relational-store-guidelines.md
//! [rdb-capi-docs]: https://docs.openharmony.cn/pages/v6.0/en/application-dev/reference/apis-arkdata/capi-rdb.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[link(name = "native_rdb_ndk.z")]
extern "C" {}

pub mod cursor;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub mod data_asset;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod data_value;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod data_values;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub mod data_values_buckets;
pub mod predicates;
pub mod rdb_crypto_param;
pub mod rdb_transaction;
pub mod rdb_types;
pub mod relational_store;
pub mod relational_store_error_code;
pub mod value_object;
pub mod values_bucket;
