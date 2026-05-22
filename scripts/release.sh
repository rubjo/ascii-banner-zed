#!/bin/bash
set -e

cd "$(dirname "$0")/.."

if [ $# -eq 0 ]; then
  target="$(rustc -vV | grep host | awk '{print $2}')"
else
  target="$1"
fi

echo "Building for target: $target"

cargo build --release --target "$target" --manifest-path ascii-banner-lsp/Cargo.toml

cd "ascii-banner-lsp/target/$target/release"

if [[ "$target" == *"-windows-"* ]]; then
  archive="zip"
  tar czf "ascii-banner-lsp-${target}.tar.gz" ascii-banner-lsp.exe
else
  tar czf "ascii-banner-lsp-${target}.tar.gz" ascii-banner-lsp
fi

echo "Created ascii-banner-lsp-${target}.tar.gz"
