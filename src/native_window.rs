/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[link(name = "ace_ndk.z")]
#[link(name = "native_window")]
extern "C" {}

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct BufferHandle {
    ///< buffer fd, -1 if not supported
    pub fd: i32,
    ///< the width of memory
    pub width: i32,
    ///< the stride of memory
    pub stride: i32,
    ///< the height of memory
    pub height: i32,
    pub size: i32,
    ///< the format of memory
    pub format: i32,
    ///< the usage of memory
    pub usage: u64,
    ///< Virtual address of memory
    pub virAddr: *mut ::core::ffi::c_void,
    ///< Shared memory key
    pub key: i32,
    ///< Physical address
    pub phyAddr: u64,
    ///< the number of reserved fd value
    pub reserveFds: u32,
    ///< the number of reserved integer value
    pub reserveInts: u32,
    ///< the data
    pub reserve: __IncompleteArrayField<i32>,
}
/** @brief native window.
@since 8*/
#[repr(C)]
#[derive(Debug)]
pub struct NativeWindow {
    _unused: [u8; 0],
}
/** @brief native window buffer.
@since 8*/
#[repr(C)]
#[derive(Debug)]
pub struct NativeWindowBuffer {
    _unused: [u8; 0],
}
/** @brief define the new type name OHNativeWindow for struct NativeWindow.
@since 8*/
pub type OHNativeWindow = NativeWindow;
/** @brief define the new type name OHNativeWindowBuffer for struct NativeWindowBuffer.
@since 8*/
pub type OHNativeWindowBuffer = NativeWindowBuffer;
/** @brief indicates a dirty region where content is updated.
@since 8*/
#[repr(C)]
#[derive(Debug)]
pub struct Region {
    pub rects: *mut Region_Rect,
    /// if rectNumber is 0, fill the Buffer dirty size by default
    pub rectNumber: i32,
}
/// if rects is nullptr, fill the Buffer dirty size by default
#[repr(C)]
#[derive(Debug)]
pub struct Region_Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}
pub mod NativeWindowOperation {
    /** @brief Indicates the operation code in the function OH_NativeWindow_NativeWindowHandleOpt.
    @since 8*/
    pub type Type = ::core::ffi::c_uint;
    /** set native window buffer geometry,
    variable parameter in function is
    [in] int32_t height, [in] int32_t width*/
    pub const SET_BUFFER_GEOMETRY: Type = 0;
    /** get native window buffer geometry,
    variable parameter in function is
    [out] int32_t *height, [out] int32_t *width*/
    pub const GET_BUFFER_GEOMETRY: Type = 1;
    /** get native window buffer format,
    variable parameter in function is
    [out] int32_t *format*/
    pub const GET_FORMAT: Type = 2;
    /** set native window buffer format,
    variable parameter in function is
    [in] int32_t format*/
    pub const SET_FORMAT: Type = 3;
    /** get native window buffer usage,
    variable parameter in function is
    [out] int32_t *usage.*/
    pub const GET_USAGE: Type = 4;
    /** set native window buffer usage,
    variable parameter in function is
    [in] int32_t usage.*/
    pub const SET_USAGE: Type = 5;
    /** set native window buffer stride,
    variable parameter in function is
    [in] int32_t stride.*/
    pub const SET_STRIDE: Type = 6;
    /** get native window buffer stride,
    variable parameter in function is
    [out] int32_t *stride.*/
    pub const GET_STRIDE: Type = 7;
    /** set native window buffer swap interval,
    variable parameter in function is
    [in] int32_t interval.*/
    pub const SET_SWAP_INTERVAL: Type = 8;
    /** get native window buffer swap interval,
    variable parameter in function is
    [out] int32_t *interval.*/
    pub const GET_SWAP_INTERVAL: Type = 9;
    /** set native window buffer timeout,
    variable parameter in function is
    [in] int32_t timeout.*/
    pub const SET_TIMEOUT: Type = 10;
    /** get native window buffer timeout,
    variable parameter in function is
    [out] int32_t *timeout.*/
    pub const GET_TIMEOUT: Type = 11;
    /** set native window buffer colorGamut,
    variable parameter in function is
    [in] int32_t colorGamut.*/
    pub const SET_COLOR_GAMUT: Type = 12;
    /** get native window buffer colorGamut,
    variable parameter in function is
    [out int32_t *colorGamut].*/
    pub const GET_COLOR_GAMUT: Type = 13;
    /** set native window buffer transform,
    variable parameter in function is
    [in] int32_t transform.*/
    pub const SET_TRANSFORM: Type = 14;
    /** get native window buffer transform,
    variable parameter in function is
    [out] int32_t *transform.*/
    pub const GET_TRANSFORM: Type = 15;
    /** set native window buffer uiTimestamp,
    variable parameter in function is
    [in] uint64_t uiTimestamp.*/
    pub const SET_UI_TIMESTAMP: Type = 16;
}
pub mod OHScalingMode {
    /** @brief Indicates Scaling Mode.
    @since 9
    @deprecated(since = "10")*/
    pub type Type = ::core::ffi::c_uint;
    /** the window content is not updated until a buffer of
    the window size is received*/
    pub const OH_SCALING_MODE_FREEZE: Type = 0;
    /// the buffer is scaled in two dimensions to match the window size
    pub const OH_SCALING_MODE_SCALE_TO_WINDOW: Type = 1;
    /** the buffer is uniformly scaled so that the smaller size of
    the buffer matches the window size*/
    pub const OH_SCALING_MODE_SCALE_CROP: Type = 2;
    /** the window is clipped to the size of the buffer's clipping rectangle
    pixels outside the clipping rectangle are considered fully transparent.*/
    pub const OH_SCALING_MODE_NO_SCALE_CROP: Type = 3;
}
pub mod OHHDRMetadataKey {
    /** @brief Enumerates the HDR metadata keys.
    @since 9
    @deprecated(since = "10")*/
    pub type Type = ::core::ffi::c_uint;
    pub const OH_METAKEY_RED_PRIMARY_X: Type = 0;
    pub const OH_METAKEY_RED_PRIMARY_Y: Type = 1;
    pub const OH_METAKEY_GREEN_PRIMARY_X: Type = 2;
    pub const OH_METAKEY_GREEN_PRIMARY_Y: Type = 3;
    pub const OH_METAKEY_BLUE_PRIMARY_X: Type = 4;
    pub const OH_METAKEY_BLUE_PRIMARY_Y: Type = 5;
    pub const OH_METAKEY_WHITE_PRIMARY_X: Type = 6;
    pub const OH_METAKEY_WHITE_PRIMARY_Y: Type = 7;
    pub const OH_METAKEY_MAX_LUMINANCE: Type = 8;
    pub const OH_METAKEY_MIN_LUMINANCE: Type = 9;
    pub const OH_METAKEY_MAX_CONTENT_LIGHT_LEVEL: Type = 10;
    pub const OH_METAKEY_MAX_FRAME_AVERAGE_LIGHT_LEVEL: Type = 11;
    pub const OH_METAKEY_HDR10_PLUS: Type = 12;
    pub const OH_METAKEY_HDR_VIVID: Type = 13;
}
/** @brief Defines the HDR metadata.
@since 9
@deprecated(since = "10")*/
#[repr(C)]
#[derive(Debug)]
pub struct OHHDRMetaData {
    pub key: OHHDRMetadataKey::Type,
    pub value: f32,
}
/** @brief Defines the ExtData Handle
@since 9
@deprecated(since = "10")*/
#[repr(C)]
#[derive(Debug)]
pub struct OHExtDataHandle {
    pub fd: i32,
    pub reserveInts: u32,
    pub reserve: __IncompleteArrayField<i32>,
}
extern "C" {
    /** @brief Creates a <b>OHNativeWindow</b> instance. A new <b>OHNativeWindow</b> instance is created each time this function is called.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param pSurface Indicates the pointer to a <b>ProduceSurface</b>. The type is a pointer to <b>sptr<OHOS::Surface></b>.
    @return Returns the pointer to the <b>OHNativeWindow</b> instance created.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_CreateNativeWindow(
        pSurface: *mut ::core::ffi::c_void,
    ) -> *mut OHNativeWindow;
}
extern "C" {
    /** @brief Decreases the reference count of a <b>OHNativeWindow</b> instance by 1, and when the reference count reaches 0, destroys the instance.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_DestroyNativeWindow(window: *mut OHNativeWindow);
}
extern "C" {
    /** @brief Creates a <b>OHNativeWindowBuffer</b> instance. A new <b>OHNativeWindowBuffer</b> instance is created each time this function is called.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param pSurfaceBuffer Indicates the pointer to a produce buffer. The type is <b>sptr<OHOS::SurfaceBuffer></b>.
    @return Returns the pointer to the <b>OHNativeWindowBuffer</b> instance created.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_CreateNativeWindowBufferFromSurfaceBuffer(
        pSurfaceBuffer: *mut ::core::ffi::c_void,
    ) -> *mut OHNativeWindowBuffer;
}
extern "C" {
    /** @brief Decreases the reference count of a <b>OHNativeWindowBuffer</b> instance by 1 and, when the reference count reaches 0, destroys the instance.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param buffer Indicates the pointer to a <b>OHNativeWindowBuffer</b> instance.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_DestroyNativeWindowBuffer(buffer: *mut OHNativeWindowBuffer);
}
extern "C" {
    /** @brief Requests a <b>OHNativeWindowBuffer</b> through a <b>OHNativeWindow</b> instance for content production.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param buffer Indicates the double pointer to a <b>OHNativeWindowBuffer</b> instance.
    @param fenceFd Indicates the pointer to a file descriptor handle.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_NativeWindowRequestBuffer(
        window: *mut OHNativeWindow,
        buffer: *mut *mut OHNativeWindowBuffer,
        fenceFd: *mut ::core::ffi::c_int,
    ) -> i32;
}
extern "C" {
    /** @brief Flushes the <b>OHNativeWindowBuffer</b> filled with the content to the buffer queue through a <b>OHNativeWindow</b> instance for content consumption.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param buffer Indicates the pointer to a <b>OHNativeWindowBuffer</b> instance.
    @param fenceFd Indicates a file descriptor handle, which is used for timing synchronization.
    @param region Indicates a dirty region where content is updated.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_NativeWindowFlushBuffer(
        window: *mut OHNativeWindow,
        buffer: *mut OHNativeWindowBuffer,
        fenceFd: ::core::ffi::c_int,
        region: Region,
    ) -> i32;
}
extern "C" {
    /** @brief Returns the <b>OHNativeWindowBuffer</b> to the buffer queue through a <b>OHNativeWindow</b> instance, without filling in any content. The <b>OHNativeWindowBuffer</b> can be used for another request.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param buffer Indicates the pointer to a <b>OHNativeWindowBuffer</b> instance.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_NativeWindowAbortBuffer(
        window: *mut OHNativeWindow,
        buffer: *mut OHNativeWindowBuffer,
    ) -> i32;
}
extern "C" {
    /** @brief Sets or obtains the attributes of a native window, including the width, height, and content format.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param code Indicates the operation code, pointer to <b>NativeWindowOperation</b>.
    @param ... variable parameter, must correspond to code one-to-one.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_NativeWindowHandleOpt(
        window: *mut OHNativeWindow,
        code: ::core::ffi::c_int,
        ...
    ) -> i32;
}
extern "C" {
    /** @brief Obtains the pointer to a <b>BufferHandle</b> of a <b>OHNativeWindowBuffer</b> instance.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param buffer Indicates the pointer to a <b>OHNativeWindowBuffer</b> instance.
    @return Returns the pointer to the <b>BufferHandle</b> instance obtained.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_GetBufferHandleFromNative(
        buffer: *mut OHNativeWindowBuffer,
    ) -> *mut BufferHandle;
}
extern "C" {
    /** @brief Adds the reference count of a native object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param obj Indicates the pointer to a <b>OHNativeWindow</b> or <b>OHNativeWindowBuffer</b> instance.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_NativeObjectReference(obj: *mut ::core::ffi::c_void) -> i32;
}
extern "C" {
    /** @brief Decreases the reference count of a native object and, when the reference count reaches 0, destroys this object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param obj Indicates the pointer to a <b>OHNativeWindow</b> or <b>OHNativeWindowBuffer</b> instance.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_NativeObjectUnreference(obj: *mut ::core::ffi::c_void) -> i32;
}
extern "C" {
    /** @brief Obtains the magic ID of a native object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param obj Indicates the pointer to a <b>OHNativeWindow</b> or <b>OHNativeWindowBuffer</b> instance.
    @return Returns the magic ID, which is unique for each native object.
    @since 8
    @version 1.0*/
    pub fn OH_NativeWindow_GetNativeObjectMagic(obj: *mut ::core::ffi::c_void) -> i32;
}
extern "C" {
    /** @brief Sets scalingMode of a native window.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param sequence Indicates the sequence to a produce buffer.
    @param scalingMode Indicates the enum value to <b>OHScalingMode</b>
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0
    @deprecated(since = "10")*/
    pub fn OH_NativeWindow_NativeWindowSetScalingMode(
        window: *mut OHNativeWindow,
        sequence: u32,
        scalingMode: OHScalingMode::Type,
    ) -> i32;
}
extern "C" {
    /** @brief Sets metaData of a native window.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param sequence Indicates the sequence to a produce buffer.
    @param size Indicates the size of a <b>OHHDRMetaData</b> vector.
    @param metaDate Indicates the pointer to a <b>OHHDRMetaData</b> vector.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0
    @deprecated(since = "10")*/
    pub fn OH_NativeWindow_NativeWindowSetMetaData(
        window: *mut OHNativeWindow,
        sequence: u32,
        size: i32,
        metaData: *const OHHDRMetaData,
    ) -> i32;
}
extern "C" {
    /** @brief Sets metaDataSet of a native window.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param sequence Indicates the sequence to a produce buffer.
    @param key Indicates the enum value to <b>OHHDRMetadataKey</b>
    @param size Indicates the size of a uint8_t vector.
    @param metaDate Indicates the pointer to a uint8_t vector.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0
    @deprecated(since = "10")*/
    pub fn OH_NativeWindow_NativeWindowSetMetaDataSet(
        window: *mut OHNativeWindow,
        sequence: u32,
        key: OHHDRMetadataKey::Type,
        size: i32,
        metaData: *const u8,
    ) -> i32;
}
extern "C" {
    /** @brief Sets tunnel handle of a native window.

    @syscap SystemCapability.Graphic.Graphic2D.NativeWindow
    @param window Indicates the pointer to a <b>OHNativeWindow</b> instance.
    @param handle Indicates the pointer to a <b>OHExtDataHandle</b>.
    @return Returns an error code, 0 is success, otherwise, failed.
    @since 9
    @version 1.0
    @deprecated(since = "10")*/
    pub fn OH_NativeWindow_NativeWindowSetTunnelHandle(
        window: *mut OHNativeWindow,
        handle: *const OHExtDataHandle,
    ) -> i32;
}
