// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
impl OH_Drawing_BlurType {
    /// Fuzzy inside and outside.
    pub const NORMAL: OH_Drawing_BlurType = OH_Drawing_BlurType(0);
    /// Solid inside, fuzzy outside.
    pub const SOLID: OH_Drawing_BlurType = OH_Drawing_BlurType(1);
    /// Nothing inside, fuzzy outside.
    pub const OUTER: OH_Drawing_BlurType = OH_Drawing_BlurType(2);
    /// Fuzzy inside, nothing outside.
    pub const INNER: OH_Drawing_BlurType = OH_Drawing_BlurType(3);
}
#[repr(transparent)]
/// Enumerates blur type.
///
///
/// Available since API-level: 11
///
/// Version: 1.0
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_BlurType(pub ::core::ffi::c_uint);
extern "C" {
    /// Creates an <b>OH_Drawing_MaskFilter</b> with a blur effect.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `blurType` - Indicates the blur type.
    ///
    /// * `sigma` - Indicates the standard deviation of the Gaussian blur to apply. Must be > 0.
    ///
    /// * `respectCTM` - Indicates the blur's sigma is modified by the CTM, default is true.
    ///
    /// # Returns
    ///
    /// * Returns the pointer to the <b>OH_Drawing_MaskFilter</b> object created.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_MaskFilterCreateBlur(
        blurType: OH_Drawing_BlurType,
        sigma: f32,
        respectCTM: bool,
    ) -> *mut OH_Drawing_MaskFilter;
    /// Destroys an <b>OH_Drawing_MaskFilter</b> object and reclaims the memory occupied by the object.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `maskFilter` - Indicates the pointer to an <b>OH_Drawing_MaskFilter</b> object.
    ///
    /// Available since API-level: 11
    ///
    /// Version: 1.0
    #[cfg(feature = "api-11")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
    pub fn OH_Drawing_MaskFilterDestroy(maskFilter: *mut OH_Drawing_MaskFilter);
}
