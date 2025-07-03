import type { FilterPattern } from 'unplugin';

import type { InstrumentationOptions } from '@datadog/js-instrumentation-wasm';

import {
  PRIVACY_HELPERS_MODULE_CJS_ID,
  PRIVACY_HELPERS_MODULE_ESM_ID,
} from './constants';

export type PluginOptions = InstrumentationOptions & {
  exclude: FilterPattern;
  include: FilterPattern;
};

export const defaultPluginOptions: PluginOptions = {
  exclude: [
    /\/node_modules\//,
    /\.preval\./
  ],
  include: [
    /\.(?:c|m)?(?:j|t)sx?$/
  ],
  input: {
    module: undefined,
    jsx: undefined,
    typescript: undefined,
  },
  output: {
    inlineSourceMap: false,
    embedCodeInSourceMap: true,
  },
  privacy: {
    addToDictionaryHelper: {
      import: {
        cjsModule: PRIVACY_HELPERS_MODULE_CJS_ID,
        esmModule: PRIVACY_HELPERS_MODULE_ESM_ID,
        func: '$',
      }
    },
  },
};
