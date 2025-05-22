import { default as unplugin, type PluginOptions } from './index';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const InstrumentationTestPlugin = (options: PluginOptions): any => ({
  name: 'instrumentation-test-plugin',
  hooks: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    'astro:config:setup': async (astro: any) => {
      astro.config.vite.plugins ||= [];
      astro.config.vite.plugins.push(unplugin.vite(options));
    },
  },
});
