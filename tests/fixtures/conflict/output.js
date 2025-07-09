import{$ as B}from'datadog:privacy-helpers.mjs';const A=B(["test"]);// Create conflicting bindings for the default names of the helpers.
const $ = 123;
const D = 456;
console.log(/* (attached comment) */ A[0]);
