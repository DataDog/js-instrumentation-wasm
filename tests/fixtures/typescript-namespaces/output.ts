import { $ } from 'datadog:privacy-helpers';
const D = $([
  'cassiopeia',
  'andromeda',
]);
declare module SomeModule {
  const variable = D[1];
}

declare namespace SomeNamespace {
  const variable = D[0];
}
