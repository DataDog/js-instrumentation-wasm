const{$}=require('datadog:privacy-helpers.cjs');const D=$(['hello']);module.exports = {
  foo: D[0],
  bar() {
    return true;
  }
};
