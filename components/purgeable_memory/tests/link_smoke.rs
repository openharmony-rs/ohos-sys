use ohos_purgeable_memory_sys as purgeable_memory;

#[test]
fn link_smoke() {
    unsafe {
        let _ = purgeable_memory::OH_PurgeableMemory_ContentSize(std::ptr::null_mut());
    }
}
