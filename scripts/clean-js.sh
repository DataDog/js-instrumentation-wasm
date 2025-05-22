#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

# Clean `yarn pack` output.
rm -rf ./artifacts
mkdir -p ./artifacts

# Remove `rollup` output.
rm -rf ./dist
mkdir -p ./dist
mkdir -p ./dist/types

# Remove `wasm-pack` output.
rm -rf ./rust/datadog-js-instrumentation/pkg

# Remove `privacy-helpers` output.
rm -rf ./src/generated
mkdir -p ./src/generated
