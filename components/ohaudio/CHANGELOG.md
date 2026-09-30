# Changelog

## 0.1.1

- Add API-24, API-25 and API-26 bindings.
- Add the `audio_accessory_common`, `audio_accessory_input_stream_manager`, `audio_accessory_manager`,
  `audio_debugging_manager` and `audio_device_enhance_manager` modules (API-26).
- `OH_AudioSession_ConcurrencyMode` and `OH_AudioSession_Strategy` moved to the new
  `audio_session_base` module, and are re-exported from `audio_session_manager`.
