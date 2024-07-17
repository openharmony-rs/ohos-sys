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

OHOS_API_VERSION=$(jq '.apiVersion' -j < "${OHOS_NDK_HOME}/oh-uni-package.json")
echo "Generating bindings for API version ${OHOS_API_VERSION}"

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
# So our wrapper headers can detect the API version (and conditionally include more header files)
BASE_CLANG_ARGS+=("-DOHOS_SYS_API_LEVEL=${OHOS_API_VERSION}")


bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --output "${ROOT_DIR}/src/hilog/hilog_api${OHOS_API_VERSION}.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/hilog/log.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --no-copy="^OH_NativeXComponent$" \
    --no-copy="^OH_NativeXComponent_KeyEvent$" \
    --no-debug="^OH_NativeXComponent$" \
    --no-debug="^OH_NativeXComponent_KeyEvent$" \
    --output "${ROOT_DIR}/src/xcomponent/xcomponent_api${OHOS_API_VERSION}.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/ace/xcomponent/native_interface_xcomponent.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

NAPI_NOCOPY_STRUCTS=(napi_env__ napi_value__ napi_ref__ napi_handle_scope__ napi_escapable_handle_scope__ )
NAPI_NOCOPY_STRUCTS+=(napi_callback_info__ napi_deferred__ napi_callback_scope__ napi_async_context__ napi_async_work__)
NAPI_NOCOPY_STRUCTS+=(napi_threadsafe_function__ napi_async_cleanup_hook_handle__ uv_loop_s napi_module)
NAPI_NOCOPY_ARGS=()
for val in "${NAPI_NOCOPY_STRUCTS[@]}"; do
    NAPI_NOCOPY_ARGS+=("--no-copy=^${val}\$" "--no-debug=^${val}\$")
done

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --blocklist-file '.*/stdarg\.h' \
    --blocklist-file '.*stddef\.h' \
    --blocklist-file '.*/stdbool\.h' \
    "${NAPI_NOCOPY_ARGS[@]}" \
    --output "${ROOT_DIR}/src/napi/napi_api${OHOS_API_VERSION}.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/napi/native_api.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --bitfield-enum 'OH_NativeBuffer_Usage' \
    --blocklist-item '_LIBCPP_.*' \
    --default-enum-style=moduleconsts \
    --no-copy '^OH_NativeBuffer$'  \
    --no-debug '^OH_NativeBuffer$'  \
    --output "${ROOT_DIR}/src/native_buffer/native_buffer_api${OHOS_API_VERSION}.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/native_buffer/native_buffer.h" \
    -- \
    -x c++ \
    "${BASE_CLANG_ARGS[@]}"

# NativeWindowOperation has wrong documentation for one of the parameters in API 10.
block_native_window_operation=""
if [[ ${OHOS_API_VERSION} -eq 10 ]]; then
    block_native_window_operation='--blocklist-item=^NativeWindowOperation$'
fi

bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=moduleconsts \
    --no-derive-copy \
    ${block_native_window_operation} \
    --output "${ROOT_DIR}/src/native_window/native_window_api${OHOS_API_VERSION}.rs" \
    "${OHOS_SYSROOT_DIR}/usr/include/native_window/external_window.h" \
    -- \
    "${BASE_CLANG_ARGS[@]}"

# API-10
DRAWING_NOCOPY_STRUCTS=(OH_Drawing_Canvas OH_Drawing_Pen OH_Drawing_Brush OH_Drawing_Path OH_Drawing_Bitmap)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_FontCollection OH_Drawing_Typography OH_Drawing_TextStyle OH_Drawing_TypographyStyle)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_TypographyCreate)
# API-11
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_Point OH_Drawing_Rect OH_Drawing_RoundRect OH_Drawing_ShaderEffect OH_Drawing_Filter)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_MaskFilter OH_Drawing_ColorFilter OH_Drawing_Font OH_Drawing_Typeface)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_TextBlob OH_Drawing_TextBlobBuilder OH_Drawing_TextBox OH_Drawing_PositionAndAffinity)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_Range OH_Drawing_Matrix OH_Drawing_RunBuffer)
# API-12
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_Region OH_Drawing_PixelMap OH_Drawing_ColorSpace OH_Drawing_Point2D OH_Drawing_Point3D)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_PathEffect OH_Drawing_ShadowLayer OH_Drawing_MemoryStream OH_Drawing_Image )
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_ImageFilter OH_Drawing_SamplingOptions OH_Drawing_GpuContext OH_Drawing_Surface)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_FontMgr OH_Drawing_FontStyleSet OH_Drawing_BitmapFormat OH_Drawing_FontParser)
DRAWING_NOCOPY_STRUCTS+=(OH_Drawing_TextShadow)
# Maybe: OH_Drawing_StrutStyle

DRAWING_NOCOPY_ARGS=()
for val in "${DRAWING_NOCOPY_STRUCTS[@]}"; do
    DRAWING_NOCOPY_ARGS+=("--no-copy=^${val}\$" "--no-debug=^${val}\$")
done

# Some drawing headers are not valid C, so we need to use libclang in c++ mode.
# Note: block-listing `^std_.*` doesn't seem to work, perhaps the underscore replaces some other character.
bindgen "${BASE_BINDGEN_ARGS[@]}" \
    --default-enum-style=newtype \
    --blocklist-file '.*cstddef.*' \
    --blocklist-file '.*pthread.*' \
    --blocklist-item '^OH_NativeBuffer$' \
    --blocklist-item '_LIBCPP_.*' \
    --blocklist-item '__cpp_.*' \
    --blocklist-item '^std.*' \
    "${DRAWING_NOCOPY_ARGS[@]}" \
    --output "${ROOT_DIR}/src/drawing/drawing_api${OHOS_API_VERSION}.rs" \
    "${ROOT_DIR}/wrappers/drawing_wrapper.h" \
    -- "${BASE_CLANG_ARGS[@]}" \
    -x c++ \
    -include stdbool.h \
    -include stddef.h \
    -include stdint.h

cargo fmt
fd -e rs . 'src/' --exec rustfmt
