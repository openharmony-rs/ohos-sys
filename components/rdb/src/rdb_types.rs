#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
mod rdb_types_ffi;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub use rdb_types_ffi::*;

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_ColumnType(pub ::core::ffi::c_uint);

pub use ohos_sys_opaque_types::{Data_Asset, OH_Data_Value, OH_Data_Values};
