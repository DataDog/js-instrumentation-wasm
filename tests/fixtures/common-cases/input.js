import { foo } from "my-module.js";

// const constant = "some string";
const constant = "some string";

foo({
  // bar: "abc",
  bar: "abc",
  // baz: `something${constant}456`,
  baz: `something${constant}456`,
  // bat: foo`123${constant}another thing`,
  bat: foo`123${constant}another thing`,
});
