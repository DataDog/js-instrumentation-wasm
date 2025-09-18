import{$}from'datadog:privacy-helpers.mjs';const D=$(["(jsx-js-expr-in-text: should preserve entities) \\x &bsol;&euro; &apos;&quot; 123","(js-expression-attr: should preserve entities) \\x &bsol;&euro; &apos;&quot; 123","(jsx-text: should decode entities) A < B & C > D. \\€ \"Everything's OK.\" ©","(double-quote-html-attr: should decode entities) \\x \\€ '\" 123",'(single-quote-html-attr: should decode entities) \\x \\€ \'" 123'," Company, 123 4th Ave, City"]);import { Text } from 'framework';

export function MyComponent() {
  return (
    <Text
      double-quote-html-attr={D[3]}
      single-quote-html-attr={D[4]}
      js-expr-attr={D[1]}
    >{D[2]}{D[0]}{D[5]}</Text>
  );
}
