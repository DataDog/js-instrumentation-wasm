#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

# Set an environment variable that signals that we're updating snapshots.
# The various build scripts and tests take this into account.
export UPDATING_SNAPSHOTS=true

# Update snapshots for the JS unit tests.
UNIT_TEST_ROOT="./tests/unit"
cd "$PROJECT_ROOT"
cd "$UNIT_TEST_ROOT"
yarn vitest --update --run

# Update snapshots for the integration tests.
cd "$PROJECT_ROOT"
for TEST_ROOT in ./tests/integration/*; do
 cd "$PROJECT_ROOT"
 cd "$TEST_ROOT"

 # Note that this cannot be a immutable install, because the integration test's
 # package.json references a tarball that we generate as part of the build with
 # `yarn pack`, and by design, the content hash of this tarball will change.
 export YARN_ENABLE_IMMUTABLE_INSTALLS="false"
 yarn install

 yarn build
 yarn update:snapshots
done
