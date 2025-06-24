import { $ } from 'datadog:privacy-helpers.mjs';
const D = $([
  `"`,
]);
import compact from 'lodash/compact';

export const PATTERN = /[:()"\\]/g;

const addQuotes = (string, hasQuotes) =>
  hasQuotes ? `"${string}"` : string;
