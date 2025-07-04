const tag = () => { };

// Should be able to exclude any kind of string with an exclude-line.
console.log(
  'not excluded',
  "exclude line 1", // datadog-privacy-allowlist-exclude-line
  'not excluded',
  'exclude line 2', // datadog-privacy-allowlist-exclude-line
  'not excluded',
  `exclude line 3`, // datadog-privacy-allowlist-exclude-line
  'not excluded',
  tag`exclude line 4`, // datadog-privacy-allowlist-exclude-line
  'not excluded',
);

// Block comments should also work.
console.log(
  'not excluded',
  "block 1", /* datadog-privacy-allowlist-exclude-line */
  'not excluded',
  'block 2', /* datadog-privacy-allowlist-exclude-line */
  'not excluded',
  `block 3`, /* datadog-privacy-allowlist-exclude-line */
  'not excluded',
  tag`block 4`, /* datadog-privacy-allowlist-exclude-line */
  'not excluded',
);

// Prefixed block comments should also work.
console.log(
  'not excluded',
  /* datadog-privacy-allowlist-exclude-line */ "prefixed block 1",
  'not excluded',
  /* datadog-privacy-allowlist-exclude-line */ 'prefixed block 2',
  'not excluded',
  /* datadog-privacy-allowlist-exclude-line */ `prefixed block 3`,
  'not excluded',
  /* datadog-privacy-allowlist-exclude-line */ tag`prefixed block 4`,
  'not excluded',
);

// Multiline block comments should also work.
console.log(
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-line
   */ "multiline block 1",
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-line
   */ 'multiline block 2',
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-line
   */ `multiline block 3`,
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-line
   */ tag`multiline block 4`,
  'not excluded',
);

// Should be able to exclude any kind of string with an exclude-next-line.
console.log(
  'not excluded',
  // datadog-privacy-allowlist-exclude-next-line
  "exclude next line 1",
  'not excluded',
  // datadog-privacy-allowlist-exclude-next-line
  'exclude next line 2',
  'not excluded',
  // datadog-privacy-allowlist-exclude-next-line
  `exclude next line 3`,
  'not excluded',
  // datadog-privacy-allowlist-exclude-next-line
  tag`exclude next line 4`,
  'not excluded',
);

// Block comments should also work for exclude-next-line.
console.log(
  'not excluded',
  /* datadog-privacy-allowlist-exclude-next-line */
  "exclude next line block 1",
  'not excluded',
  /* datadog-privacy-allowlist-exclude-next-line */
  'exclude next line block 2',
  'not excluded',
  /* datadog-privacy-allowlist-exclude-next-line */
  `exclude next line block 3`,
  'not excluded',
  /* datadog-privacy-allowlist-exclude-next-line */
  tag`exclude next line block 4`,
  'not excluded',
);

// Multiline block comments should also work for exclude-next-line.
console.log(
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-next-line
   */
  "exclude next line multiline block 1",
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-next-line
   */
  'exclude next line multiline block 2',
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-next-line
   */
  `exclude next line multiline block 3`,
  'not excluded',
  /*
   datadog-privacy-allowlist-exclude-next-line
   */
  tag`exclude next line multiline block 4`,
  'not excluded',
);

// We should be able to exclude a range of lines.
console.log(
  'not excluded',
  // datadog-privacy-allowlist-exclude-begin
  "exclude range 1",
  'exclude range 2',
  `exclude range 3`,
  tag`exclude range 4`,
  // datadog-privacy-allowlist-exclude-end
  'not excluded',
);

// We should be able to exclude a range of with a block comment.
console.log(
  'not excluded',
  /* datadog-privacy-allowlist-exclude-begin */
  "exclude range with block comment 1",
  'exclude range with block comment 2',
  `exclude range with block comment 3`,
  tag`exclude range with block comment 4`,
  /* datadog-privacy-allowlist-exclude-end */
  'not excluded',
);

// Extra 'exclude-begin' directives inside an exclusion and 'exclude-end'
// directives outside of an exclusion should have no effect.
console.log(
  'not excluded',
  // datadog-privacy-allowlist-exclude-end
  'not excluded',
  // datadog-privacy-allowlist-exclude-begin
  "exclude range with extra directives 1",
  'exclude range with extra directives 2',
  // datadog-privacy-allowlist-exclude-begin
  `exclude range with extra directives 3`,
  tag`exclude range with extra directives 4`,
  // datadog-privacy-allowlist-exclude-end
  'not excluded',
);

// An unterminated 'exclude-begin' should cover the rest of the file. (And extra
// 'exclude-begins' after that point should be ignored.)
console.log(
  'not excluded',
  /* datadog-privacy-allowlist-exclude-begin */
  "exclude range with unterminated comment 1",
  'exclude range with unterminated comment 2',
  /* datadog-privacy-allowlist-exclude-begin */
  `exclude range with unterminated comment 3`,
  tag`exclude range with unterminated comment 4`,
);
