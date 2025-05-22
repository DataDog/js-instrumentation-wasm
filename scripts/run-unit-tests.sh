#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

./scripts/run-unit-tests-rust.sh
./scripts/run-unit-tests-js.sh
