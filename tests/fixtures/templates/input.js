const foo = () => {};
const bar = 1;

// Trivial template expressions.
const trivial1 = ``;
const trivial2 = `absolute`;
const trivial3 = `backdrops${bar}`;
const trivial4 = `${bar}cacophony`;
const trivial5 = `dachshund${bar}`;
const trivial6 = `eagerness${bar}fabricate`;
const trivial7 = `gallingly${bar}hairbrush${bar}idealists${bar}`;

// Escape sequences.
const escape1 = `abc\r\n\t123`;
const escape2 = `\`'"\u{6F}`;

// Tagged template expressions.
const tagged1 = foo``;
const tagged2 = foo`judgment`;
const tagged3 = foo`keyboard${bar}laughter${bar}material`;

// Nested template expressions.
const nested1 = `newlyweds${`orthodoxy${`paralyzed`}quadrants`}reflexive`;
const nested2 = foo`superior${foo`triangle${bar}umbrella`}vertical`;
const nested3 = `waferlike${bar}yardstick${foo`abstract`}`;

// Using the results of template expressions as a tag.
const resulttag1 = foo`bacteria${bar}contrast``doubtful${bar}endeavor`;

// Reuse of quasis and template expressions that we've already seen.
const reuse1 = `absolute`;
const reuse2 = `gallingly${bar}hairbrush${bar}idealists${bar}`;
const reuse3 = `\`'"\u{6F}`;
const reuse4 = foo`superior${foo`triangle${bar}umbrella`}vertical`;
