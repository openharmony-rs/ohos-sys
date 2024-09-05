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
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),);
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
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),);
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
}
impl HiTraceId_Valid {
    /** @brief Invalid <b>HiTraceId</b> instance.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_ID_INVALID: HiTraceId_Valid = HiTraceId_Valid(0);
}
impl HiTraceId_Valid {
    /** @brief Valid <b>HiTraceId</b> instance.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_ID_VALID: HiTraceId_Valid = HiTraceId_Valid(1);
}
#[repr(transparent)]
/** @brief Defines whether a <b>HiTraceId</b> instance is valid.

@syscap SystemCapability.HiviewDFX.HiTrace

@since 12*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTraceId_Valid(pub ::core::ffi::c_uint);
impl HiTrace_Version {
    /** @brief Version 1.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_VER_1: HiTrace_Version = HiTrace_Version(0);
}
#[repr(transparent)]
/** @brief Enumerates the HiTrace version numbers.

@syscap SystemCapability.HiviewDFX.HiTrace

@since 12*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTrace_Version(pub ::core::ffi::c_uint);
impl HiTrace_Flag {
    /** @brief Default flag.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_DEFAULT: HiTrace_Flag = HiTrace_Flag(0);
}
impl HiTrace_Flag {
    /** @brief Both synchronous and asynchronous calls are traced. By default, only synchronous calls are traced.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_INCLUDE_ASYNC: HiTrace_Flag = HiTrace_Flag(1);
}
impl HiTrace_Flag {
    /** @brief No spans are created. By default, spans are created.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_DONOT_CREATE_SPAN: HiTrace_Flag = HiTrace_Flag(2);
}
impl HiTrace_Flag {
    /** @brief Trace points are automatically added to spans. By default, no trace point is added.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_TP_INFO: HiTrace_Flag = HiTrace_Flag(4);
}
impl HiTrace_Flag {
    /** @brief Information about the start and end of the trace task is not printed. By default, information about the
    start and end of the trace task is printed.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_NO_BE_INFO: HiTrace_Flag = HiTrace_Flag(8);
}
impl HiTrace_Flag {
    /** @brief The ID is not added to the log. By default, the ID is added to the log.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_DONOT_ENABLE_LOG: HiTrace_Flag = HiTrace_Flag(16);
}
impl HiTrace_Flag {
    /** @brief Tracing is triggered by faults.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_FAULT_TRIGGER: HiTrace_Flag = HiTrace_Flag(32);
}
impl HiTrace_Flag {
    /** @brief Trace points are added only for call chain trace between devices.
    By default, device-to-device trace points are not added.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_FLAG_D2D_TP_INFO: HiTrace_Flag = HiTrace_Flag(64);
}
impl ::core::ops::BitOr<HiTrace_Flag> for HiTrace_Flag {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        HiTrace_Flag(self.0 | other.0)
    }
}
impl ::core::ops::BitOrAssign for HiTrace_Flag {
    #[inline]
    fn bitor_assign(&mut self, rhs: HiTrace_Flag) {
        self.0 |= rhs.0;
    }
}
impl ::core::ops::BitAnd<HiTrace_Flag> for HiTrace_Flag {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        HiTrace_Flag(self.0 & other.0)
    }
}
impl ::core::ops::BitAndAssign for HiTrace_Flag {
    #[inline]
    fn bitand_assign(&mut self, rhs: HiTrace_Flag) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
/** @brief Enumerates the HiTrace flags.

@syscap SystemCapability.HiviewDFX.HiTrace

@since 12*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTrace_Flag(pub ::core::ffi::c_uint);
#[repr(u32)]
/** @brief Enumerates the HiTrace trace point types.

@syscap SystemCapability.HiviewDFX.HiTrace

@since 12*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HiTrace_Tracepoint_Type {
    /** @brief CS trace point.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    HITRACE_TP_CS = 0,
    /** @brief CR trace point.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    HITRACE_TP_CR = 1,
    /** @brief SS trace point.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    HITRACE_TP_SS = 2,
    /** @brief SR trace point.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    HITRACE_TP_SR = 3,
    /** @brief General trace point.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    HITRACE_TP_GENERAL = 4,
}
impl HiTrace_Communication_Mode {
    /** @brief Default communication mode.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_CM_DEFAULT: HiTrace_Communication_Mode = HiTrace_Communication_Mode(0);
}
impl HiTrace_Communication_Mode {
    /** @brief Inter-thread communication.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_CM_THREAD: HiTrace_Communication_Mode = HiTrace_Communication_Mode(1);
}
impl HiTrace_Communication_Mode {
    /** @brief Inter-process communication.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_CM_PROCESS: HiTrace_Communication_Mode = HiTrace_Communication_Mode(2);
}
impl HiTrace_Communication_Mode {
    /** @brief Inter-device communication.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub const HITRACE_CM_DEVICE: HiTrace_Communication_Mode = HiTrace_Communication_Mode(3);
}
#[repr(transparent)]
/** @brief Enumerates the HiTrace communication modes.

@syscap SystemCapability.HiviewDFX.HiTrace

@since 12*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HiTrace_Communication_Mode(pub ::core::ffi::c_uint);
/** @brief Defines a <b>HiTraceId</b> instance.

@struct HiTraceId

@syscap SystemCapability.HiviewDFX.HiTrace

@since 12*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HiTraceId {
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 16usize]>,
}
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
    /** @brief Starts tracing of a process.

    This API starts tracing, creates a <b>HiTraceId</b> instance, and sets it to the TLS of the calling thread.
    This API works only when it is called for the first time.

    @param name Pointer to a process name.
    @param flags Trace flag.
    @return Returns the created <b>HiTraceId</b> instance.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_BeginChain(
        name: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> HiTraceId;
    /** @brief Ends tracing and clears the <b>HiTraceId</b> instance of the calling thread from the TLS.


    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_EndChain();
    /** @brief Obtains the trace ID of the calling thread from the TLS.


    @return Returns the trace ID of the calling thread. If the calling thread does not have a trace ID,
    an invalid trace ID is returned.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_GetId() -> HiTraceId;
    /** @brief Sets the trace ID of the calling thread. If the ID is invalid, no operation is performed.

    This API sets a <b>HiTraceId</b> instance to the TLS of the calling thread.

    @param id Trace ID to set.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_SetId(id: *const HiTraceId);
    /** @brief Clears the trace ID of the calling thread and invalidates it.

    This API clears the <b>HiTraceId</b> instance in the TLS of the calling thread.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_ClearId();
    /** @brief Creates a span ID based on the trace ID of the calling thread.

    This API generates a new span and corresponding <b>HiTraceId</b> instance based on the <b>HiTraceId</b>
    instance in the TLS of the calling thread.

    @return Returns a valid span ID. If span creation is not allowed, the ID of the calling thread is traced.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_CreateSpan() -> HiTraceId;
    /** @brief Prints HiTrace information, including the trace ID.

    This API prints trace point information, including the communication mode, trace point type, timestamp, and span.

    @param mode Communication mode for the trace point.
    @param type Trace point type.
    @param id Trace ID.
    @param fmt Custom information to print.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_Tracepoint(
        mode: HiTrace_Communication_Mode,
        type_: HiTrace_Tracepoint_Type,
        id: *const HiTraceId,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    /** @brief Initializes a <b>HiTraceId</b> structure.

    @param id ID of the <b>HiTraceId</b> structure to be initialized.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_InitId(id: *mut HiTraceId);
    /** @brief Creates a <b>HiTraceId</b> structure based on a byte array.

    @param id ID of the <b>HiTraceId</b> structure to be created.
    @param pIdArray Byte array.
    @param len Length of the byte array.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_IdFromBytes(id: *mut HiTraceId, pIdArray: *const u8, len: ::core::ffi::c_int);
    /** @brief Checks whether a <b>HiTraceId</b> instance is valid.


    @param id <b>HiTraceId</b> instance to check.
    @return Returns <b>true</b> if the <b>HiTraceId</b> instance is valid; returns <b>false</b> otherwise.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_IsIdValid(id: *const HiTraceId) -> bool;
    /** @brief Checks whether the specified trace flag in a <b>HiTraceId</b> instance is enabled.


    @param id <b>HiTraceId</b> instance to check.
    @param flag Specified trace flag.
    @return Returns <b>true</b> if the specified trace flag is enabled; returns <b>false</b> otherwise.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_IsFlagEnabled(id: *const HiTraceId, flag: HiTrace_Flag) -> bool;
    /** @brief Enables the specified trace flag in a <b>HiTraceId</b> instance.


    @param id <b>HiTraceId</b> instance for which you want to enable the specified trace flag.
    @param flag Specified trace flag.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_EnableFlag(id: *const HiTraceId, flag: HiTrace_Flag);
    /** @brief Obtains the trace flag set in a <b>HiTraceId</b> instance.

    @param id <b>HiTraceId</b> instance.

    @return Returns the trace flag set in the specified <b>HiTraceId</b> instance.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_GetFlags(id: *const HiTraceId) -> ::core::ffi::c_int;
    /** @brief Sets the trace flag for a <b>HiTraceId</b> instance.

    @param id <b>HiTraceId</b> instance.
    @param flags Trace flag to set.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_SetFlags(id: *mut HiTraceId, flags: ::core::ffi::c_int);
    /** @brief Obtains the trace chain ID.

    @param id <b>HiTraceId</b> instance for which you want to obtain the trace chain ID.

    @return Returns the trace chain ID of the specified <b>HiTraceId</b> instance.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_GetChainId(id: *const HiTraceId) -> u64;
    /** @brief Sets the trace chain ID to a <b>HiTraceId</b> instance

    @param id <b>HiTraceId</b> instance.
    @param chainId Trace chain ID to set.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_SetChainId(id: *mut HiTraceId, chainId: u64);
    /** @brief Obtains the span ID in a <b>HiTraceId</b> instance.

    @param id <b>HiTraceId</b> instance for which you want to obtain the span ID.

    @return Returns the span ID in the specified <b>HiTraceId</b> instance.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_GetSpanId(id: *const HiTraceId) -> u64;
    /** @brief Sets the span ID in a <b>HiTraceId</b> instance.

    @param id <b>HiTraceId</b> instance for which you want to set the span ID.
    @param spanId Span ID to set.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_SetSpanId(id: *mut HiTraceId, spanId: u64);
    /** @brief Obtains the parent span ID in a <b>HiTraceId</b> instance.

    @param id <b>HiTraceId</b> instance for which you want to obtain the parent span ID.

    @return Returns the parent span ID in the specified <b>HiTraceId</b> instance.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_GetParentSpanId(id: *const HiTraceId) -> u64;
    /** @brief Sets the parent span ID in a <b>HiTraceId</b> instance.

    @param id <b>HiTraceId</b> instance for which you want to set the parent span ID.
    @param parentSpanId Parent span ID to set.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_SetParentSpanId(id: *mut HiTraceId, parentSpanId: u64);
    /** @brief Converts a <b>HiTraceId</b> instance into a byte array for caching or communication.

    @param id <b>HiTraceId</b> instance to be converted.
    @param pIdArray Byte array.
    @param len Length of the byte array.

    @return Returns the length of the byte array after conversion.

    @syscap SystemCapability.HiviewDFX.HiTrace

    @since 12*/
    pub fn OH_HiTrace_IdToBytes(
        id: *const HiTraceId,
        pIdArray: *mut u8,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
