use ohos_transient_task_sys as transient_task;

#[cfg(feature = "api-13")]
#[test]
fn link_smoke_api_13() {
    unsafe {
        let _ = transient_task::transient_task_api::OH_BackgroundTaskManager_CancelSuspendDelay(0);
    }
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    use core::ptr;
    unsafe {
        let _ = transient_task::transient_task_api::OH_BackgroundTaskManager_GetTransientTaskInfo(
            ptr::null_mut(),
        );
    }
}
