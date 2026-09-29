#![cfg(feature = "api-12")]

use std::ptr;

use ohos_abilitykit_sys as abilitykit;

#[test]
fn link_smoke() {
    unsafe {
        #[cfg(feature = "api-15")]
        {
            let element: abilitykit::base::want::AbilityBase_Element = std::mem::zeroed();
            let _ = abilitykit::base::want::OH_AbilityBase_CreateWant(element);
        }

        #[cfg(feature = "api-13")]
        {
            let _ = abilitykit::runtime::application_context::
                OH_AbilityRuntime_ApplicationContextGetCacheDir(ptr::null_mut(), 0, ptr::null_mut());
        }

        #[cfg(feature = "api-17")]
        {
            let _ = abilitykit::runtime::start_options::OH_AbilityRuntime_CreateStartOptions();
        }

        let _ =
            abilitykit::childprocess::OH_Ability_CreateNativeChildProcess(ptr::null_mut(), None);
    }

    #[cfg(feature = "api-24")]
    unsafe {
        let _ = abilitykit::runtime::context::OH_AbilityRuntime_Context_GetAreaMode(
            core::mem::zeroed(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-26")]
    unsafe {
        let _ = abilitykit::childprocess::OH_Ability_IsNativeChildProcessSupported();
        let _ = abilitykit::runtime::connect_options::OH_AbilityRuntime_CreateConnectOptions();
        let _ = abilitykit::runtime::modular_object_dispatcher::OH_AbilityRuntime_ModObjDispatcher_TypeInfoClear(ptr::null_mut());
        let _ = abilitykit::runtime::modular_object_extension_ability::OH_AbilityRuntime_ModObjExtensionAbility_RegisterOnCreateFunc(core::mem::zeroed(), core::mem::zeroed());
        let _ = abilitykit::runtime::modular_object_extension_context::OH_AbilityRuntime_ModObjExtensionContext_TerminateSelf(core::mem::zeroed());
        let _ = abilitykit::runtime::modular_object_extension_manager::OH_AbilityRuntime_AcquireSelfModularObjectExtensionInfos(ptr::null_mut());
        let _ = abilitykit::runtime::native_ability_wrapper::OH_AbilityRuntime_GetEnv(
            ptr::null(),
            ptr::null_mut(),
        );
        let _ =
            abilitykit::runtime::start_options::OH_AbilityRuntime_GetStartOptionsWindowModeValue(
                ptr::null_mut(),
                ptr::null_mut(),
            );
    }
}
