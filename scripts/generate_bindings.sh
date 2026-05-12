#!/usr/bin/env bash
# Deprecated script. Use the generator written in Rust instead.
# The generator also runs `cargo +nightly fmt` and applies any patches in
# `scripts/generator/patches/`.

set -eux

ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )"/.. &> /dev/null && pwd )

(cd "${ROOT_DIR}/scripts/generator" && cargo run )
