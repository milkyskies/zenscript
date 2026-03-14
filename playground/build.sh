#!/usr/bin/env bash
set -euo pipefail

# Build the WASM package for the playground
# Requires: wasm-pack (cargo install wasm-pack)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_DIR="$SCRIPT_DIR/../crates/floe-wasm"
OUT_DIR="$SCRIPT_DIR/pkg"

echo "Building floe-wasm..."
wasm-pack build "$CRATE_DIR" --target web --out-dir "$OUT_DIR"

echo "Done! Open playground/index.html in a browser."
