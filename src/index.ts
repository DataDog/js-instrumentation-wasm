import {
  default as init,
  transform
} from '../rust/datadog-js-instrumentation/pkg/datadog_js_instrumentation.js';
import {
  default as plugin
} from '../rust/datadog-js-instrumentation/pkg/datadog_js_instrumentation_bg.wasm';

export interface InstrumentationInput {
  id: string;
  code: string;
  map?: string;
}

export interface InstrumentationOutput {
  id: string;
  code: string;
  map?: string;
}

export interface InputOptions {
  module?: 'cjs' | 'esm' | undefined;
  jsx?: boolean;
  typescript?: boolean;
}

export interface PrivacyHelperOptions {
  module: string;
  addToDictionaryFunction: string;
}

export interface PrivacyOptions {
  helpers?: PrivacyHelperOptions;
}

export interface InstrumentationOptions {
  input?: InputOptions;
  privacy?: PrivacyOptions;
}

interface RustOptions {
  module: 'cjs' | 'esm' | undefined;
  jsx: boolean | undefined;
  typescript: boolean | undefined;

  addToDictionaryHelper: string;
  helpersModule: string;
  transformStrategy: 'ast' | undefined;
}

interface RustOutput {
  code: string;
  map?: string;
}

function convertOptions(options: InstrumentationOptions | undefined): RustOptions {
  return {
    module: options?.input?.module ?? 'esm',
    jsx: options?.input?.jsx,
    typescript: options?.input?.typescript,

    addToDictionaryHelper: options?.privacy?.helpers?.addToDictionaryFunction ?? '$',
    helpersModule: options?.privacy?.helpers?.module ?? 'datadog:privacy-helpers',
    transformStrategy: 'ast',
  };
}

async function loadWasmPlugin() {
  const module = await (plugin as unknown as () => unknown)();
  await init({
    module_or_path: module,
  });
}

export async function instrument(
  input: InstrumentationInput,
  options?: InstrumentationOptions | undefined,
): Promise<InstrumentationOutput> {
  try {
    await loadWasmPlugin();
    const rustOptions = convertOptions(options);
    const output: RustOutput = transform(input.id, input.code, rustOptions);
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
