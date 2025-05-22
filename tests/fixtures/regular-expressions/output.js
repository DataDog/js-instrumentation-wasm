import { $ } from 'datadog:privacy-helpers';
const D = $([
  `"`,
]);
import compact from 'lodash/compact';

export const PATTERN = /[:()"\\]/g;

const addQuotes = (string, hasQuotes) =>
  hasQuotes ? `"${string}"` : string;
