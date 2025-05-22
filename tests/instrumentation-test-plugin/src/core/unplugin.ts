import { createFilter } from '@rollup/pluginutils';
import { createUnplugin, type UnpluginFactory } from 'unplugin';

import {
  defaultPrivacyHelpers,
  instrument,
  type InstrumentationOptions
} from '@datadog/js-instrumentation-wasm';

import { PRIVACY_HELPERS_MODULE_ID } from './constants';
import { defaultPluginOptions, type PluginOptions } from './options';

type UnpluginOptions = Partial<PluginOptions> | undefined;

function buildInstrumentationOptions(
  pluginOptions: PluginOptions
): InstrumentationOptions {
  return {
    input: {
      module: pluginOptions.module === 'unknown' ? undefined : pluginOptions.module,
      jsx: pluginOptions.jsx,
      typescript: pluginOptions.typescript,
    },
    privacy: {
      helpers: {
        module: PRIVACY_HELPERS_MODULE_ID,
        addToDictionaryFunction: '$',
      },
    },
  };
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
          code: defaultPrivacyHelpers,
        };
      },
    },

    transformInclude(id) {
      return instrumentationFilter(id);
    },

    transform: {
      order: 'pre',

      async handler(code, id) {
        const result = await instrument({ id, code }, instrumentationOptions);
        return { code: result.code, map: result.map };
      },
    },
  };
};

export const unplugin = /* #__PURE__ */ createUnplugin(unpluginFactory);

export default unplugin;
