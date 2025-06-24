const { $ } = require('datadog:privacy-helpers.cjs');
const D = $([
  'test',
]);
const foo = require('foo-module');
foo(D[0]);
