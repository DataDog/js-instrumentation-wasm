#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

# Clean `yarn pack` output.
rm -rf ./artifacts

# Remove `rollup` output.
rm -rf ./dist
