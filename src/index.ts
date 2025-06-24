import {
  initSync,
  transform
} from '../rust/datadog-js-instrumentation/pkg/datadog_js_instrumentation.js';
import {
  default as plugin
} from '../rust/datadog-js-instrumentation/pkg/datadog_js_instrumentation_bg.wasm';

export interface InstrumentationInput {
  /** The filename or id of the code to instrument. */
  id: string;
  /** The source code to instrument. */
  code: string;
  /** The source map for the code to instrument, if any. */
  map?: string;
}

export interface InstrumentationOutput {
  /** The filename or id of the code that was instrumented. */
  id: string;
  /** The instrumented source code. */
  code: string;
  /** The source map for the instrumented code. */
  map?: string;
}

export interface InputOptions {
  /**
   * If 'cjs', interpret the input as a CommonJS module. If 'esm', interpret the input as
   * an ES module. If 'undefined' (the default), guess based on the module contents.
   */
  module?: 'cjs' | 'esm' | undefined;
  /** If true (the default), allow JSX syntax in the input. */
  jsx?: boolean;
  /** If true (the default), allow TypeScript syntax in the input. */
  typescript?: boolean;
}

/**
 * Declare the helper using the given JavaScript expression.
 *
 * Example: `{ code: '(v) => console.log(v)' }` will produce output like:
 * ```js
 *   const $ = (v) => console.log(v);
 * ```
 */
export type ExpressionPrivacyHelperSource = {
  expression: { code: string };
};

/**
 * Declare the helper by importing the given function from the given module.
 *
 * Example:
 * ```js
 *   {
 *     cjsModule: 'custom/helpers.cjs',
 *     esmModule: 'custom/helpers.mjs',
 *     func: 'foo'
 *   }
 * ```
 * will produce output like:
 * ```js
 *   import { foo as $ } from 'custom/helpers.mjs';
 * ```
 *
 * If the input is an ES module, `import` will be used to import `esmModule`;
 * likewise, for CommonJS modules, `require()` will be used to import `cjsModule`.
 */
export type ImportPrivacyHelperSource = {
  import: {
    cjsModule: string;
    esmModule: string;
    func: string;
  }
};

export type PrivacyHelperSource =
  | ExpressionPrivacyHelperSource
  | ImportPrivacyHelperSource;

export interface PrivacyOptions {
  /** The source for the helper function used to add strings to the dictionary. */
  addToDictionaryHelper?: PrivacyHelperSource;
}

export interface InstrumentationOptions {
  /** Options that configure how the input is interpreted. */
  input?: InputOptions;
  /** Options that configure the privacy instrumentation transform. */
  privacy?: PrivacyOptions;
}

export interface RustPrivacyOptions {
  addToDictionaryHelper: PrivacyHelperSource;
}

interface RustInstrumentationOptions {
  input: InputOptions;
  privacy: RustPrivacyOptions;
}

interface RustInstrumentationOutput {
  code: string;
  map?: string;
}

function convertOptions(
  options: InstrumentationOptions | undefined
): RustInstrumentationOptions {
  return {
    input: options?.input ?? {
      module: 'esm',
      jsx: undefined,
      typescript: undefined,
    },
    privacy: {
      addToDictionaryHelper: options?.privacy?.addToDictionaryHelper ?? {
        import: {
          cjsModule: 'datadog:privacy-helpers.cjs',
          esmModule: 'datadog:privacy-helpers.mjs',
          func: '$',
        }
      }
    },
  };
}

let wasmPluginLoaded = false;
async function ensureWasmPluginLoaded() {
  if (wasmPluginLoaded) {
    return;
  }

  const module = (plugin as unknown as () => unknown)();
  initSync({ module });

  wasmPluginLoaded = true;
}

/**
  * Instrument the provided input, applying the provided options, if any.
  *
  * @returns the instrumented code.
  */
export function instrument(
  input: InstrumentationInput,
  options?: InstrumentationOptions | undefined,
): InstrumentationOutput {
  try {
    ensureWasmPluginLoaded();
    const rustOptions = convertOptions(options);
    const output: RustInstrumentationOutput = transform(
      input.id,
      input.code,
      rustOptions
    );
    return {
      id: input.id,
      code: output.code,
      map: output.map,
    };
  } catch (e) {
    console.log(`Instrumentation threw error`, e);
    throw e;
  }
}
