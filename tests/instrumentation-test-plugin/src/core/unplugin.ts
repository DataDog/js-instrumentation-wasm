import { createFilter } from '@rollup/pluginutils';
import { createUnplugin, type UnpluginFactory } from 'unplugin';

import {
  ImportPrivacyHelperSource,
  instrument,
  type InstrumentationOptions
} from '@datadog/js-instrumentation-wasm';

import helpers from './generated/privacy-helpers.js-txt';

import {
  PRIVACY_HELPERS_MODULE_CJS_ID,
  PRIVACY_HELPERS_MODULE_ESM_ID,
} from './constants';
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
        cjsModule: PRIVACY_HELPERS_MODULE_CJS_ID,
        esmModule: PRIVACY_HELPERS_MODULE_ESM_ID,
        func: '$',
      }
    };
  }
  return options;
}

function getHelperModuleNames(
  options: InstrumentationOptions
): { cjsModule: string, esmModule: string } {
  const addToDictionaryHelper = options?.privacy?.addToDictionaryHelper ?? {};
  if (!('import' in addToDictionaryHelper)) {
    // We're using an expression-style helper. Use the default module names, although we
    // won't ever actually load or resolve them.
    return {
      cjsModule: PRIVACY_HELPERS_MODULE_CJS_ID,
      esmModule: PRIVACY_HELPERS_MODULE_ESM_ID
    };
  }
  const importHelper = addToDictionaryHelper as ImportPrivacyHelperSource;
  return {
    cjsModule: importHelper.import.cjsModule,
    esmModule: importHelper.import.esmModule,
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
  const { cjsModule, esmModule } = getHelperModuleNames(instrumentationOptions);
  return {
    name: 'instrumentation-test-plugin',

    resolveId(source) {
      if (source === cjsModule || source === esmModule) {
        return { id: source };
      }
      return null;
    },

    load: {
      order: 'pre',

      filter: {
        id: {
          include: new RegExp(`^(?:${cjsModule})|(?:${esmModule})$`)
        },
      },

      handler(id) {
        if (id === cjsModule || id === esmModule) {
          return { code: helpers };
        }
        return null;
      },
    },

    transformInclude(id) {
      // Check for a literal match for our helpers.
      if (id === cjsModule || id === esmModule) {
        return false;
      }

      // Check for a URI ending with an encoded version of our helpers. (This is how
      // webpack does things.)
      const decodedId = decodeURIComponent(id);
      if (decodedId.endsWith(cjsModule) || decodedId.endsWith(esmModule)) {
        return false;
      }

      // We're not dealing with our helpers; fall back to the filter specified in the
      // options.
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
