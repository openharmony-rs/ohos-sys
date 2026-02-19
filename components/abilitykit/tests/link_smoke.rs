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
}
