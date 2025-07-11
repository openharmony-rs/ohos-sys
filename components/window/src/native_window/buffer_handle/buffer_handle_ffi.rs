// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
/// Buffer handle used to transfer and obtain information about the buffer.
///
/// Available since API-level: 8
#[repr(C)]
#[derive(Debug)]
pub struct BufferHandle {
    /// < buffer fd, -1 if not supported
    pub fd: i32,
    /// < the width of memory
    pub width: i32,
    /// < the stride of memory
    pub stride: i32,
    /// < the height of memory
    pub height: i32,
    pub size: i32,
    /// < the format of memory
    pub format: i32,
    /// < the usage of memory
    pub usage: u64,
    /// < Virtual address of memory
    pub virAddr: *mut ::core::ffi::c_void,
    /// < Shared memory key
    pub key: i32,
    /// < Physical address
    pub phyAddr: u64,
    /// < the number of reserved fd value
    pub reserveFds: u32,
    /// < the number of reserved integer value
    pub reserveInts: u32,
    /// < the data
    pub reserve: __IncompleteArrayField<i32>,
}
