import { $ } from 'datadog:privacy-helpers.mjs';
const D = $([
  $`bar${0}`,
]);
function foo() { }
console.log(foo(D[0], 1000))
