use hitrace_sys as hitrace;

#[test]
fn link_smoke() {
    unsafe {
        let _ = hitrace::OH_HiTrace_StartTrace(c"test".as_ptr());
        #[cfg(feature = "api-19")]
        let _ = hitrace::OH_HiTrace_IsTraceEnabled();
    }
}
