use std::ptr;

use ohos_rdb_sys as rdb;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = rdb::relational_store::OH_Rdb_CreateConfig();
        let _ = rdb::rdb_transaction::OH_RdbTrans_CreateOptions();
    }

    touch_type::<rdb::relational_store_error_code::OH_Rdb_ErrCode>();
    touch_type::<rdb::value_object::OH_VObject>();

    #[cfg(feature = "api-11")]
    unsafe {
        let _ = rdb::data_asset::OH_Data_Asset_CreateOne();
        let _ =
            rdb::values_bucket::OH_VBucket_PutAsset(ptr::null_mut(), ptr::null(), ptr::null_mut());
    }

    #[cfg(feature = "api-18")]
    unsafe {
        let _ = rdb::cursor::OH_Cursor_GetFloatVectorCount(ptr::null_mut(), 0, ptr::null_mut());
        let _ = rdb::data_value::OH_Value_Create();
        let _ = rdb::data_values::OH_Values_Create();
        let _ = rdb::data_values_buckets::OH_VBuckets_Create();
    }

    #[cfg(feature = "api-18")]
    touch_type::<rdb::rdb_types::Rdb_ConflictResolution>();

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = rdb::predicates::OH_Predicates_NotLike(ptr::null_mut(), ptr::null(), ptr::null());
        let _ = rdb::rdb_crypto_param::OH_Rdb_CreateCryptoParam();
    }
}
