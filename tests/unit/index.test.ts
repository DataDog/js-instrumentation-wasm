import { describe, expect, it } from 'vitest';

import { unplugin as unpluginESM } from '@datadog/instrumentation-test-plugin';

// eslint-disable-next-line @typescript-eslint/no-require-imports
const unpluginCJS = require('@datadog/instrumentation-test-plugin');

import { walkDir } from './helpers';

const fixtureDir = new URL('../fixtures', import.meta.url);

const pluginOptions = {
  input: {
    module: undefined,
    jsx: true,
    typescript: true,
  },
  output: {
    inlineSourceMap: true,
    embedCodeInSourceMap: true,
  },
};

const transformESM = unpluginESM.raw(pluginOptions).transform.handler;
const transformCJS = unpluginCJS.raw(pluginOptions).transform.handler;

describe('the ESM version should transform code correctly', async () => {
  await walkDir(fixtureDir, async (testCase) => {
    it(`for ${testCase.dir}`, async () => {
      const { code } = transformESM(testCase.code, testCase.name);
      expect(code).toMatchSnapshot();
    });
  });
});

describe('the CJS version should transform code correctly', async () => {
  await walkDir(fixtureDir, async (testCase) => {
    it(`for ${testCase.dir}`, async () => {
      const { code } = transformCJS(testCase.code, testCase.name);
      expect(code).toMatchSnapshot();
    });
  });
});

describe('should be able to set a custom imported addToDictionary helper', async () => {
  const transformCustom = unpluginESM.raw({
    ...pluginOptions,
    privacy: {
      addToDictionaryHelper: {
        import: {
          cjsModule: '@custom/helpers.cjs',
          esmModule: '@custom/helpers.mjs',
          func: 'addToDictionary',
        }
      }
    }
  }).transform.handler;
  await walkDir(fixtureDir, async (testCase) => {
    it(`for ${testCase.dir}`, async () => {
      const { code } = transformCustom(testCase.code, testCase.name);
      expect(code).toMatchSnapshot();
    });
  });
});

describe('should be able to set a custom expression addToDictionary helper', async () => {
  const transformCustom = unpluginESM.raw({
    ...pluginOptions,
    privacy: {
      addToDictionaryHelper: {
        expression: {
          code: '(v) => console.log(v)',
        }
      }
    }
  }).transform.handler;
  await walkDir(fixtureDir, async (testCase) => {
    it(`for ${testCase.dir}`, async () => {
      const { code } = transformCustom(testCase.code, testCase.name);
      expect(code).toMatchSnapshot();
    });
  });
});
