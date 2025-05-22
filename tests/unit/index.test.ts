import { describe, expect, it } from 'vitest';

import { unplugin as unpluginESM } from '@datadog/instrumentation-test-plugin';

// eslint-disable-next-line @typescript-eslint/no-require-imports
const unpluginCJS = require('@datadog/instrumentation-test-plugin');

import { walkDir } from './helpers';

const fixtureDir = new URL('../fixtures', import.meta.url);

const pluginOptions = {
  module: 'unknown',
  jsx: true,
  typescript: true,
};

const transformESM = unpluginESM.raw(pluginOptions).transform.handler;
const transformCJS = unpluginCJS.raw(pluginOptions).transform.handler;

describe('the ESM version should transform code correctly', async () => {
  await walkDir(fixtureDir, async (testCase) => {
    it(`should transform ${testCase.dir} correctly`, async () => {
      const { code } = await transformESM(testCase.code, testCase.name);
      expect(code).toMatchSnapshot();
    });
  });
});

describe('the CJS version should transform code correctly', async () => {
  await walkDir(fixtureDir, async (testCase) => {
    it(`should transform ${testCase.dir} correctly`, async () => {
      const { code } = await transformCJS(testCase.code, testCase.name);
      expect(code).toMatchSnapshot();
    });
  });
});
