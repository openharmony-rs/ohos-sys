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
    echo "Note: the NDK directory is the 'native' directory in the SDK"
    exit 1
fi

if [[ ! -d "${OHOS_NDK_HOME}" ]]
then
    echo "OHOS_NDK_HOME was set to '${OHOS_NDK_HOME}', which is not a valid directory"
    exit 1
fi

OHOS_SYSROOT_DIR="${OHOS_NDK_HOME}/sysroot"
if [[ ! -d "${OHOS_SYSROOT_DIR}" ]]
then
    echo "OpenHarmony sysroot not found under \${OHOS_NDK_HOME}/sysroot (${OHOS_NDK_HOME}/sysroot)"
    exit 1
fi

export LIBCLANG_PATH=${OHOS_NDK_HOME}/llvm/lib
export CLANG_PATH=${OHOS_NDK_HOME}/llvm/bin/clang

BASE_BINDGEN_ARGS=(--no-layout-tests --formatter=prettyplease)
BASE_BINDGEN_ARGS+=(--blocklist-file='.*stdint\.h' --blocklist-file='.*stddef\.h')
BASE_BINDGEN_ARGS+=(--blocklist-file='.*stdarg\.h' --blocklist-file='.*stdbool\.h')
BASE_BINDGEN_ARGS+=(--blocklist-file='.*/std[a-z]{3,4}\.h' --blocklist-file='.*/__std[a-z_]+\.h')
BASE_BINDGEN_ARGS+=(--blocklist-item='__(BYTE_ORDER|LONG_MAX|LITTLE_ENDIAN|BIG_ENDIAN|USE_TIME_BITS64)')
BASE_BINDGEN_ARGS+=(--blocklist-item='u?intmax_t')
BASE_BINDGEN_ARGS+=(--use-core --raw-line="#![allow(non_upper_case_globals)]")
BASE_BINDGEN_ARGS+=(--raw-line="#![allow(non_camel_case_types)]" --raw-line="#![allow(non_snake_case)]")

# TODO: How to detect / deal with target specific bindings
BASE_CLANG_ARGS=("--sysroot=${OHOS_SYSROOT_DIR}")
BASE_CLANG_ARGS+=(--target=aarch64-linux-ohos)


bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --output "${ROOT_DIR}/src/hilog.rs" \
    --raw-line='' \
    --raw-line='#[link(name="hilog_ndk.z")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    "${OHOS_SYSROOT_DIR}/usr/include/hilog/log.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --raw-line='' \
    --raw-line='#[link(name="ace_ndk.z")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    --output "${ROOT_DIR}/src/xcomponent.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/ace/xcomponent/native_interface_xcomponent.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

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
    -- \
    "${BASE_CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --bitfield-enum 'OH_NativeBuffer_Usage' \
    --raw-line='' \
    --raw-line='#[link(name="native_buffer")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    --default-enum-style=moduleconsts \
    --no-derive-copy \
    --output "${ROOT_DIR}/src/native_buffer.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/native_buffer/native_buffer.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --raw-line='' \
    --raw-line='#[link(name="ace_ndk.z")]' \
    --raw-line='#[link(name="native_window")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    --default-enum-style=moduleconsts \
    --no-derive-copy \
    --output "${ROOT_DIR}/src/native_window.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/native_window/external_window.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

cargo fmt
