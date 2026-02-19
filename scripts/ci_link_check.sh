#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${OHOS_SDK_NATIVE:-}" ]]; then
  echo "OHOS_SDK_NATIVE is not set. Point it at the SDK native directory." >&2
  exit 1
fi

TARGET="${LINK_CHECK_TARGET:-aarch64-unknown-linux-ohos}"
SDK_NATIVE="${OHOS_SDK_NATIVE}"
SYSROOT="${SDK_NATIVE}/sysroot"
CLANG="${SDK_NATIVE}/llvm/bin/${TARGET}-clang"
SDK_PACKAGE_JSON="${SDK_NATIVE}/oh-uni-package.json"

if [[ ! -d "${SYSROOT}" ]]; then
  echo "Missing sysroot at ${SYSROOT}." >&2
  exit 1
fi

if [[ ! -x "${CLANG}" ]]; then
  echo "Missing clang at ${CLANG}." >&2
  exit 1
fi

if [[ ! -f "${SDK_PACKAGE_JSON}" ]]; then
  echo "Missing SDK package metadata at ${SDK_PACKAGE_JSON}." >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to parse ${SDK_PACKAGE_JSON}." >&2
  exit 1
fi


if ! rustc --print target-list | grep -q "^${TARGET}$"; then
  echo "Rust target ${TARGET} is not installed. Install with: rustup target add ${TARGET}" >&2
  exit 1
fi

export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_LINKER="${CLANG}"
export RUSTFLAGS="-C link-arg=--sysroot=${SYSROOT}"

API_LEVEL="$(jq -r '.apiVersion // empty' "${SDK_PACKAGE_JSON}")"
if [[ -z "${API_LEVEL}" ]]; then
  echo "Unable to read api Level from ${SDK_PACKAGE_JSON}." >&2
  exit 1
fi
FEATURE_ARGS=(--features "api-${API_LEVEL}")

echo "Link check: building ohos-media-sys tests for ${TARGET} with SDK sysroot."
echo "Link check: using features: api-${API_LEVEL}"

cargo test --workspace --tests --no-run --target "${TARGET}" "${FEATURE_ARGS[@]}" --features all-components
