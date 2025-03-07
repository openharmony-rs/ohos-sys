// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte =
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl HiTraceId_Valid {
    /// Invalid <b>HiTraceId</b> instance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_ID_INVALID: HiTraceId_Valid = HiTraceId_Valid(0);
    /// Valid <b>HiTraceId</b> instance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_ID_VALID: HiTraceId_Valid = HiTraceId_Valid(1);
}
#[repr(transparent)]
/// Defines whether a <b>HiTraceId</b> instance is valid.
///
///
/// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTraceId_Valid(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl HiTrace_Version {
    /// Version 1.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_VER_1: HiTrace_Version = HiTrace_Version(0);
}
#[repr(transparent)]
/// Enumerates the HiTrace version numbers.
///
///
/// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTrace_Version(pub ::core::ffi::c_uint);
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl ::core::ops::BitOr<HiTrace_Flag> for HiTrace_Flag {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        HiTrace_Flag(self.0 | other.0)
    }
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl ::core::ops::BitOrAssign for HiTrace_Flag {
    #[inline]
    fn bitor_assign(&mut self, rhs: HiTrace_Flag) {
        self.0 |= rhs.0;
    }
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl ::core::ops::BitAnd<HiTrace_Flag> for HiTrace_Flag {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        HiTrace_Flag(self.0 & other.0)
    }
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl ::core::ops::BitAndAssign for HiTrace_Flag {
    #[inline]
    fn bitand_assign(&mut self, rhs: HiTrace_Flag) {
        self.0 &= rhs.0;
    }
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl HiTrace_Flag {
    /// Default flag.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_DEFAULT: HiTrace_Flag = HiTrace_Flag(0);
    /// Both synchronous and asynchronous calls are traced. By default, only synchronous calls are traced.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_INCLUDE_ASYNC: HiTrace_Flag = HiTrace_Flag(1);
    /// No spans are created. By default, spans are created.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_DONOT_CREATE_SPAN: HiTrace_Flag = HiTrace_Flag(2);
    /// Trace points are automatically added to spans. By default, no trace point is added.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_TP_INFO: HiTrace_Flag = HiTrace_Flag(4);
    /// Information about the start and end of the trace task is not printed. By default, information about the
    /// start and end of the trace task is printed.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_NO_BE_INFO: HiTrace_Flag = HiTrace_Flag(8);
    /// The ID is not added to the log. By default, the ID is added to the log.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_DONOT_ENABLE_LOG: HiTrace_Flag = HiTrace_Flag(16);
    /// Tracing is triggered by faults.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_FAULT_TRIGGER: HiTrace_Flag = HiTrace_Flag(32);
    /// Trace points are added only for call chain trace between devices.
    /// By default, device-to-device trace points are not added.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_FLAG_D2D_TP_INFO: HiTrace_Flag = HiTrace_Flag(64);
}
#[repr(transparent)]
/// Enumerates the HiTrace flags.
///
///
/// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTrace_Flag(pub ::core::ffi::c_uint);
/// Enumerates the HiTrace trace point types.
///
///
/// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
pub enum HiTrace_Tracepoint_Type {
    /// CS trace point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    HITRACE_TP_CS = 0,
    /// CR trace point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    HITRACE_TP_CR = 1,
    /// SS trace point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    HITRACE_TP_SS = 2,
    /// SR trace point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    HITRACE_TP_SR = 3,
    /// General trace point.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    HITRACE_TP_GENERAL = 4,
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl HiTrace_Communication_Mode {
    /// Default communication mode.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_CM_DEFAULT: HiTrace_Communication_Mode = HiTrace_Communication_Mode(0);
    /// Inter-thread communication.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_CM_THREAD: HiTrace_Communication_Mode = HiTrace_Communication_Mode(1);
    /// Inter-process communication.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_CM_PROCESS: HiTrace_Communication_Mode = HiTrace_Communication_Mode(2);
    /// Inter-device communication.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub const HITRACE_CM_DEVICE: HiTrace_Communication_Mode = HiTrace_Communication_Mode(3);
}
#[repr(transparent)]
/// Enumerates the HiTrace communication modes.
///
///
/// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTrace_Communication_Mode(pub ::core::ffi::c_uint);
/// Defines a <b>HiTraceId</b> instance.
///
/// HiTraceId
///
///
/// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
///
///
/// Available since API-level: 12
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HiTraceId {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize]>,
}
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
impl HiTraceId {
    #[inline]
    pub fn valid(&self) -> u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_valid(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn valid_raw(this: *const Self) -> u64 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 16usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_valid_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 16usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn ver(&self) -> u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 3u8) as u64) }
    }
    #[inline]
    pub fn set_ver(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn ver_raw(this: *const Self) -> u64 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 16usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                3u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_ver_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 16usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn chainId(&self) -> u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 60u8) as u64) }
    }
    #[inline]
    pub fn set_chainId(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 60u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn chainId_raw(this: *const Self) -> u64 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 16usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                60u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_chainId_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 16usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                60u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn flags(&self) -> u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(64usize, 12u8) as u64) }
    }
    #[inline]
    pub fn set_flags(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(64usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn flags_raw(this: *const Self) -> u64 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 16usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                64usize,
                12u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_flags_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 16usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                64usize,
                12u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn spanId(&self) -> u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(76usize, 26u8) as u64) }
    }
    #[inline]
    pub fn set_spanId(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(76usize, 26u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn spanId_raw(this: *const Self) -> u64 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 16usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                76usize,
                26u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_spanId_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 16usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                76usize,
                26u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn parentSpanId(&self) -> u64 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(102usize, 26u8) as u64) }
    }
    #[inline]
    pub fn set_parentSpanId(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            self._bitfield_1.set(102usize, 26u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn parentSpanId_raw(this: *const Self) -> u64 {
        unsafe {
            ::core::mem::transmute(<__BindgenBitfieldUnit<[u8; 16usize]>>::raw_get(
                ::core::ptr::addr_of!((*this)._bitfield_1),
                102usize,
                26u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_parentSpanId_raw(this: *mut Self, val: u64) {
        unsafe {
            let val: u64 = ::core::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 16usize]>>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                102usize,
                26u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        valid: u64,
        ver: u64,
        chainId: u64,
        flags: u64,
        spanId: u64,
        parentSpanId: u64,
    ) -> __BindgenBitfieldUnit<[u8; 16usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 16usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let valid: u64 = unsafe { ::core::mem::transmute(valid) };
            valid as u64
        });
        __bindgen_bitfield_unit.set(1usize, 3u8, {
            let ver: u64 = unsafe { ::core::mem::transmute(ver) };
            ver as u64
        });
        __bindgen_bitfield_unit.set(4usize, 60u8, {
            let chainId: u64 = unsafe { ::core::mem::transmute(chainId) };
            chainId as u64
        });
        __bindgen_bitfield_unit.set(64usize, 12u8, {
            let flags: u64 = unsafe { ::core::mem::transmute(flags) };
            flags as u64
        });
        __bindgen_bitfield_unit.set(76usize, 26u8, {
            let spanId: u64 = unsafe { ::core::mem::transmute(spanId) };
            spanId as u64
        });
        __bindgen_bitfield_unit.set(102usize, 26u8, {
            let parentSpanId: u64 = unsafe { ::core::mem::transmute(parentSpanId) };
            parentSpanId as u64
        });
        __bindgen_bitfield_unit
    }
}
extern "C" {
    /// Starts tracing of a process.
    ///
    /// This API starts tracing, creates a <b>HiTraceId</b> instance, and sets it to the TLS of the calling thread.
    /// This API works only when it is called for the first time.
    ///
    /// # Arguments
    ///
    /// * `name` - Pointer to a process name.
    ///
    /// * `flags` - Trace flag.
    ///
    /// # Returns
    ///
    /// * Returns the created <b>HiTraceId</b> instance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_BeginChain(
        name: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> HiTraceId;
    /// Ends tracing and clears the <b>HiTraceId</b> instance of the calling thread from the TLS.
    ///
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_EndChain();
    /// Obtains the trace ID of the calling thread from the TLS.
    ///
    ///
    ///
    /// # Returns
    ///
    /// * Returns the trace ID of the calling thread. If the calling thread does not have a trace ID,
    /// an invalid trace ID is returned.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_GetId() -> HiTraceId;
    /// Sets the trace ID of the calling thread. If the ID is invalid, no operation is performed.
    ///
    /// This API sets a <b>HiTraceId</b> instance to the TLS of the calling thread.
    ///
    /// # Arguments
    ///
    /// * `id` - Trace ID to set.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_SetId(id: *const HiTraceId);
    /// Clears the trace ID of the calling thread and invalidates it.
    ///
    /// This API clears the <b>HiTraceId</b> instance in the TLS of the calling thread.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_ClearId();
    /// Creates a span ID based on the trace ID of the calling thread.
    ///
    /// This API generates a new span and corresponding <b>HiTraceId</b> instance based on the <b>HiTraceId</b>
    /// instance in the TLS of the calling thread.
    ///
    ///
    /// # Returns
    ///
    /// * Returns a valid span ID. If span creation is not allowed, the ID of the calling thread is traced.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_CreateSpan() -> HiTraceId;
    /// Prints HiTrace information, including the trace ID.
    ///
    /// This API prints trace point information, including the communication mode, trace point type, timestamp, and span.
    ///
    /// # Arguments
    ///
    /// * `mode` - Communication mode for the trace point.
    ///
    /// * `type` - Trace point type.
    ///
    /// * `id` - Trace ID.
    ///
    /// * `fmt` - Custom information to print.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_Tracepoint(
        mode: HiTrace_Communication_Mode,
        type_: HiTrace_Tracepoint_Type,
        id: *const HiTraceId,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    /// Initializes a <b>HiTraceId</b> structure.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the <b>HiTraceId</b> structure to be initialized.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_InitId(id: *mut HiTraceId);
    /// Creates a <b>HiTraceId</b> structure based on a byte array.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the <b>HiTraceId</b> structure to be created.
    ///
    /// * `pIdArray` - Byte array.
    ///
    /// * `len` - Length of the byte array.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_IdFromBytes(id: *mut HiTraceId, pIdArray: *const u8, len: ::core::ffi::c_int);
    /// Checks whether a <b>HiTraceId</b> instance is valid.
    ///
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance to check.
    ///
    /// # Returns
    ///
    /// * Returns <b>true</b> if the <b>HiTraceId</b> instance is valid; returns <b>false</b> otherwise.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_IsIdValid(id: *const HiTraceId) -> bool;
    /// Checks whether the specified trace flag in a <b>HiTraceId</b> instance is enabled.
    ///
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance to check.
    ///
    /// * `flag` - Specified trace flag.
    ///
    /// # Returns
    ///
    /// * Returns <b>true</b> if the specified trace flag is enabled; returns <b>false</b> otherwise.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_IsFlagEnabled(id: *const HiTraceId, flag: HiTrace_Flag) -> bool;
    /// Enables the specified trace flag in a <b>HiTraceId</b> instance.
    ///
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance for which you want to enable the specified trace flag.
    ///
    /// * `flag` - Specified trace flag.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_EnableFlag(id: *const HiTraceId, flag: HiTrace_Flag);
    /// Obtains the trace flag set in a <b>HiTraceId</b> instance.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance.
    ///
    ///
    /// # Returns
    ///
    /// * Returns the trace flag set in the specified <b>HiTraceId</b> instance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_GetFlags(id: *const HiTraceId) -> ::core::ffi::c_int;
    /// Sets the trace flag for a <b>HiTraceId</b> instance.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance.
    ///
    /// * `flags` - Trace flag to set.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_SetFlags(id: *mut HiTraceId, flags: ::core::ffi::c_int);
    /// Obtains the trace chain ID.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance for which you want to obtain the trace chain ID.
    ///
    ///
    /// # Returns
    ///
    /// * Returns the trace chain ID of the specified <b>HiTraceId</b> instance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_GetChainId(id: *const HiTraceId) -> u64;
    /// Sets the trace chain ID to a <b>HiTraceId</b> instance
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance.
    ///
    /// * `chainId` - Trace chain ID to set.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_SetChainId(id: *mut HiTraceId, chainId: u64);
    /// Obtains the span ID in a <b>HiTraceId</b> instance.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance for which you want to obtain the span ID.
    ///
    ///
    /// # Returns
    ///
    /// * Returns the span ID in the specified <b>HiTraceId</b> instance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_GetSpanId(id: *const HiTraceId) -> u64;
    /// Sets the span ID in a <b>HiTraceId</b> instance.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance for which you want to set the span ID.
    ///
    /// * `spanId` - Span ID to set.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_SetSpanId(id: *mut HiTraceId, spanId: u64);
    /// Obtains the parent span ID in a <b>HiTraceId</b> instance.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance for which you want to obtain the parent span ID.
    ///
    ///
    /// # Returns
    ///
    /// * Returns the parent span ID in the specified <b>HiTraceId</b> instance.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_GetParentSpanId(id: *const HiTraceId) -> u64;
    /// Sets the parent span ID in a <b>HiTraceId</b> instance.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance for which you want to set the parent span ID.
    ///
    /// * `parentSpanId` - Parent span ID to set.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_SetParentSpanId(id: *mut HiTraceId, parentSpanId: u64);
    /// Converts a <b>HiTraceId</b> instance into a byte array for caching or communication.
    ///
    /// # Arguments
    ///
    /// * `id` - <b>HiTraceId</b> instance to be converted.
    ///
    /// * `pIdArray` - Byte array.
    ///
    /// * `len` - Length of the byte array.
    ///
    ///
    /// # Returns
    ///
    /// * Returns the length of the byte array after conversion.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    ///
    /// Available since API-level: 12
    #[cfg(feature = "api-12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
    pub fn OH_HiTrace_IdToBytes(
        id: *const HiTraceId,
        pIdArray: *mut u8,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    /// Marks the start of a synchronous trace task.
    ///
    /// The <b>OH_HiTrace_StartTrace</b> and <b>OH_HiTrace_FinishTrace</b> APIs must be used in pairs.
    /// The two APIs can be used in nested mode. The stack data structure is used for matching during trace data parsing.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of a trace task.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    /// Available since API-level: 10
    pub fn OH_HiTrace_StartTrace(name: *const ::core::ffi::c_char);
    /// Marks the end of a synchronous trace task.
    ///
    /// This API must be used with <b>OH_HiTrace_StartTrace</b> in pairs. During trace data parsing, the system matches
    /// it with the <b>OH_HiTrace_StartTrace</b> API recently invoked in the service process.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    /// Available since API-level: 10
    pub fn OH_HiTrace_FinishTrace();
    /// Marks the start of an asynchronous trace task.
    ///
    /// This API is called to implement performance trace in asynchronous manner. The start and end of an asynchronous
    /// trace task do not occur in sequence. Therefore, a unique <b>taskId</b> is required to ensure proper data parsing.
    /// It is passed as an input parameter for the asynchronous API.
    /// This API is used with <b>OH_HiTrace_FinishAsyncTrace</b> in pairs. The two APIs that have the same name and
    /// task ID together form an asynchronous timeslice trace task.
    /// If multiple trace tasks with the same name need to be performed at the same time or a trace task needs to be
    /// performed multiple times concurrently, different task IDs must be specified in <b>OH_HiTrace_StartTrace</b>.
    /// If the trace tasks with the same name are not performed at the same time, the same taskId can be used.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the asynchronous trace task.
    ///
    /// * `taskId` - ID of the asynchronous trace task. The start and end of an asynchronous trace task do not occur in
    /// sequence. Therefore, the start and end of an asynchronous trace need to be matched based on the task name and the
    /// unique task ID together.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    /// Available since API-level: 10
    pub fn OH_HiTrace_StartAsyncTrace(name: *const ::core::ffi::c_char, taskId: i32);
    /// Marks the end of an asynchronous trace task.
    ///
    /// This API is called in the callback function after an asynchronous trace is complete.
    /// It is used with <b>OH_HiTrace_StartAsyncTrace</b> in pairs. Its name and task ID must be the same as those of
    /// <b>OH_HiTrace_StartAsyncTrace</b>.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the asynchronous trace task.
    ///
    /// * `taskId` - ID of the asynchronous trace task. The start and end of an asynchronous trace task do not occur in
    /// sequence. Therefore, the start and end of an asynchronous trace need to be matched based on the task name and the
    /// unique task ID together.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    /// Available since API-level: 10
    pub fn OH_HiTrace_FinishAsyncTrace(name: *const ::core::ffi::c_char, taskId: i32);
    /// Traces the value change of an integer variable based on its name.
    ///
    /// This API can be executed for multiple times to trace the value change of a given integer variable at different
    /// time points.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the integer variable. It does not need to be the same as the real variable name.
    ///
    /// * `count` - Integer value. Generally, an integer variable can be passed.
    ///
    ///
    /// Required System Capabilities: SystemCapability.HiviewDFX.HiTrace
    ///
    /// Available since API-level: 10
    pub fn OH_HiTrace_CountTrace(name: *const ::core::ffi::c_char, count: i64);
}
