#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

./scripts/build-rust.sh "$1"
./scripts/build-js.sh "$1"
./scripts/build-tests.sh "$1"
