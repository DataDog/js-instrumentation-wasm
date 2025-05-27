#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

# Remove `cargo build` output.
cargo clean --manifest-path ./rust/Cargo.toml

# Remove `wasm-pack` output.
rm -rf ./rust/datadog-js-instrumentation/pkg
