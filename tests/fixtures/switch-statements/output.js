import { $ } from 'datadog:privacy-helpers';
const D = $([
  'francis',
  'result',
  'emmett',
  "arnold",
  'betty',
  $`danielle`,
  `charles`,
]);
const tag = () => D[1];

export const foo = (value) => {
  const emmett = D[2];

  switch (value) {
    case D[3]:
      return 1;

    case D[4]:
      return 2;

    case `charles`:
      return 3;

    case tag(D[5]):
      return 4;

    case emmett:
      return 5;

    case D[0]:
      return 6;

    default:
      return 7;
  }
};
