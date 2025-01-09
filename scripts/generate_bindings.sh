#!/usr/bin/env bash
# Deprecated script. Use the generator written in Rust instead.

set -eux

ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )"/.. &> /dev/null && pwd )

(cd "${ROOT_DIR}/scripts/generator" && cargo run )

cd "${ROOT_DIR}"
cargo +nightly fmt
