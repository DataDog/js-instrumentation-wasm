#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

# Build TypeScript code and generate the files we'll actually distribute.
mkdir -p ./dist
yarn rollup -c

# Build bundled type definitions.
mkdir -p ./dist/types
yarn dts-buddy dist/types/index.d.ts \
  -m @datadog/js-instrumentation-wasm:src/index.ts

# Generate the packed plugin.
mkdir -p ./artifacts
yarn pack --out ./artifacts/%s.tgz
