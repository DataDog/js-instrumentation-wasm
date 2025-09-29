import{$}from'datadog:privacy-helpers.mjs';const D=$([`This is
  a multiline template literal
with several embedded newlines
    and inconsistent spacing.`,`This is
  a multiline template literal
with expressions `,` embedded here
    and `,'foo','bar',$`This is
  a multiline template literal
with several embedded newlines
    and inconsistent spacing.`,$`This is
  a multiline template literal
with expressions ${0} embedded here
    and ${0} there.`,` there.`]);const foo = D[3];
const bar = D[4];
const tag = (v) => v;

const multilineTemplateLiteral = `${D[0]}`;

const multilineTemplateLiteralWithExpressions = `${D[1]}${foo}${D[2]}${bar} there.`;

const multilineTaggedTemplateLiteral = tag(D[5]);

const multilineTaggedTemplateLiteralWithExpressions = tag(D[6], foo, bar);
