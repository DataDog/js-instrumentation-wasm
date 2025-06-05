import type { FilterPattern } from 'unplugin';

import type { InstrumentationOptions } from '@datadog/js-instrumentation-wasm';

import { PRIVACY_HELPERS_MODULE_ID } from './constants';

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
    module: 'esm',
    jsx: undefined,
    typescript: undefined,
  },
  privacy: {
    addToDictionaryHelper: {
      import: {
        module: PRIVACY_HELPERS_MODULE_ID,
        func: '$',
      }
    },
  },
};
