#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

./scripts/clean-rust.sh
./scripts/clean-js.sh
./scripts/clean-tests.sh
