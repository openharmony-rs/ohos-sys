// Sizes, alignments, and offsets checked against the SDK C headers with the
// aarch64 and armv7 OpenHarmony Clang targets.
use core::mem::{align_of, offset_of, size_of};
#[cfg(feature = "api-18")]
use ohos_usb_sys::Usb_DeviceArray;
use ohos_usb_sys::{
    DDK_Ashmem, UsbConfigDescriptor, UsbControlRequestSetup, UsbDdkConfigDescriptor,
    UsbDeviceDescriptor, UsbDeviceMemMap, UsbEndpointDescriptor, UsbRequestPipe,
};

const _: () = {
    assert!(size_of::<UsbControlRequestSetup>() == 8);
    assert!(align_of::<UsbControlRequestSetup>() == 8);
    assert!(size_of::<UsbDeviceDescriptor>() == 24);
    assert!(align_of::<UsbDeviceDescriptor>() == 8);
    assert!(size_of::<UsbConfigDescriptor>() == 9);
    assert!(align_of::<UsbConfigDescriptor>() == 1);
    assert!(size_of::<UsbEndpointDescriptor>() == 9);
    assert!(align_of::<UsbEndpointDescriptor>() == 1);
    assert!(size_of::<UsbRequestPipe>() == 16);
    assert!(align_of::<UsbRequestPipe>() == 8);
};

#[cfg(target_pointer_width = "64")]
const _: () = {
    assert!(size_of::<DDK_Ashmem>() == 32);
    assert!(offset_of!(DDK_Ashmem, address) == 8);
    assert!(size_of::<UsbDdkConfigDescriptor>() == 40);
    assert!(offset_of!(UsbDdkConfigDescriptor, interface) == 16);
    assert!(size_of::<UsbDeviceMemMap>() == 32);
};

#[cfg(target_pointer_width = "32")]
const _: () = {
    assert!(size_of::<DDK_Ashmem>() == 24);
    assert!(offset_of!(DDK_Ashmem, address) == 4);
    assert!(size_of::<UsbDdkConfigDescriptor>() == 24);
    assert!(offset_of!(UsbDdkConfigDescriptor, interface) == 12);
    assert!(size_of::<UsbDeviceMemMap>() == 20);
};

#[cfg(all(feature = "api-18", target_pointer_width = "64"))]
const _: () = {
    assert!(size_of::<Usb_DeviceArray>() == 16);
};

#[cfg(all(feature = "api-18", target_pointer_width = "32"))]
const _: () = {
    assert!(size_of::<Usb_DeviceArray>() == 8);
};
