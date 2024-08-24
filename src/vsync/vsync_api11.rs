/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
pub struct OH_NativeVSync {
    _unused: [u8; 0],
}
pub type OH_NativeVSync_FrameCallback = ::core::option::Option<
    unsafe extern "C" fn(timestamp: ::core::ffi::c_longlong, data: *mut ::core::ffi::c_void),
>;
extern "C" {
    /** @brief Creates a <b>NativeVsync</b> instance.\n
    A new <b>NativeVsync</b> instance is created each time this function is called.

    @syscap SystemCapability.Graphic.Graphic2D.NativeVsync
    @param name Indicates the vsync connection name.
    @param length Indicates the name's length.
    @return Returns the pointer to the <b>NativeVsync</b> instance created.
    @since 9
    @version 1.0*/
    pub fn OH_NativeVSync_Create(
        name: *const ::core::ffi::c_char,
        length: ::core::ffi::c_uint,
    ) -> *mut OH_NativeVSync;
    /** @brief Delete the NativeVsync instance.

    @syscap SystemCapability.Graphic.Graphic2D.NativeVsync
    @param window Indicates the pointer to a <b>NativeVsync</b> instance.
    @return Returns int32_t, return value == 0, success, otherwise, failed.
    @since 9
    @version 1.0*/
    pub fn OH_NativeVSync_Destroy(nativeVsync: *mut OH_NativeVSync);
    /** @brief Request next vsync with callback.

    @syscap SystemCapability.Graphic.Graphic2D.NativeVsync
    @param nativeVsync Indicates the pointer to a NativeVsync.
    @param callback Indicates the OH_NativeVSync_FrameCallback which will be called when next vsync coming.
    @param data Indicates data whick will be used in callback.
    @return Returns int32_t, return value == 0, success, otherwise, failed.
    @since 9
    @version 1.0*/
    pub fn OH_NativeVSync_RequestFrame(
        nativeVsync: *mut OH_NativeVSync,
        callback: OH_NativeVSync_FrameCallback,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    /** @brief Get vsync period.

    @syscap SystemCapability.Graphic.Graphic2D.NativeVsync
    @param nativeVsync Indicates the pointer to a NativeVsync.
    @param period Indicates the vsync period.
    @return Returns int32_t, return value == 0, success, otherwise, failed.
    @since 10
    @version 1.0*/
    pub fn OH_NativeVSync_GetPeriod(
        nativeVsync: *mut OH_NativeVSync,
        period: *mut ::core::ffi::c_longlong,
    ) -> ::core::ffi::c_int;
}