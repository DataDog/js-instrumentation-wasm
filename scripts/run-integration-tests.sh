#!/bin/sh
set -e

PROJECT_ROOT="$(git rev-parse --show-toplevel)"
cd "$PROJECT_ROOT"

FAILED_TESTS=""

for TEST_ROOT in ./tests/integration/*; do
 cd "$PROJECT_ROOT"
 cd "$TEST_ROOT"

 # Note that this cannot be a immutable install, because the integration test's
 # package.json references a tarball that we generate as part of the build with
 # `yarn pack`, and by design, the content hash of this tarball will change.
 export YARN_ENABLE_IMMUTABLE_INSTALLS="false"
 yarn install

 yarn build
 
 # Run tests but don't exit on failure
 if ! yarn test; then
   FAILED_TESTS="$FAILED_TESTS\n  - $(basename $TEST_ROOT)"
 fi
done

# Report results
if [ -n "$FAILED_TESTS" ]; then
  echo "\n❌ Some integration tests failed:$FAILED_TESTS"
  exit 1
else
  echo "\n✅ All integration tests passed!"
fi
