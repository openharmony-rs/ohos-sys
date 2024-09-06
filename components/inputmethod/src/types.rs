mod types_api12;

pub use types_api12::*;

impl InputMethod_ErrorCode {
    pub fn is_ok(&self) -> bool {
        *self == Self::IME_ERR_OK
    }
}
