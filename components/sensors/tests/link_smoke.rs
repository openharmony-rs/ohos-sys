#![cfg(feature = "api-11")]

use std::ptr;

use ohos_sensors_sys as sensors;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    unsafe {
        let _ = sensors::sensor::OH_Sensor_GetInfos(ptr::null_mut(), ptr::null_mut());
        let _ = sensors::sensor_type::OH_Sensor_CreateInfos(0);
        let _ = sensors::vibrator::OH_Vibrator_PlayVibration(0, std::mem::zeroed());
    }

    touch_type::<sensors::vibrator_type::Vibrator_Attribute>();
}
