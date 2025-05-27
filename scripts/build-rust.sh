#!/bin/sh
set -e

if [ "$1" = "release" ] || [ "$1" = "" ]; then
  MODE="release"
elif [ "$1" = "debug" ]; then
  MODE="debug"
  echo "WARNING: Debug builds easily overflow the stack. This may cause some tests to fail."
else
  echo "Unknown build mode: $1"
  exit 1
fi

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

# Build the Rust code and generate the WASM component.
echo "Running wasm-pack in $MODE mode"
RUST_ENTRY_POINT="rust/datadog-js-instrumentation"
if [ "$MODE" = "debug" ]; then
  wasm-pack build --dev --target web "$RUST_ENTRY_POINT"
else
  wasm-pack build --target web "$RUST_ENTRY_POINT"
fi
