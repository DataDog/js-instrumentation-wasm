import{$}from'datadog:privacy-helpers.mjs';const D=$(["some string",`something`,"abc",$`123${0}another thing`]);import { foo } from "my-module.js";

// const constant = "some string";
const constant = D[0];

foo({
  // bar: "abc",
  bar: D[2],
  // baz: `something${constant}456`,
  baz: `${D[1]}${constant}456`,
  // bat: foo`123${constant}another thing`,
  bat: foo(D[3], constant),
});
