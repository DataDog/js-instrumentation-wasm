#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

UNIT_TEST_ROOT="./tests/unit"
cd "$UNIT_TEST_ROOT"

# Note that this cannot be a immutable install, because the integration test's
# package.json references a tarball that we generate as part of the build with
# `yarn pack`, and by design, the content hash of this tarball will change.
export YARN_ENABLE_IMMUTABLE_INSTALLS="false"
yarn install

yarn vitest --run
