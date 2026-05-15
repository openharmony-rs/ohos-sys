#[cfg(all(feature = "commonevent", feature = "api-12"))]
use ohos_basic_services_kit_sys::commonevent;
#[cfg(all(feature = "commonevent", feature = "api-18"))]
use ohos_basic_services_kit_sys::commonevent_support;

#[test]
fn link_smoke() {
    #[cfg(all(feature = "commonevent", feature = "api-12"))]
    unsafe {
        let mut events = [c"usual.event.SCREEN_ON".as_ptr()];
        let info = commonevent::OH_CommonEvent_CreateSubscribeInfo(events.as_mut_ptr(), 1);
        commonevent::OH_CommonEvent_DestroySubscribeInfo(info);
    }

    #[cfg(all(feature = "commonevent", feature = "api-18"))]
    unsafe {
        let _ = commonevent::OH_CommonEvent_Publish(
            commonevent_support::COMMON_EVENT_SCREEN_ON.as_ptr(),
        );
    }

    #[cfg(all(feature = "battery-info", feature = "api-13"))]
    unsafe {
        let _ = ohos_basic_services_kit_sys::battery_info::OH_BatteryInfo_GetCapacity();
    }

    #[cfg(all(feature = "print", feature = "api-12"))]
    unsafe {
        let _ = ohos_basic_services_kit_sys::print::OH_Print_Init();
    }

    #[cfg(all(feature = "scan", feature = "api-12"))]
    unsafe {
        let _ = ohos_basic_services_kit_sys::scan::OH_Scan_Init();
    }

    #[cfg(all(feature = "os-account", feature = "api-12"))]
    unsafe {
        let mut buf = [0u8; 32];
        let _ = ohos_basic_services_kit_sys::os_account::OH_OsAccount_GetName(
            buf.as_mut_ptr() as *mut _,
            buf.len(),
        );
    }

    #[cfg(all(feature = "time-service", feature = "api-12"))]
    unsafe {
        let mut buf = [0u8; 64];
        let _ = ohos_basic_services_kit_sys::time_service::OH_TimeService_GetTimeZone(
            buf.as_mut_ptr() as *mut _,
            buf.len() as u32,
        );
    }
}
