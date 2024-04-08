#!/usr/bin/env bash

set -eux

ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )"/.. &> /dev/null && pwd )

if ! command -v bindgen
then
    echo "Error: bindgen not found"
    exit 1
fi

if [[ -z "${OHOS_NDK_HOME}" ]]
then
    echo "OHOS_NDK_HOME was not set. Please set it to the base directory of the OHOS NDK"
    exit 1
fi

if [[ ! -d "${OHOS_NDK_HOME}" ]]
then
    echo "OHOS_NDK_HOME was set to '${OHOS_NDK_HOME}', which is not a valid directory"
    exit 1
fi

OHOS_SYSROOT_DIR="${OHOS_NDK_HOME}/native/sysroot"
if [[ ! -d "${OHOS_SYSROOT_DIR}" ]]
then
    echo "OpenHarmony sysroot not found under \${OHOS_NDK_HOME}/native/sysroot (${OHOS_NDK_HOME}/native/sysroot)"
    exit 1
fi

BASE_BINDGEN_ARGS=(--no-layout-tests --formatter=prettyplease --blocklist-file='/usr/include/.*')
BASE_BINDGEN_ARGS+=(--blocklist-file='.*stdint\.h' --blocklist-file='.*stddef\.h')
BASE_BINDGEN_ARGS+=(--blocklist-file='.*stdarg\.h' --blocklist-file='.*stdbool\.h')
BASE_BINDGEN_ARGS+=(--use-core --raw-line="#![allow(non_upper_case_globals)]")
BASE_BINDGEN_ARGS+=(--raw-line="#![allow(non_camel_case_types)]" --raw-line="#![allow(non_snake_case)]")
CLANG_ARGS=("--sysroot=${OHOS_SYSROOT_DIR}")

# TODO: How to detect / deal with target specific bindings
CLANG_ARGS+=("-I${OHOS_SYSROOT_DIR}/usr/include/aarch64-linux-ohos/")


bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --output "${ROOT_DIR}/src/hilog.rs" \
    --raw-line='' \
    --raw-line='#[link(name="hilog_ndk.z")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    "${OHOS_SYSROOT_DIR}/usr/include/hilog/log.h" \
    -- "${CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --raw-line='' \
    --raw-line='#[link(name="ace_ndk.z")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    --output "${ROOT_DIR}/src/ace/xcomponent/native_interface_xcomponent.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/ace/xcomponent/native_interface_xcomponent.h" \
    -- "${CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --blocklist-file '.*/stdarg\.h' \
    --blocklist-file '.*stddef\.h' \
    --blocklist-file '.*/stdbool\.h' \
    --raw-line='' \
    --raw-line='#[link(name="ace_napi.z")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    --output "${ROOT_DIR}/src/napi.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/napi/native_api.h" \
    -- "${CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --bitfield-enum 'OH_NativeBuffer_Usage' \
    --default-enum-style=moduleconsts \
    --no-derive-copy \
    --output "${ROOT_DIR}/src/native_window.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/native_window/external_window.h" \
    -- "${CLANG_ARGS[@]}"

# cargo fmt