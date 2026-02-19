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

    #[cfg(feature = "api-11")]
    unsafe {
        let _ = rawfile::raw_file::OH_ResourceManager_GetRawFileRemainingLength(ptr::null_mut());
        let _ = rawfile::raw_file_manager::OH_ResourceManager_OpenRawFile64(
            ptr::null_mut(),
            ptr::null(),
        );
    }

    #[cfg(feature = "api-12")]
    unsafe {
        let _ = rawfile::raw_file::OH_ResourceManager_GetRawFileDescriptorData(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ =
            rawfile::raw_file_manager::OH_ResourceManager_IsRawDir(ptr::null_mut(), ptr::null());
    }
}
