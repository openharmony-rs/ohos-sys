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

BASE_BINDGEN_ARGS=(--no-layout-tests --formatter=prettyplease)
BASE_BINDGEN_ARGS+=(--use-core --raw-line="#![allow(non_upper_case_globals)]")
BASE_BINDGEN_ARGS+=(--raw-line="#![allow(non_camel_case_types)]" --raw-line="#![allow(non_snake_case)]")


bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --output "${ROOT_DIR}/src/ffi.rs" \
    --allowlist-file="${OHOS_SYSROOT_DIR}/usr/include/hitrace/trace.h" \
    --raw-line='' \
    --raw-line='#[link(name="hitrace_ndk.z")]' \
    --raw-line='extern "C" {}' \
    --raw-line='' \
    "${OHOS_SYSROOT_DIR}/usr/include/hitrace/trace.h"

cargo fmt