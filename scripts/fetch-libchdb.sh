#!/usr/bin/env bash
# Fetch the vendored libchdb.so for the current platform.
#
# The shared object is ~530MB so it is not committed to git. This script
# downloads the pinned chdb-core release and extracts libchdb.so + chdb.h
# into vendor/chdb/.
set -euo pipefail

VERSION="v26.5.0"
EXPECTED_SHA256="8dc5155d80b20a27d3e7322bb49ce003807ebd2b8cbd43896c829d16d24b0cdf"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dest="$repo_root/vendor/chdb"

# chdb-core only publishes prebuilt binaries for linux/macos on x86_64/aarch64.
case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)   asset="linux-x86_64-libchdb.tar.gz" ;;
  Linux-aarch64)  asset="linux-aarch64-libchdb.tar.gz"; EXPECTED_SHA256="" ;;
  Darwin-x86_64)  asset="macos-x86_64-libchdb.tar.gz"; EXPECTED_SHA256="" ;;
  Darwin-arm64)   asset="macos-arm64-libchdb.tar.gz"; EXPECTED_SHA256="" ;;
  *) echo "unsupported platform: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac

url="https://github.com/chdb-io/chdb-core/releases/download/$VERSION/$asset"

if [[ -f "$dest/libchdb.so" ]]; then
  echo "libchdb.so already present at $dest/libchdb.so; nothing to do."
  exit 0
fi

mkdir -p "$dest"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "Downloading $url ..."
curl -fSL "$url" -o "$tmp/libchdb.tar.gz"

if [[ -n "$EXPECTED_SHA256" ]]; then
  echo "$EXPECTED_SHA256  $tmp/libchdb.tar.gz" | sha256sum -c -
fi

tar xzf "$tmp/libchdb.tar.gz" -C "$tmp"
cp "$tmp/libchdb.so" "$dest/libchdb.so"
# Refresh headers too in case the release moved.
[[ -f "$tmp/chdb.h" ]]   && cp "$tmp/chdb.h" "$dest/chdb.h"
[[ -f "$tmp/chdb.hpp" ]] && cp "$tmp/chdb.hpp" "$dest/chdb.hpp"

echo "Installed libchdb.so to $dest/"
