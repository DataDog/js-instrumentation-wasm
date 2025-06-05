import { createFilter } from '@rollup/pluginutils';
import { createUnplugin, type UnpluginFactory } from 'unplugin';

import {
  instrument,
  type InstrumentationOptions
} from '@datadog/js-instrumentation-wasm';

import helpers from './generated/privacy-helpers.js-txt';

import { PRIVACY_HELPERS_MODULE_ID } from './constants';
import { defaultPluginOptions, PluginOptions } from './options';

type UnpluginOptions = PluginOptions | undefined;

function buildInstrumentationOptions(
  pluginOptions: PluginOptions | undefined
): InstrumentationOptions {
  const options: InstrumentationOptions = pluginOptions ?? {};
  if (!options.privacy?.addToDictionaryHelper) {
    options.privacy = options.privacy ?? {};
    options.privacy.addToDictionaryHelper = {
      import: {
        module: PRIVACY_HELPERS_MODULE_ID,
        func: '$',
      }
    };
  }
  return options;
}

export const unpluginFactory: UnpluginFactory<UnpluginOptions> = options => {
  const pluginOptions = {
    ...defaultPluginOptions,
    ...options,
  };
  const instrumentationOptions = buildInstrumentationOptions(pluginOptions);
  const instrumentationFilter = createFilter(
    pluginOptions.include,
    pluginOptions.exclude
  );
  return {
    name: 'instrumentation-test-plugin',

    resolveId(source) {
      if (source === PRIVACY_HELPERS_MODULE_ID) {
        return { id: PRIVACY_HELPERS_MODULE_ID };
      }
      return null;
    },

    load: {
      order: 'pre',

      filter: {
        id: { include: new RegExp(`^${PRIVACY_HELPERS_MODULE_ID}$`) },
      },

      handler(id) {
        if (id !== PRIVACY_HELPERS_MODULE_ID) {
          return null;
        }
        return {
          code: helpers,
        };
      },
    },

    transformInclude(id) {
      return instrumentationFilter(id);
    },

    transform: {
      order: 'pre',

      handler(code, id) {
        const result = instrument({ id, code }, instrumentationOptions);
        return { code: result.code, map: result.map };
      },
    },
  };
};

export const unplugin = /* #__PURE__ */ createUnplugin(unpluginFactory);

export default unplugin;
