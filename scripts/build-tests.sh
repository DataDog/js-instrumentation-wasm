#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"

###############################################################################
### Build instrumentation-test-plugin.
###############################################################################

INSTRUMENTATION_TEST_PLUGIN_ROOT="./tests/instrumentation-test-plugin"
cd "$PROJECT_ROOT"
cd "$INSTRUMENTATION_TEST_PLUGIN_ROOT"

# Note that this cannot be a immutable install, because the integration test's
# package.json references a tarball that we generate as part of the build with
# `yarn pack`, and by design, the content hash of this tarball will change.
export YARN_ENABLE_IMMUTABLE_INSTALLS="false"
yarn install

# Build TypeScript code and bundle.
yarn rollup -c

# Build bundled type definitions.
yarn dts-buddy dist/types/index.d.ts \
  -m @datadog/instrumentation-test-plugin:src/index.ts

# Generate the packed plugin.
yarn pack --out ./artifacts/%s.tgz
