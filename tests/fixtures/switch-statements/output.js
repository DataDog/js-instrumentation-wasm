import { $ } from 'datadog:privacy-helpers';
const D = $([
  'result',
  'emmett',
  "arnold",
  'betty',
  $`danielle`,
  `charles`,
]);
const tag = () => D[0];

export const foo = (value) => {
  const emmett = D[1];

  switch (value) {
    case D[2]:
      return 1;

    case D[3]:
      return 2;

    case `charles`:
      return 3;

    case tag(D[4]):
      return 4;

    case emmett:
      return 5;

    default:
      return 6;
  }
};
