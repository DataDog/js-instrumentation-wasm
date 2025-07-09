import{$}from'datadog:privacy-helpers.mjs';const D=$(["abcd",`abcdefghi`,"abcde","abc",`abcdefgh`,"xyz0","xyz",$``,$`${0}${0}${0}`,"a","ab",`abcde`,`abcdef`,`abcdefg`]);const foo = () => {};

// For short strings, we should use the original string instead of replacing it with a
// dictionary reference.
const short1 = "";
const short2 = "a";
const short3 = "ab";

// Longer strings should be replaced with a dictionary reference.
const long1 = D[3];
const long2 = D[0];
const long3 = D[2];

// Similarly, we should keep short quasis in template literals instead of replacing them
// with a dictionary reference.
const quasi1 = `abcde${5}abcdef${6}abcdefg${7}${D[4]}${8}${D[1]}`;

// The dictionary now contains 10 items, which will make the references for any further
// items larger because the size of the index will increase by one character. However,
// we should still be able to use references for all of the examples below, because we
// should be smart enough to reorder the dictionary to make the new items fit.
const twoDigits1 = D[3];
const twoDigits2 = D[0];
const twoDigits3 = D[6];
const twoDigits4 = D[5];

// The same applies to quasis.
const twoDigitsQuasi1 = `abcde${5}abcdef${6}abcdefg${7}${D[4]}${8}${D[1]}`;

// We should never replace an empty string, or a sequence of empty quasis, with a
// dictionary reference. (Although for tagged templates, it's a bit trickier to detect
// this case, and so we will continue to generate dictionary entries for them.)
const empty1 = "";
const empty2 = "";
const empty3 = ``;
const empty4 = `${0}${1}${2}`;
const empty5 = foo(D[7]);
const empty6 = foo(D[8], 0, 1, 2);
