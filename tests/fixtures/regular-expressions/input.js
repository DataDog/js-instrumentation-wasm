import compact from 'lodash/compact';

export const PATTERN = /[:()"\\]/g;

const addQuotes = (string, hasQuotes) =>
  hasQuotes ? `"${string}"` : string;
