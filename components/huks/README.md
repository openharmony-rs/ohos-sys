# OpenHarmony Universal Keystore Kit (HUKS) bindings

Raw FFI bindings to the OpenHarmony Universal KeyStore (`libhuks_ndk.z.so`) —
HUKS, the system service for hardware-backed key management and cryptographic
operations. Keys managed by HUKS can be imported by applications or generated
by calling the HUKS APIs; private key material does not leave the keystore.

Available since OpenHarmony API-level 9. This crate's minimum is API-level 10
to match the rest of `ohos-sys`.

C API reference: https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-universal-keystore-kit/_huks_key_api.md

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
