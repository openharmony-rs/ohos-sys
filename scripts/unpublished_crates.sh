#!/usr/bin/env bash
# Prints `<name> <version>` for every publishable workspace package whose version is not on crates.io,
# in dependency order, i.e. in an order in which they can be published.

set -euo pipefail

index=$(mktemp)
trap 'rm -f "$index"' EXIT

metadata=$(cargo metadata --format-version 1 --no-deps)
packages=$(jq -r '.packages[] | select(.publish != []) | "\(.name) \(.version)"' <<< "$metadata")
declare -A versions
while read -r name version; do
  versions[$name]=$version
done <<< "$packages"

# A `tsort` input line `a b` means that `a` comes before `b`, and `a a` adds `a` without an edge.
order=$(jq -r '
    [.packages[] | select(.publish != []) | .name] as $publishable
    | .packages[] | select(.publish != []) | .name as $name
    | "\($name) \($name)",
      (.dependencies[] | select(.kind != "dev" and (.name | IN($publishable[]))) | "\(.name) \($name)")
  ' <<< "$metadata" | tsort)

for name in $order; do
  version=${versions[$name]}
  # Path of the package in the crates.io sparse index.
  lowercase=${name,,}
  case ${#lowercase} in
    1) path="1/${lowercase}" ;;
    2) path="2/${lowercase}" ;;
    3) path="3/${lowercase:0:1}/${lowercase}" ;;
    *) path="${lowercase:0:2}/${lowercase:2:2}/${lowercase}" ;;
  esac
  status=$(curl -sS -o "$index" -w '%{http_code}' -A 'ohos-sys release workflow' \
    "https://index.crates.io/${path}")
  case "$status" in
    200) ;;
    404) echo "::error::${name} is not on crates.io. Its first version has to be published manually." >&2; exit 1 ;;
    *) echo "::error::Unexpected crates.io index response for ${name}: HTTP ${status}" >&2; exit 1 ;;
  esac
  if ! jq -e --arg version "$version" 'select(.vers == $version)' "$index" > /dev/null; then
    echo "${name} ${version}"
  fi
done
