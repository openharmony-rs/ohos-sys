#![cfg(feature = "api-10")]

use std::ptr;

use ohaudio_sys as ohaudio;

#[test]
fn link_smoke() {
    unsafe {
        let _ = ohaudio::audiocapturer::OH_AudioCapturer_Release(ptr::null_mut());
        let _ = ohaudio::audiorenderer::OH_AudioRenderer_Release(ptr::null_mut());

        let mut builder = ptr::null_mut();
        let _ = ohaudio::audiostreambuilder::OH_AudioStreamBuilder_Create(
            &mut builder,
            ohaudio::audiostream_base::OH_AudioStream_Type::AUDIOSTREAM_TYPE_RENDERER,
        );
    }

    #[cfg(feature = "api-11")]
    unsafe {
        let _ = ohaudio::audiorenderer::OH_AudioRenderer_GetSpeed(ptr::null_mut(), ptr::null_mut());
        let _ = ohaudio::audiostreambuilder::OH_AudioStreamBuilder_SetFrameSizeInCallback(
            ptr::null_mut(),
            0,
        );
    }

    #[cfg(feature = "api-12")]
    unsafe {
        let _ = ohaudio::audiocapturer::OH_AudioCapturer_GetOverflowCount(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ =
            ohaudio::audiorenderer::OH_AudioRenderer_GetVolume(ptr::null_mut(), ptr::null_mut());
        let _ = ohaudio::audiostreambuilder::OH_AudioStreamBuilder_SetRendererPrivacy(
            ptr::null_mut(),
            ohaudio::audiostream_base::OH_AudioStream_PrivacyType::AUDIO_STREAM_PRIVACY_TYPE_PUBLIC,
        );
        let _ = ohaudio::audio_device_base::OH_AudioDeviceDescriptor_GetDeviceRole(
            ptr::null_mut(),
            ptr::null_mut(),
        );

        let mut manager = ptr::null_mut();
        let _ = ohaudio::audio_manager::OH_GetAudioManager(&mut manager);

        let mut routing_manager = ptr::null_mut();
        let _ = ohaudio::audio_routing_manager::OH_AudioManager_GetAudioRoutingManager(
            &mut routing_manager,
        );

        let mut session_manager = ptr::null_mut();
        let _ = ohaudio::audio_session_manager::OH_AudioManager_GetAudioSessionManager(
            &mut session_manager,
        );
    }

    #[cfg(feature = "api-13")]
    unsafe {
        let _ = ohaudio::audio_routing_manager::OH_AudioRoutingManager_IsMicBlockDetectionSupported(
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-15")]
    unsafe {
        let _ = ohaudio::audiorenderer::OH_AudioRenderer_GetAudioTimestampInfo(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "api-19")]
    unsafe {
        let _ = ohaudio::audiostreambuilder::OH_AudioStreamBuilder_SetVolumeMode(
            ptr::null_mut(),
            ohaudio::audiostream_base::OH_AudioStream_VolumeMode::AUDIOSTREAM_VOLUMEMODE_SYSTEM_GLOBAL,
        );

        let mut stream_manager = ptr::null_mut();
        let _ = ohaudio::audio_stream_manager::OH_AudioManager_GetAudioStreamManager(
            &mut stream_manager,
        );
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = ohaudio::audiocapturer::OH_AudioCapturer_GetFastStatus(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ = ohaudio::audiorenderer::OH_AudioRenderer_GetFastStatus(
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let _ =
            ohaudio::audiostreambuilder::OH_AudioStreamBuilder_SetRendererWriteDataCallbackAdvanced(
                ptr::null_mut(),
                None,
                ptr::null_mut(),
            );
        let _ = ohaudio::audio_manager::OH_AudioManager_RegisterAudioSceneChangeCallback(
            ptr::null_mut(),
            None,
            ptr::null_mut(),
        );
        let _ = ohaudio::audio_session_manager::OH_AudioSessionManager_SetScene(
            ptr::null_mut(),
            ohaudio::audio_session_manager::OH_AudioSession_Scene::AUDIO_SESSION_SCENE_MEDIA,
        );
        let _ =
            ohaudio::audio_stream_manager::OH_AudioStreamManager_IsAcousticEchoCancelerSupported(
                ptr::null_mut(),
                ohaudio::audiostream_base::OH_AudioStream_SourceType::AUDIOSTREAM_SOURCE_TYPE_MIC,
                ptr::null_mut(),
            );

        let mut resource_manager = ptr::null_mut();
        let _ = ohaudio::audio_resource_manager::OH_AudioManager_GetAudioResourceManager(
            &mut resource_manager,
        );

        let mut volume_manager = ptr::null_mut();
        let _ = ohaudio::audio_volume_manager::OH_AudioManager_GetAudioVolumeManager(
            &mut volume_manager,
        );
    }

    #[cfg(feature = "api-21")]
    unsafe {
        let _ = ohaudio::audio_session_manager::OH_AudioSessionManager_GetAvailableDevices(
            ptr::null_mut(),
            ohaudio::audio_device_base::OH_AudioDevice_Usage::AUDIO_DEVICE_USAGE_MEDIA_OUTPUT,
            ptr::null_mut(),
        );
        let _ = ohaudio::audio_stream_manager::OH_AudioStreamManager_IsIntelligentNoiseReductionEnabledForCurrentDevice(
            ptr::null_mut(),
            ohaudio::audiostream_base::OH_AudioStream_SourceType::AUDIOSTREAM_SOURCE_TYPE_MIC,
        );
    }
}
