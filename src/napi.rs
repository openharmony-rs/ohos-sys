/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const NAPI_VERSION: u32 = 8;
pub const NAPI_VERSION_EXPERIMENTAL: u32 = 2147483647;
pub const __BYTE_ORDER: u32 = 1234;
pub const __LONG_MAX: u64 = 9223372036854775807;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __USE_TIME_BITS64: u32 = 1;
pub const NAPI_AUTO_LENGTH: i32 = -1;
pub const NAPI_MODULE_VERSION: u32 = 1;
impl napi_qos_t {
    pub const napi_qos_background: napi_qos_t = napi_qos_t(0);
}
impl napi_qos_t {
    pub const napi_qos_utility: napi_qos_t = napi_qos_t(1);
}
impl napi_qos_t {
    pub const napi_qos_default: napi_qos_t = napi_qos_t(2);
}
impl napi_qos_t {
    pub const napi_qos_user_initiated: napi_qos_t = napi_qos_t(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_qos_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::core::ffi::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
pub type intmax_t = ::core::ffi::c_long;
pub type uintmax_t = ::core::ffi::c_ulong;
pub type char16_t = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_env__ {
    _unused: [u8; 0],
}
pub type napi_env = *mut napi_env__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_value__ {
    _unused: [u8; 0],
}
pub type napi_value = *mut napi_value__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_ref__ {
    _unused: [u8; 0],
}
pub type napi_ref = *mut napi_ref__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_handle_scope__ {
    _unused: [u8; 0],
}
pub type napi_handle_scope = *mut napi_handle_scope__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_escapable_handle_scope__ {
    _unused: [u8; 0],
}
pub type napi_escapable_handle_scope = *mut napi_escapable_handle_scope__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_callback_info__ {
    _unused: [u8; 0],
}
pub type napi_callback_info = *mut napi_callback_info__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_deferred__ {
    _unused: [u8; 0],
}
pub type napi_deferred = *mut napi_deferred__;
impl napi_property_attributes {
    pub const napi_default: napi_property_attributes = napi_property_attributes(0);
}
impl napi_property_attributes {
    pub const napi_writable: napi_property_attributes = napi_property_attributes(1);
}
impl napi_property_attributes {
    pub const napi_enumerable: napi_property_attributes = napi_property_attributes(2);
}
impl napi_property_attributes {
    pub const napi_configurable: napi_property_attributes = napi_property_attributes(4);
}
impl napi_property_attributes {
    pub const napi_static: napi_property_attributes = napi_property_attributes(1024);
}
impl napi_property_attributes {
    pub const napi_default_method: napi_property_attributes = napi_property_attributes(
        5,
    );
}
impl napi_property_attributes {
    pub const napi_default_jsproperty: napi_property_attributes = napi_property_attributes(
        7,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_property_attributes(pub ::core::ffi::c_uint);
impl napi_valuetype {
    pub const napi_undefined: napi_valuetype = napi_valuetype(0);
}
impl napi_valuetype {
    pub const napi_null: napi_valuetype = napi_valuetype(1);
}
impl napi_valuetype {
    pub const napi_boolean: napi_valuetype = napi_valuetype(2);
}
impl napi_valuetype {
    pub const napi_number: napi_valuetype = napi_valuetype(3);
}
impl napi_valuetype {
    pub const napi_string: napi_valuetype = napi_valuetype(4);
}
impl napi_valuetype {
    pub const napi_symbol: napi_valuetype = napi_valuetype(5);
}
impl napi_valuetype {
    pub const napi_object: napi_valuetype = napi_valuetype(6);
}
impl napi_valuetype {
    pub const napi_function: napi_valuetype = napi_valuetype(7);
}
impl napi_valuetype {
    pub const napi_external: napi_valuetype = napi_valuetype(8);
}
impl napi_valuetype {
    pub const napi_bigint: napi_valuetype = napi_valuetype(9);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_valuetype(pub ::core::ffi::c_uint);
impl napi_typedarray_type {
    pub const napi_int8_array: napi_typedarray_type = napi_typedarray_type(0);
}
impl napi_typedarray_type {
    pub const napi_uint8_array: napi_typedarray_type = napi_typedarray_type(1);
}
impl napi_typedarray_type {
    pub const napi_uint8_clamped_array: napi_typedarray_type = napi_typedarray_type(2);
}
impl napi_typedarray_type {
    pub const napi_int16_array: napi_typedarray_type = napi_typedarray_type(3);
}
impl napi_typedarray_type {
    pub const napi_uint16_array: napi_typedarray_type = napi_typedarray_type(4);
}
impl napi_typedarray_type {
    pub const napi_int32_array: napi_typedarray_type = napi_typedarray_type(5);
}
impl napi_typedarray_type {
    pub const napi_uint32_array: napi_typedarray_type = napi_typedarray_type(6);
}
impl napi_typedarray_type {
    pub const napi_float32_array: napi_typedarray_type = napi_typedarray_type(7);
}
impl napi_typedarray_type {
    pub const napi_float64_array: napi_typedarray_type = napi_typedarray_type(8);
}
impl napi_typedarray_type {
    pub const napi_bigint64_array: napi_typedarray_type = napi_typedarray_type(9);
}
impl napi_typedarray_type {
    pub const napi_biguint64_array: napi_typedarray_type = napi_typedarray_type(10);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_typedarray_type(pub ::core::ffi::c_uint);
impl napi_status {
    pub const napi_ok: napi_status = napi_status(0);
}
impl napi_status {
    pub const napi_invalid_arg: napi_status = napi_status(1);
}
impl napi_status {
    pub const napi_object_expected: napi_status = napi_status(2);
}
impl napi_status {
    pub const napi_string_expected: napi_status = napi_status(3);
}
impl napi_status {
    pub const napi_name_expected: napi_status = napi_status(4);
}
impl napi_status {
    pub const napi_function_expected: napi_status = napi_status(5);
}
impl napi_status {
    pub const napi_number_expected: napi_status = napi_status(6);
}
impl napi_status {
    pub const napi_boolean_expected: napi_status = napi_status(7);
}
impl napi_status {
    pub const napi_array_expected: napi_status = napi_status(8);
}
impl napi_status {
    pub const napi_generic_failure: napi_status = napi_status(9);
}
impl napi_status {
    pub const napi_pending_exception: napi_status = napi_status(10);
}
impl napi_status {
    pub const napi_cancelled: napi_status = napi_status(11);
}
impl napi_status {
    pub const napi_escape_called_twice: napi_status = napi_status(12);
}
impl napi_status {
    pub const napi_handle_scope_mismatch: napi_status = napi_status(13);
}
impl napi_status {
    pub const napi_callback_scope_mismatch: napi_status = napi_status(14);
}
impl napi_status {
    pub const napi_queue_full: napi_status = napi_status(15);
}
impl napi_status {
    pub const napi_closing: napi_status = napi_status(16);
}
impl napi_status {
    pub const napi_bigint_expected: napi_status = napi_status(17);
}
impl napi_status {
    pub const napi_date_expected: napi_status = napi_status(18);
}
impl napi_status {
    pub const napi_arraybuffer_expected: napi_status = napi_status(19);
}
impl napi_status {
    pub const napi_detachable_arraybuffer_expected: napi_status = napi_status(20);
}
impl napi_status {
    pub const napi_would_deadlock: napi_status = napi_status(21);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_status(pub ::core::ffi::c_uint);
pub type napi_callback = ::core::option::Option<
    unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value,
>;
pub type napi_finalize = ::core::option::Option<
    unsafe extern "C" fn(
        env: napi_env,
        finalize_data: *mut ::core::ffi::c_void,
        finalize_hint: *mut ::core::ffi::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_property_descriptor {
    pub utf8name: *const ::core::ffi::c_char,
    pub name: napi_value,
    pub method: napi_callback,
    pub getter: napi_callback,
    pub setter: napi_callback,
    pub value: napi_value,
    pub attributes: napi_property_attributes,
    pub data: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_extended_error_info {
    pub error_message: *const ::core::ffi::c_char,
    pub engine_reserved: *mut ::core::ffi::c_void,
    pub engine_error_code: u32,
    pub error_code: napi_status,
}
impl napi_key_collection_mode {
    pub const napi_key_include_prototypes: napi_key_collection_mode = napi_key_collection_mode(
        0,
    );
}
impl napi_key_collection_mode {
    pub const napi_key_own_only: napi_key_collection_mode = napi_key_collection_mode(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_key_collection_mode(pub ::core::ffi::c_uint);
impl napi_key_filter {
    pub const napi_key_all_properties: napi_key_filter = napi_key_filter(0);
}
impl napi_key_filter {
    pub const napi_key_writable: napi_key_filter = napi_key_filter(1);
}
impl napi_key_filter {
    pub const napi_key_enumerable: napi_key_filter = napi_key_filter(2);
}
impl napi_key_filter {
    pub const napi_key_configurable: napi_key_filter = napi_key_filter(4);
}
impl napi_key_filter {
    pub const napi_key_skip_strings: napi_key_filter = napi_key_filter(8);
}
impl napi_key_filter {
    pub const napi_key_skip_symbols: napi_key_filter = napi_key_filter(16);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_key_filter(pub ::core::ffi::c_uint);
impl napi_key_conversion {
    pub const napi_key_keep_numbers: napi_key_conversion = napi_key_conversion(0);
}
impl napi_key_conversion {
    pub const napi_key_numbers_to_strings: napi_key_conversion = napi_key_conversion(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_key_conversion(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_type_tag {
    pub lower: u64,
    pub upper: u64,
}
extern "C" {
    pub fn napi_get_last_error_info(
        env: napi_env,
        result: *mut *const napi_extended_error_info,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_undefined(env: napi_env, result: *mut napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_get_null(env: napi_env, result: *mut napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_get_global(env: napi_env, result: *mut napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_get_boolean(
        env: napi_env,
        value: bool,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_object(env: napi_env, result: *mut napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_create_array(env: napi_env, result: *mut napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_create_array_with_length(
        env: napi_env,
        length: usize,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_double(
        env: napi_env,
        value: f64,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_int32(
        env: napi_env,
        value: i32,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_uint32(
        env: napi_env,
        value: u32,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_int64(
        env: napi_env,
        value: i64,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_string_latin1(
        env: napi_env,
        str_: *const ::core::ffi::c_char,
        length: usize,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_string_utf8(
        env: napi_env,
        str_: *const ::core::ffi::c_char,
        length: usize,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_string_utf16(
        env: napi_env,
        str_: *const char16_t,
        length: usize,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_symbol(
        env: napi_env,
        description: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_function(
        env: napi_env,
        utf8name: *const ::core::ffi::c_char,
        length: usize,
        cb: napi_callback,
        data: *mut ::core::ffi::c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_error(
        env: napi_env,
        code: napi_value,
        msg: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_type_error(
        env: napi_env,
        code: napi_value,
        msg: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_range_error(
        env: napi_env,
        code: napi_value,
        msg: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_typeof(
        env: napi_env,
        value: napi_value,
        result: *mut napi_valuetype,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_double(
        env: napi_env,
        value: napi_value,
        result: *mut f64,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_int32(
        env: napi_env,
        value: napi_value,
        result: *mut i32,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_uint32(
        env: napi_env,
        value: napi_value,
        result: *mut u32,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_int64(
        env: napi_env,
        value: napi_value,
        result: *mut i64,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_bool(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_string_latin1(
        env: napi_env,
        value: napi_value,
        buf: *mut ::core::ffi::c_char,
        bufsize: usize,
        result: *mut usize,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_string_utf8(
        env: napi_env,
        value: napi_value,
        buf: *mut ::core::ffi::c_char,
        bufsize: usize,
        result: *mut usize,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_string_utf16(
        env: napi_env,
        value: napi_value,
        buf: *mut char16_t,
        bufsize: usize,
        result: *mut usize,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_coerce_to_bool(
        env: napi_env,
        value: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_coerce_to_number(
        env: napi_env,
        value: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_coerce_to_object(
        env: napi_env,
        value: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_coerce_to_string(
        env: napi_env,
        value: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_prototype(
        env: napi_env,
        object: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_property_names(
        env: napi_env,
        object: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_set_property(
        env: napi_env,
        object: napi_value,
        key: napi_value,
        value: napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_has_property(
        env: napi_env,
        object: napi_value,
        key: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_property(
        env: napi_env,
        object: napi_value,
        key: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_delete_property(
        env: napi_env,
        object: napi_value,
        key: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_has_own_property(
        env: napi_env,
        object: napi_value,
        key: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_set_named_property(
        env: napi_env,
        object: napi_value,
        utf8name: *const ::core::ffi::c_char,
        value: napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_has_named_property(
        env: napi_env,
        object: napi_value,
        utf8name: *const ::core::ffi::c_char,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_named_property(
        env: napi_env,
        object: napi_value,
        utf8name: *const ::core::ffi::c_char,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_set_element(
        env: napi_env,
        object: napi_value,
        index: u32,
        value: napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_has_element(
        env: napi_env,
        object: napi_value,
        index: u32,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_element(
        env: napi_env,
        object: napi_value,
        index: u32,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_delete_element(
        env: napi_env,
        object: napi_value,
        index: u32,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_define_properties(
        env: napi_env,
        object: napi_value,
        property_count: usize,
        properties: *const napi_property_descriptor,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_array(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_array_length(
        env: napi_env,
        value: napi_value,
        result: *mut u32,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_strict_equals(
        env: napi_env,
        lhs: napi_value,
        rhs: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_call_function(
        env: napi_env,
        recv: napi_value,
        func: napi_value,
        argc: usize,
        argv: *const napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_new_instance(
        env: napi_env,
        constructor: napi_value,
        argc: usize,
        argv: *const napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_instanceof(
        env: napi_env,
        object: napi_value,
        constructor: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_cb_info(
        env: napi_env,
        cbinfo: napi_callback_info,
        argc: *mut usize,
        argv: *mut napi_value,
        this_arg: *mut napi_value,
        data: *mut *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_new_target(
        env: napi_env,
        cbinfo: napi_callback_info,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_define_class(
        env: napi_env,
        utf8name: *const ::core::ffi::c_char,
        length: usize,
        constructor: napi_callback,
        data: *mut ::core::ffi::c_void,
        property_count: usize,
        properties: *const napi_property_descriptor,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_wrap(
        env: napi_env,
        js_object: napi_value,
        native_object: *mut ::core::ffi::c_void,
        finalize_cb: napi_finalize,
        finalize_hint: *mut ::core::ffi::c_void,
        result: *mut napi_ref,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_unwrap(
        env: napi_env,
        js_object: napi_value,
        result: *mut *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_remove_wrap(
        env: napi_env,
        js_object: napi_value,
        result: *mut *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_external(
        env: napi_env,
        data: *mut ::core::ffi::c_void,
        finalize_cb: napi_finalize,
        finalize_hint: *mut ::core::ffi::c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_external(
        env: napi_env,
        value: napi_value,
        result: *mut *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_reference(
        env: napi_env,
        value: napi_value,
        initial_refcount: u32,
        result: *mut napi_ref,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_delete_reference(env: napi_env, ref_: napi_ref) -> napi_status;
}
extern "C" {
    pub fn napi_reference_ref(
        env: napi_env,
        ref_: napi_ref,
        result: *mut u32,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_reference_unref(
        env: napi_env,
        ref_: napi_ref,
        result: *mut u32,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_reference_value(
        env: napi_env,
        ref_: napi_ref,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_open_handle_scope(
        env: napi_env,
        result: *mut napi_handle_scope,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_close_handle_scope(
        env: napi_env,
        scope: napi_handle_scope,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_open_escapable_handle_scope(
        env: napi_env,
        result: *mut napi_escapable_handle_scope,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_close_escapable_handle_scope(
        env: napi_env,
        scope: napi_escapable_handle_scope,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_escape_handle(
        env: napi_env,
        scope: napi_escapable_handle_scope,
        escapee: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_throw(env: napi_env, error: napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_throw_error(
        env: napi_env,
        code: *const ::core::ffi::c_char,
        msg: *const ::core::ffi::c_char,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_throw_type_error(
        env: napi_env,
        code: *const ::core::ffi::c_char,
        msg: *const ::core::ffi::c_char,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_throw_range_error(
        env: napi_env,
        code: *const ::core::ffi::c_char,
        msg: *const ::core::ffi::c_char,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_error(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_exception_pending(env: napi_env, result: *mut bool) -> napi_status;
}
extern "C" {
    pub fn napi_get_and_clear_last_exception(
        env: napi_env,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_arraybuffer(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_arraybuffer(
        env: napi_env,
        byte_length: usize,
        data: *mut *mut ::core::ffi::c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_external_arraybuffer(
        env: napi_env,
        external_data: *mut ::core::ffi::c_void,
        byte_length: usize,
        finalize_cb: napi_finalize,
        finalize_hint: *mut ::core::ffi::c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_arraybuffer_info(
        env: napi_env,
        arraybuffer: napi_value,
        data: *mut *mut ::core::ffi::c_void,
        byte_length: *mut usize,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_typedarray(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_typedarray(
        env: napi_env,
        type_: napi_typedarray_type,
        length: usize,
        arraybuffer: napi_value,
        byte_offset: usize,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_typedarray_info(
        env: napi_env,
        typedarray: napi_value,
        type_: *mut napi_typedarray_type,
        length: *mut usize,
        data: *mut *mut ::core::ffi::c_void,
        arraybuffer: *mut napi_value,
        byte_offset: *mut usize,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_dataview(
        env: napi_env,
        length: usize,
        arraybuffer: napi_value,
        byte_offset: usize,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_dataview(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_dataview_info(
        env: napi_env,
        dataview: napi_value,
        bytelength: *mut usize,
        data: *mut *mut ::core::ffi::c_void,
        arraybuffer: *mut napi_value,
        byte_offset: *mut usize,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_version(env: napi_env, result: *mut u32) -> napi_status;
}
extern "C" {
    pub fn napi_create_promise(
        env: napi_env,
        deferred: *mut napi_deferred,
        promise: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_resolve_deferred(
        env: napi_env,
        deferred: napi_deferred,
        resolution: napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_reject_deferred(
        env: napi_env,
        deferred: napi_deferred,
        rejection: napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_promise(
        env: napi_env,
        value: napi_value,
        is_promise: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_run_script(
        env: napi_env,
        script: napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_adjust_external_memory(
        env: napi_env,
        change_in_bytes: i64,
        adjusted_value: *mut i64,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_date(
        env: napi_env,
        time: f64,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_date(
        env: napi_env,
        value: napi_value,
        is_date: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_date_value(
        env: napi_env,
        value: napi_value,
        result: *mut f64,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_add_finalizer(
        env: napi_env,
        js_object: napi_value,
        native_object: *mut ::core::ffi::c_void,
        finalize_cb: napi_finalize,
        finalize_hint: *mut ::core::ffi::c_void,
        result: *mut napi_ref,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_bigint_int64(
        env: napi_env,
        value: i64,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_bigint_uint64(
        env: napi_env,
        value: u64,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_bigint_words(
        env: napi_env,
        sign_bit: ::core::ffi::c_int,
        word_count: usize,
        words: *const u64,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_bigint_int64(
        env: napi_env,
        value: napi_value,
        result: *mut i64,
        lossless: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_bigint_uint64(
        env: napi_env,
        value: napi_value,
        result: *mut u64,
        lossless: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_value_bigint_words(
        env: napi_env,
        value: napi_value,
        sign_bit: *mut ::core::ffi::c_int,
        word_count: *mut usize,
        words: *mut u64,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_all_property_names(
        env: napi_env,
        object: napi_value,
        key_mode: napi_key_collection_mode,
        key_filter: napi_key_filter,
        key_conversion: napi_key_conversion,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_set_instance_data(
        env: napi_env,
        data: *mut ::core::ffi::c_void,
        finalize_cb: napi_finalize,
        finalize_hint: *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_instance_data(
        env: napi_env,
        data: *mut *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_detach_arraybuffer(
        env: napi_env,
        arraybuffer: napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_detached_arraybuffer(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_type_tag_object(
        env: napi_env,
        value: napi_value,
        type_tag: *const napi_type_tag,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_check_object_type_tag(
        env: napi_env,
        value: napi_value,
        type_tag: *const napi_type_tag,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_object_freeze(env: napi_env, object: napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_object_seal(env: napi_env, object: napi_value) -> napi_status;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_callback_scope__ {
    _unused: [u8; 0],
}
pub type napi_callback_scope = *mut napi_callback_scope__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_async_context__ {
    _unused: [u8; 0],
}
pub type napi_async_context = *mut napi_async_context__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_async_work__ {
    _unused: [u8; 0],
}
pub type napi_async_work = *mut napi_async_work__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_threadsafe_function__ {
    _unused: [u8; 0],
}
pub type napi_threadsafe_function = *mut napi_threadsafe_function__;
impl napi_threadsafe_function_release_mode {
    pub const napi_tsfn_release: napi_threadsafe_function_release_mode = napi_threadsafe_function_release_mode(
        0,
    );
}
impl napi_threadsafe_function_release_mode {
    pub const napi_tsfn_abort: napi_threadsafe_function_release_mode = napi_threadsafe_function_release_mode(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_threadsafe_function_release_mode(pub ::core::ffi::c_uint);
impl napi_threadsafe_function_call_mode {
    pub const napi_tsfn_nonblocking: napi_threadsafe_function_call_mode = napi_threadsafe_function_call_mode(
        0,
    );
}
impl napi_threadsafe_function_call_mode {
    pub const napi_tsfn_blocking: napi_threadsafe_function_call_mode = napi_threadsafe_function_call_mode(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct napi_threadsafe_function_call_mode(pub ::core::ffi::c_uint);
pub type napi_async_execute_callback = ::core::option::Option<
    unsafe extern "C" fn(env: napi_env, data: *mut ::core::ffi::c_void),
>;
pub type napi_async_complete_callback = ::core::option::Option<
    unsafe extern "C" fn(
        env: napi_env,
        status: napi_status,
        data: *mut ::core::ffi::c_void,
    ),
>;
pub type napi_threadsafe_function_call_js = ::core::option::Option<
    unsafe extern "C" fn(
        env: napi_env,
        js_callback: napi_value,
        context: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_node_version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub release: *const ::core::ffi::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_async_cleanup_hook_handle__ {
    _unused: [u8; 0],
}
pub type napi_async_cleanup_hook_handle = *mut napi_async_cleanup_hook_handle__;
pub type napi_async_cleanup_hook = ::core::option::Option<
    unsafe extern "C" fn(
        handle: napi_async_cleanup_hook_handle,
        data: *mut ::core::ffi::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uv_loop_s {
    _unused: [u8; 0],
}
pub type napi_addon_register_func = ::core::option::Option<
    unsafe extern "C" fn(env: napi_env, exports: napi_value) -> napi_value,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct napi_module {
    pub nm_version: ::core::ffi::c_int,
    pub nm_flags: ::core::ffi::c_uint,
    pub nm_filename: *const ::core::ffi::c_char,
    pub nm_register_func: napi_addon_register_func,
    pub nm_modname: *const ::core::ffi::c_char,
    pub nm_priv: *mut ::core::ffi::c_void,
    pub reserved: [*mut ::core::ffi::c_void; 4usize],
}
extern "C" {
    pub fn napi_module_register(mod_: *mut napi_module);
}
extern "C" {
    pub fn napi_fatal_error(
        location: *const ::core::ffi::c_char,
        location_len: usize,
        message: *const ::core::ffi::c_char,
        message_len: usize,
    ) -> !;
}
extern "C" {
    pub fn napi_async_init(
        env: napi_env,
        async_resource: napi_value,
        async_resource_name: napi_value,
        result: *mut napi_async_context,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_async_destroy(
        env: napi_env,
        async_context: napi_async_context,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_make_callback(
        env: napi_env,
        async_context: napi_async_context,
        recv: napi_value,
        func: napi_value,
        argc: usize,
        argv: *const napi_value,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_buffer(
        env: napi_env,
        length: usize,
        data: *mut *mut ::core::ffi::c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_external_buffer(
        env: napi_env,
        length: usize,
        data: *mut ::core::ffi::c_void,
        finalize_cb: napi_finalize,
        finalize_hint: *mut ::core::ffi::c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_buffer_copy(
        env: napi_env,
        length: usize,
        data: *const ::core::ffi::c_void,
        result_data: *mut *mut ::core::ffi::c_void,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_is_buffer(
        env: napi_env,
        value: napi_value,
        result: *mut bool,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_buffer_info(
        env: napi_env,
        value: napi_value,
        data: *mut *mut ::core::ffi::c_void,
        length: *mut usize,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_async_work(
        env: napi_env,
        async_resource: napi_value,
        async_resource_name: napi_value,
        execute: napi_async_execute_callback,
        complete: napi_async_complete_callback,
        data: *mut ::core::ffi::c_void,
        result: *mut napi_async_work,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_delete_async_work(env: napi_env, work: napi_async_work) -> napi_status;
}
extern "C" {
    pub fn napi_queue_async_work(env: napi_env, work: napi_async_work) -> napi_status;
}
extern "C" {
    pub fn napi_cancel_async_work(env: napi_env, work: napi_async_work) -> napi_status;
}
extern "C" {
    pub fn napi_get_node_version(
        env: napi_env,
        version: *mut *const napi_node_version,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_uv_event_loop(
        env: napi_env,
        loop_: *mut *mut uv_loop_s,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_fatal_exception(env: napi_env, err: napi_value) -> napi_status;
}
extern "C" {
    pub fn napi_add_env_cleanup_hook(
        env: napi_env,
        fun: ::core::option::Option<unsafe extern "C" fn(arg: *mut ::core::ffi::c_void)>,
        arg: *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_remove_env_cleanup_hook(
        env: napi_env,
        fun: ::core::option::Option<unsafe extern "C" fn(arg: *mut ::core::ffi::c_void)>,
        arg: *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_open_callback_scope(
        env: napi_env,
        resource_object: napi_value,
        context: napi_async_context,
        result: *mut napi_callback_scope,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_close_callback_scope(
        env: napi_env,
        scope: napi_callback_scope,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_create_threadsafe_function(
        env: napi_env,
        func: napi_value,
        async_resource: napi_value,
        async_resource_name: napi_value,
        max_queue_size: usize,
        initial_thread_count: usize,
        thread_finalize_data: *mut ::core::ffi::c_void,
        thread_finalize_cb: napi_finalize,
        context: *mut ::core::ffi::c_void,
        call_js_cb: napi_threadsafe_function_call_js,
        result: *mut napi_threadsafe_function,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_get_threadsafe_function_context(
        func: napi_threadsafe_function,
        result: *mut *mut ::core::ffi::c_void,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_call_threadsafe_function(
        func: napi_threadsafe_function,
        data: *mut ::core::ffi::c_void,
        is_blocking: napi_threadsafe_function_call_mode,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_acquire_threadsafe_function(
        func: napi_threadsafe_function,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_release_threadsafe_function(
        func: napi_threadsafe_function,
        mode: napi_threadsafe_function_release_mode,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_unref_threadsafe_function(
        env: napi_env,
        func: napi_threadsafe_function,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_ref_threadsafe_function(
        env: napi_env,
        func: napi_threadsafe_function,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_add_async_cleanup_hook(
        env: napi_env,
        hook: napi_async_cleanup_hook,
        arg: *mut ::core::ffi::c_void,
        remove_handle: *mut napi_async_cleanup_hook_handle,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_remove_async_cleanup_hook(
        remove_handle: napi_async_cleanup_hook_handle,
    ) -> napi_status;
}
extern "C" {
    pub fn node_api_get_module_file_name(
        env: napi_env,
        result: *mut *const ::core::ffi::c_char,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_run_script_path(
        env: napi_env,
        path: *const ::core::ffi::c_char,
        result: *mut napi_value,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_queue_async_work_with_qos(
        env: napi_env,
        work: napi_async_work,
        qos: napi_qos_t,
    ) -> napi_status;
}
extern "C" {
    pub fn napi_load_module(
        env: napi_env,
        path: *const ::core::ffi::c_char,
        result: *mut napi_value,
    ) -> napi_status;
}
