import type { PluginOptions } from '.';
import { addVitePlugin, addWebpackPlugin, defineNuxtModule } from '@nuxt/kit';
import { InstrumentationTestPlugin as ViteInstrumentationTestPlugin } from './vite';
import { InstrumentationTestPlugin as WebpackInstrumentationTestPlugin } from './webpack';
import '@nuxt/schema';

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface ModuleOptions extends PluginOptions { }

export const InstrumentationTestPlugin = defineNuxtModule<PluginOptions>({
  meta: {
    name: 'nuxt-instrumentation-test-plugin',
    configKey: 'unpluginStarter',
  },
  defaults: {
    // ...default options
  },
  setup(options, _nuxt) {
    addVitePlugin(() => ViteInstrumentationTestPlugin(options));
    addWebpackPlugin(() => WebpackInstrumentationTestPlugin(options));

    // ...
  },
});
