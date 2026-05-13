use std::ptr;

use ohos_asset_store_sys as asset;

#[test]
fn link_smoke() {
    #[cfg(feature = "api-11")]
    unsafe {
        let _ = asset::asset_api::OH_Asset_Add(ptr::null(), 0);
        let _ = asset::asset_api::OH_Asset_Remove(ptr::null(), 0);
        let _ = asset::asset_api::OH_Asset_Update(ptr::null(), 0, ptr::null(), 0);
        let _ = asset::asset_api::OH_Asset_PreQuery(ptr::null(), 0, ptr::null_mut());
        let _ = asset::asset_api::OH_Asset_Query(ptr::null(), 0, ptr::null_mut());
        let _ = asset::asset_api::OH_Asset_PostQuery(ptr::null(), 0);
        let _ = asset::asset_api::OH_Asset_ParseAttr(
            ptr::null(),
            asset::asset_type::Asset_Tag::ASSET_TAG_ALIAS,
        );
        asset::asset_api::OH_Asset_FreeBlob(ptr::null_mut());
        asset::asset_api::OH_Asset_FreeResultSet(ptr::null_mut());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = asset::asset_api::OH_Asset_QuerySyncResult(ptr::null(), 0, ptr::null_mut());
    }
}
