#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"

###############################################################################
### Clean up instrumentation-test-plugin.
###############################################################################

INSTRUMENTATION_TEST_PLUGIN_ROOT="./tests/instrumentation-test-plugin"
cd "$PROJECT_ROOT"
cd "$INSTRUMENTATION_TEST_PLUGIN_ROOT"

# Clean `yarn pack` output.
rm -rf ./artifacts

# Remove `rollup` output.
rm -rf ./dist

# Remove output generated from `privacy-helpers`.
rm -rf ./src/core/generated

###############################################################################
### Clean up the integration tests.
###############################################################################

cd "$PROJECT_ROOT"

for TEST_ROOT in ./tests/integration/*; do
  cd "$PROJECT_ROOT"
  cd "$TEST_ROOT"
  yarn clean
done
