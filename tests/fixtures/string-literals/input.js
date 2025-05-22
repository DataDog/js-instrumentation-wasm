const foo = () => {};

// Trivial string literals.
const trivial1 = "";
const trivial2 = '';
const trivial3 = "appendix";
const trivial4 = 'bowling';

// Escape sequences.
const escape1 = "cat\r\n\tdog";
const escape2 = "egg'\"'";
const escape3 = 'fizz"\'"';
const escape4 = "gem`'\"\u{6F}";

// String literals used in expressions.
const expression1 = "hammer".toLowerCase();
const expression2 = foo["image"];
const expression3 = foo("jewel");
const expression4 = "karat"[1];

// String literals used in function declarations.
const func1 = (_a = "labor") => {};

// String literals used in object literals.
const object1 = { "macrame": 1 };
const object2 = { ["nanobot"]: 2 };
const object3 = { "observe"(_a){} };

// String literals used in array literals.
const array1 = ["pacific"];
const array2 = [...'quarrel'];

// Reuse of string literals we've already seen.
const reuse1 = "appendix";
const reuse2 = "karat"[1];
const reuse3 = { "observe"(_a){} };
const reuse4 = [...'quarrel'];
