const foo = 'foo';
const bar = 'bar';
const tag = (v) => v;

const multilineTemplateLiteral = `This is
  a multiline template literal
with several embedded newlines
    and inconsistent spacing.`;

const multilineTemplateLiteralWithExpressions = `This is
  a multiline template literal
with expressions ${foo} embedded here
    and ${bar} there.`;

const multilineTaggedTemplateLiteral = tag`This is
  a multiline template literal
with several embedded newlines
    and inconsistent spacing.`;

const multilineTaggedTemplateLiteralWithExpressions = tag`This is
  a multiline template literal
with expressions ${foo} embedded here
    and ${bar} there.`;
