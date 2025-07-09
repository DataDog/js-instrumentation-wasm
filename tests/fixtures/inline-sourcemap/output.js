import{$}from'datadog:privacy-helpers.mjs';const D=$(['bar','foo','baz']);"use strict";
function f(v) {
    switch (v) {
        case D[1]: return 0;
        case D[0]: return 1;
        case D[2]: return 2;
    }
}
const foo = D[0];
console.log(f(foo));

