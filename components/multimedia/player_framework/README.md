# OpenHarmony Media Player framework bindings

Provides bindings to the Media subsystem on OpenHarmony allowing Audio and Video playback and more.
These bindings are under construction.

C API reference: https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-media-kit/capi-avplayer.md

## Blocklisted bindings

Some SDK types are not yet available in this workspace, so the following bindings are blocklisted in
`scripts/generator/src/dir_conf.rs` to keep the crate compiling:

- `OH_AudioInterrupt_ForceType`, `OH_AudioInterrupt_Hint`, `OH_AudioStream_DeviceChangeReason`
  - Affects low-power audio sink interrupt/device change callbacks.
- `OH_LowPowerAudioSink_OnInterrupted`, `OH_LowPowerAudioSink_OnDeviceChanged`
  - Dependent on the missing audio interrupt types above.
- `OH_LowPowerAudioSinkCallback_SetInterruptListener`, `OH_LowPowerAudioSinkCallback_SetDeviceChangeListener`
  - Callback registrations that depend on the missing audio interrupt types.
- `OH_MediaAsset`, `OH_AVRecorder_OnUri`, `OH_AVRecorder_SetUriCallback`
  - Media library types are not yet bound in this repository.

When the missing dependencies are added (e.g. ohaudio and media library bindings), these blocklists
should be removed and the bindings regenerated.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
