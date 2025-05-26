import { $ } from 'datadog:privacy-helpers';
const D = $([
  'betelgeuse',
  'cassiopeia',
  'andromeda',
]);
declare module SomeModule {
  const variable = D[2];
}

declare module "SomeModule" {
  const variable = D[0];
}

declare namespace SomeNamespace {
  const variable = D[1];
}
