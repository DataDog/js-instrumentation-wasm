const foo = () => {};

// For short strings, we should use the original string instead of replacing it with a
// dictionary reference.
const short1 = "";
const short2 = "a";
const short3 = "ab";

// Longer strings should be replaced with a dictionary reference.
const long1 = "abc";
const long2 = "abcd";
const long3 = "abcde";

// Similarly, we should keep short quasis in template literals instead of replacing them
// with a dictionary reference.
const quasi1 = `abcde${5}abcdef${6}abcdefg${7}abcdefgh${8}abcdefghi`;

// The dictionary now contains 10 items, which will make the references for any further
// items larger because the size of the index will increase by one character. However,
// we should still be able to use references for all of the examples below, because we
// should be smart enough to reorder the dictionary to make the new items fit.
const twoDigits1 = "abc";
const twoDigits2 = "abcd";
const twoDigits3 = "xyz";
const twoDigits4 = "xyz0";

// The same applies to quasis.
const twoDigitsQuasi1 = `abcde${5}abcdef${6}abcdefg${7}abcdefgh${8}abcdefghi`;

// We should never replace an empty string, or a sequence of empty quasis, with a
// dictionary reference. (Although for tagged templates, it's a bit trickier to detect
// this case, and so we will continue to generate dictionary entries for them.)
const empty1 = "";
const empty2 = "";
const empty3 = ``;
const empty4 = `${0}${1}${2}`;
const empty5 = foo``;
const empty6 = foo`${0}${1}${2}`;
