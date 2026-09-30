#!/usr/bin/env bash
# Usage: release_tag.sh <crate name> <version>
# Prints the git tag of a crate release: `v<version>` for ohos-sys, `<crate name>-v<version>` otherwise.

set -euo pipefail

if [[ "$1" == ohos-sys ]]; then
  echo "v$2"
else
  echo "$1-v$2"
fi
