mod types_ffi;

pub use types_ffi::*;

impl InputMethod_ErrorCode {
    pub fn is_ok(&self) -> bool {
        *self == Self::IME_ERR_OK
    }
}
