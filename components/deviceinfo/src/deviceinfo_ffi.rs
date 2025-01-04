// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern "C" {
    /// Obtains the device type represented by a string,
    /// which can be `phone` (or `default` for phones), `wearable`, `liteWearable`,
    /// `tablet`, `tv`, `car`, or `smartVision`.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetDeviceType() -> *const ::core::ffi::c_char;
    /// Obtains the device manufacturer represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetManufacture() -> *const ::core::ffi::c_char;
    /// Obtains the device brand represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetBrand() -> *const ::core::ffi::c_char;
    /// Obtains the product name speaded in the market
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetMarketName() -> *const ::core::ffi::c_char;
    /// Obtains the product series represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetProductSeries() -> *const ::core::ffi::c_char;
    /// Obtains the product model represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetProductModel() -> *const ::core::ffi::c_char;
    /// Obtains the software model represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetSoftwareModel() -> *const ::core::ffi::c_char;
    /// Obtains the hardware model represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetHardwareModel() -> *const ::core::ffi::c_char;
    /// Obtains the bootloader version number represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetBootloaderVersion() -> *const ::core::ffi::c_char;
    /// Obtains the application binary interface (Abi) list represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetAbiList() -> *const ::core::ffi::c_char;
    /// Obtains the security patch tag represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetSecurityPatchTag() -> *const ::core::ffi::c_char;
    /// Obtains the product version displayed for customer represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetDisplayVersion() -> *const ::core::ffi::c_char;
    /// Obtains the incremental version represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetIncrementalVersion() -> *const ::core::ffi::c_char;
    /// Obtains the OS release type represented by a string.
    ///
    /// <p>The OS release category can be `Release`, `Beta`, or `Canary`.
    /// The specific release type may be `Release`, `Beta1`, or others alike.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetOsReleaseType() -> *const ::core::ffi::c_char;
    /// Obtains the OS full version name represented by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetOSFullName() -> *const ::core::ffi::c_char;
    /// Obtains the SDK API version number.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetSdkApiVersion() -> ::core::ffi::c_int;
    /// Obtains the first API version number.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetFirstApiVersion() -> ::core::ffi::c_int;
    /// Obtains the version ID by a string.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetVersionId() -> *const ::core::ffi::c_char;
    /// Obtains the build type of the current running OS.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetBuildType() -> *const ::core::ffi::c_char;
    /// Obtains the build user of the current running OS.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetBuildUser() -> *const ::core::ffi::c_char;
    /// Obtains the build host of the current running OS.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetBuildHost() -> *const ::core::ffi::c_char;
    /// Obtains the build time of the current running OS.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetBuildTime() -> *const ::core::ffi::c_char;
    /// Obtains the version hash of the current running OS.
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetBuildRootHash() -> *const ::core::ffi::c_char;
    /// Obtains the Distribution OS name represented by a string.
    ///
    /// <p>Independent Software Vendor (ISV) may distribute OHOS with their own OS name.
    /// If ISV not specified, it will return an empty string
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetDistributionOSName() -> *const ::core::ffi::c_char;
    /// Obtains the ISV distribution OS version represented by a string.
    /// If ISV not specified, it will return the same value as OH_GetOSFullName
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetDistributionOSVersion() -> *const ::core::ffi::c_char;
    /// Obtains the ISV distribution OS api version represented by a integer.
    /// If ISV not specified, it will return the same value as OH_GetSdkApiVersion
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetDistributionOSApiVersion() -> ::core::ffi::c_int;
    /// Obtains the ISV distribution OS release type represented by a string.
    /// If ISV not specified, it will return the same value as OH_GetOsReleaseType
    ///
    /// Required System Capabilities: SystemCapability.Startup.SystemInfo
    ///
    /// Available since API-level: 10
    pub fn OH_GetDistributionOSReleaseType() -> *const ::core::ffi::c_char;
}
