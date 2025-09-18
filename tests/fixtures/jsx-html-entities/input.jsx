import { Text } from 'framework';

export function MyComponent() {
  return (
    <Text
      double-quote-html-attr="(double-quote-html-attr: should decode entities) \x &bsol;&euro; &apos;&quot; 123"
      single-quote-html-attr='(single-quote-html-attr: should decode entities) \x &bsol;&euro; &apos;&quot; 123'
      js-expr-attr={"(js-expression-attr: should preserve entities) \\x &bsol;&euro; &apos;&quot; 123"}
    >
      (jsx-text: should decode entities) A &lt; B&nbsp;&amp; C &gt; D. &#92;&euro; &quot;Everything&apos;s OK.&quot;
      &#169;{"(jsx-js-expr-in-text: should preserve entities) \\x &bsol;&euro; &apos;&quot; 123"} Company, 123 4th Ave, City
    </Text>
  );
}
