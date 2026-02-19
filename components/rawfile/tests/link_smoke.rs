use std::ptr;

use ohos_rawfile_sys as rawfile;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    #[cfg(feature = "_functions")]
    unsafe {
        let _ = rawfile::raw_file_manager::OH_ResourceManager_InitNativeResourceManager(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = rawfile::raw_dir::OH_ResourceManager_GetRawFileCount(ptr::null_mut());
        let _ = rawfile::raw_file::OH_ResourceManager_GetRawFileSize(ptr::null_mut());
    }

    touch_type::<rawfile::RawFile>();
}
