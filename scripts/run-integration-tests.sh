#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

for TEST_ROOT in ./tests/integration/*; do
 cd "$PROJECT_ROOT"
 cd "$TEST_ROOT"

 # Note that this cannot be a immutable install, because the integration test's
 # package.json references a tarball that we generate as part of the build with
 # `yarn pack`, and by design, the content hash of this tarball will change.
 yarn install

 yarn build
 yarn test
done
