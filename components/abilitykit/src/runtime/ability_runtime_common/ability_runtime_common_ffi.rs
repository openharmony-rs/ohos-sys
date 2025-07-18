// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type AbilityRuntimeResult = Result<(), AbilityRuntimeErrorCode>;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
impl AbilityRuntimeErrorCode {
    /// permission denied.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const PERMISSION_DENIED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(201).unwrap() });
    /// Invalid parameters.
    pub const PARAM_INVALID: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(401).unwrap() });
    /// StartSelfUIAbility is not supported.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const NOT_SUPPORTED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(801).unwrap() });
    /// No such ability.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const NO_SUCH_ABILITY: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000001).unwrap() });
    /// Incorrect ability type.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const INCORRECT_ABILITY_TYPE: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000002).unwrap() });
    /// The crowdtesting application expires.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const CROWDTEST_EXPIRED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000008).unwrap() });
    /// The ability cannot be started in Wukong Mode.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const WUKONG_MODE: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000009).unwrap() });
    /// The context does not exist.
    pub const CONTEXT_NOT_EXIST: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000011).unwrap() });
    /// The app is controlled.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const CONTROLLED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000012).unwrap() });
    /// The app is controlled by EDM.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const EDM_CONTROLLED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000013).unwrap() });
    /// Cross-app start is not allowed.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const CROSS_APP: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000018).unwrap() });
    /// Internal error.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const INTERNAL: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000050).unwrap() });
    /// Not top ability.
    ///
    /// Available since API-level: 15
    #[cfg(feature = "api-15")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-15")))]
    pub const NOT_TOP_ABILITY: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000053).unwrap() });
    /// Setting visibility is disabled.
    ///
    /// Available since API-level: 17
    #[cfg(feature = "api-17")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
    pub const ABILITY_RUNTIME_ERROR_VISIBILITY_SETTING_DISABLED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000067).unwrap() });
    /// App clone or multi-instance is not supported.
    ///
    /// Available since API-level: 17
    #[cfg(feature = "api-17")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
    pub const MULTI_APP_NOT_SUPPORTED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000072).unwrap() });
    /// The app instance key is invalid.
    ///
    /// Available since API-level: 17
    #[cfg(feature = "api-17")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
    pub const INVALID_APP_INSTANCE_KEY: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000076).unwrap() });
    /// The number of app instances reaches the limit.
    ///
    /// Available since API-level: 17
    #[cfg(feature = "api-17")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
    pub const UPPER_LIMIT_REACHED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000077).unwrap() });
    /// The multi-instance is not supported.
    ///
    /// Available since API-level: 17
    #[cfg(feature = "api-17")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
    pub const ABILITY_RUNTIME_ERROR_MULTI_INSTANCE_NOT_SUPPORTED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000078).unwrap() });
    /// The APP_INSTANCE_KEY cannot be specified.
    ///
    /// Available since API-level: 17
    #[cfg(feature = "api-17")]
    #[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
    pub const APP_INSTANCE_KEY_NOT_SUPPORTED: AbilityRuntimeErrorCode =
        AbilityRuntimeErrorCode(const { core::num::NonZero::new(16000079).unwrap() });
}
#[repr(transparent)]
/// Enumerates the error codes.
///
///
/// Available since API-level: 13
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct AbilityRuntimeErrorCode(pub core::num::NonZero<::core::ffi::c_uint>);
