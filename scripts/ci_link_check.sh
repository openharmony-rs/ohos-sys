#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${OHOS_SDK_NATIVE:-}" ]]; then
  echo "OHOS_SDK_NATIVE is not set. Point it at the SDK native directory." >&2
  exit 1
fi

TARGET="aarch64-unknown-linux-ohos"
SDK_NATIVE="${OHOS_SDK_NATIVE}"
SYSROOT="${SDK_NATIVE}/sysroot"
CLANG="${SDK_NATIVE}/llvm/bin/${TARGET}-clang"

if [[ ! -d "${SYSROOT}" ]]; then
  echo "Missing sysroot at ${SYSROOT}." >&2
  exit 1
fi

if [[ ! -x "${CLANG}" ]]; then
  echo "Missing clang at ${CLANG}." >&2
  exit 1
fi


if ! rustc --print target-list | rg -q "^${TARGET}$"; then
  echo "Rust target ${TARGET} is not installed. Install with: rustup target add ${TARGET}" >&2
  exit 1
fi

export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_LINKER="${CLANG}"
export RUSTFLAGS="-C link-arg=--sysroot=${SYSROOT}"

echo "Link check: building ohos-media-sys tests for ${TARGET} with SDK sysroot."
cargo test -p ohos-media-sys --tests --no-run --target "${TARGET}" --all-features
