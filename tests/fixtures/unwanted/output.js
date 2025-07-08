// No strings in this file should appear in the dictionary.
"use strict";
"use server";
"use client";
import "unwanted1.js";
import foo from "unwanted2.js";
import { foo } from "unwanted3.js";
import * as foo from "unwanted4.js";
import { foo as bar1 } from "unwanted5.js";
import { default as bar2 } from "unwanted6.js";
import { "unwanted7" as bar3 } from "unwanted8.js";
export { bar1 };
export { bar1 as bar2 };
export { bar1 as "unwanted9" };
export * from "unwanted10.js";
export * as foo from "unwanted11.js";
export { bar } from "unwanted12.js";
export { default } from "unwanted13.js";
export { default as bar } from "unwanted14.js";
export { foo as "unwanted15" } from "unwanted16.js";
const bar4 = require("unwanted17.js");
const bar5 = import("unwanted18.js");
function doSomething() {
  "use strict";
  return 0;
}
const regex = new RegExp('foo');
const func = new Function('foo');
