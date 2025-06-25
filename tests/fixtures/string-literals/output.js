import { $ } from 'datadog:privacy-helpers.mjs';
const D = $([
  "appendix",
  "gem`'\"\u{6F}",
  "cat\r\n\tdog",
  "observe",
  'quarrel',
  'fizz"\'"',
  "karat",
  'bowling',
  "egg'\"'",
  "macrame",
  "nanobot",
  "pacific",
  "hammer",
  "image",
  "jewel",
  "labor",
]);
const foo = () => {};

// Trivial string literals.
const trivial1 = "";
const trivial2 = '';
const trivial3 = D[0];
const trivial4 = D[7];

// Escape sequences.
const escape1 = D[2];
const escape2 = D[8];
const escape3 = D[5];
const escape4 = D[1];

// String literals used in expressions.
const expression1 = D[12].toLowerCase();
const expression2 = foo[D[13]];
const expression3 = foo(D[14]);
const expression4 = D[6][1];

// String literals used in function declarations.
const func1 = (_a = D[15]) => {};

// String literals used in object literals.
const object1 = { [D[9]]: 1 };
const object2 = { [D[10]]: 2 };
const object3 = { [D[3]](_a){} };

// String literals used in array literals.
const array1 = [D[11]];
const array2 = [...D[4]];

// Reuse of string literals we've already seen.
const reuse1 = D[0];
const reuse2 = D[6][1];
const reuse3 = { [D[3]](_a){} };
const reuse4 = [...D[4]];
