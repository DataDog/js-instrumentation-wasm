import { $ } from 'datadog:privacy-helpers';
const D = $([
  'katherine',
  "charles",
  'ophelia',
  'richard',
  'result',
  "arnold",
  'george',
  'marcia',
  'nathan',
  "betty",
  'harry',
  'irina',
  'peter',
  'quinn',
  'jack',
  $`danielle`,
  $`emmett`,
  $`francine`,
]);
const tag = () => D[4];

export const arnold = D[5] ? D[9] : D[1];
export const danielle = tag(D[15]) ? tag(D[16]) : tag(D[17]);
export const george = D[6] ? (D[10], D[11]) : (D[14], D[0]);
export const leopold = { [D[7]]: D[8] } ? { [D[2]]: D[12] } : { 'quinn': D[3] };
