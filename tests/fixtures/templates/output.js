import{$}from'datadog:privacy-helpers.mjs';const D=$([`\`'"\u{6F}`,`abc\r\n\t123`,`gallingly`,`hairbrush`,`idealists`,`absolute`,`backdrops`,`cacophony`,`dachshund`,`eagerness`,`fabricate`,`newlyweds`,`orthodoxy`,`paralyzed`,`quadrants`,`reflexive`,`waferlike`,`yardstick`,$`superior${0}vertical`,$`triangle${0}umbrella`,$``,$`judgment`,$`keyboard${0}laughter${0}material`,$`abstract`,$`doubtful${0}endeavor`,$`bacteria${0}contrast`]);const foo = () => {};
const bar = 1;

// Trivial template expressions.
const trivial1 = ``;
const trivial2 = `${D[5]}`;
const trivial3 = `${D[6]}${bar}`;
const trivial4 = `${bar}${D[7]}`;
const trivial5 = `${D[8]}${bar}`;
const trivial6 = `${D[9]}${bar}${D[10]}`;
const trivial7 = `${D[2]}${bar}${D[3]}${bar}${D[4]}${bar}`;

// Escape sequences.
const escape1 = `${D[1]}`;
const escape2 = `${D[0]}`;

// Tagged template expressions.
const tagged1 = foo(D[20]);
const tagged2 = foo(D[21]);
const tagged3 = foo(D[22], bar, bar);

// Nested template expressions.
const nested1 = `${D[11]}${`${D[12]}${`${D[13]}`}${D[14]}`}${D[15]}`;
const nested2 = foo(D[18], foo(D[19], bar));
const nested3 = `${D[16]}${bar}${D[17]}${foo(D[23])}`;

// Using the results of template expressions as a tag.
const resulttag1 = foo(D[25], bar)(D[24], bar);

// Reuse of quasis and template expressions that we've already seen.
const reuse1 = `${D[5]}`;
const reuse2 = `${D[2]}${bar}${D[3]}${bar}${D[4]}${bar}`;
const reuse3 = `${D[0]}`;
const reuse4 = foo(D[18], foo(D[19], bar));
