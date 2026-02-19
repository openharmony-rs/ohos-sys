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
        let _ = rdb::relational_store::OH_Rdb_SetDistributedTables(
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            rdb::relational_store::Rdb_DistributedType::RDB_DISTRIBUTED_CLOUD,
            ptr::null(),
        );
    }

    #[cfg(feature = "api-18")]
    unsafe {
        let _ = rdb::cursor::OH_Cursor_GetFloatVectorCount(ptr::null_mut(), 0, ptr::null_mut());
        let _ = rdb::data_value::OH_Value_Create();
        let _ = rdb::data_values::OH_Values_Create();
        let _ = rdb::data_values_buckets::OH_VBuckets_Create();
        let _ = rdb::relational_store::OH_Rdb_SetPersistent(ptr::null_mut(), false);
        let _ = rdb::values_bucket::OH_VBucket_PutFloatVector(
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
            0,
        );
    }

    #[cfg(feature = "api-18")]
    touch_type::<rdb::rdb_types::Rdb_ConflictResolution>();

    #[cfg(feature = "api-17")]
    unsafe {
        let _ = rdb::relational_store::OH_Rdb_SetTokenizer(ptr::null_mut(), std::mem::zeroed());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = rdb::predicates::OH_Predicates_NotLike(ptr::null_mut(), ptr::null(), ptr::null());
        let _ = rdb::rdb_crypto_param::OH_Rdb_CreateCryptoParam();
        let _ = rdb::rdb_transaction::OH_RdbTrans_InsertWithConflictResolution(
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
            std::mem::zeroed(),
            ptr::null_mut(),
        );
        let _ = rdb::relational_store::OH_Rdb_SetCustomDir(ptr::null_mut(), ptr::null());
    }

    #[cfg(feature = "api-10")]
    unsafe {
        let _ = rdb::relational_store::OH_Rdb_CreateValueObject();
    }
}
