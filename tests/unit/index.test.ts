import { describe, expect, it } from 'vitest';

import { unplugin as unpluginESM } from '@datadog/instrumentation-test-plugin';

// eslint-disable-next-line @typescript-eslint/no-require-imports
const unpluginCJS = require('@datadog/instrumentation-test-plugin');

import { expectEveryPositionToHaveASourceMapping, TestCase, walkDir } from './helpers';

const fixtureDir = new URL('../fixtures', import.meta.url);

const pluginOptions = {
  input: {
    module: undefined,
    jsx: true,
    typescript: true,
  },
  output: {
    inlineSourceMap: true,
  },
};

const transformESM = unpluginESM.raw(pluginOptions).transform.handler;
const transformCJS = unpluginCJS.raw(pluginOptions).transform.handler;

const expectEveryPositionToHaveASourceMappingIfAppropriate = async (
  testCase: TestCase,
  output: { code: string, map: string }
): Promise<void> => {
  if (testCase.code.includes('//# sourceMapping')) {
    // This test case includes an input source map. When an input source map
    // is present, we chain with it, and in this case we can't guarantee that
    // every location has a mapping, because we can only update mappings that
    // existed in the original source map. Skip the expectation in this case.
    return;
  }

  // This test case includes no input source map, so the output source map
  // will be generated from scratch by us. In this case, every location
  // in the output source code should have a mapping.
  return expectEveryPositionToHaveASourceMapping(
    testCase.name,
    output.code,
    output.map
  );
};

describe('the ESM version should transform code correctly', async () => {
  await walkDir(fixtureDir, async (testCase) => {
    it(`for ${testCase.dir}`, async () => {
      const output = transformESM(testCase.code, testCase.name);
      expect(output.code).toMatchSnapshot();
      await expectEveryPositionToHaveASourceMappingIfAppropriate(testCase, output);
    });
  });
});

describe('the CJS version should transform code correctly', async () => {
  await walkDir(fixtureDir, async (testCase) => {
    it(`for ${testCase.dir}`, async () => {
      const output = transformCJS(testCase.code, testCase.name);
      expect(output.code).toMatchSnapshot();
      await expectEveryPositionToHaveASourceMappingIfAppropriate(testCase, output);
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
      const output = transformCustom(testCase.code, testCase.name);
      expect(output.code).toMatchSnapshot();
      await expectEveryPositionToHaveASourceMappingIfAppropriate(testCase, output);
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
      const output = transformCustom(testCase.code, testCase.name);
      expect(output.code).toMatchSnapshot();
      await expectEveryPositionToHaveASourceMappingIfAppropriate(testCase, output);
    });
  });
});
