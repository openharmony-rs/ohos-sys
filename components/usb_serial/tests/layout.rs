#[cfg(feature = "api-18")]
const _: () = {
    use core::mem::{align_of, offset_of, size_of};
    use ohos_usb_serial_sys::{UsbSerial_DdkRetCode, UsbSerial_Params};

    assert!(size_of::<UsbSerial_Params>() == 8);
    assert!(align_of::<UsbSerial_Params>() == 8);
    assert!(offset_of!(UsbSerial_Params, baudRate) == 0);
    assert!(offset_of!(UsbSerial_Params, nDataBits) == 4);
    assert!(offset_of!(UsbSerial_Params, nStopBits) == 5);
    assert!(offset_of!(UsbSerial_Params, parity) == 6);
    assert!(UsbSerial_DdkRetCode::USB_SERIAL_DDK_SUCCESS.0 == 31600000);
};
