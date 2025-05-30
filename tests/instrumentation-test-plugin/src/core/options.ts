import type { FilterPattern } from 'unplugin';

export interface PluginOptions {
  exclude: FilterPattern;
  include: FilterPattern;
  module: 'cjs' | 'esm' | 'unknown';
  jsx: boolean | undefined;
  transformStrategy: 'ast' | undefined;
  typescript: boolean | undefined;
}

export const defaultPluginOptions: PluginOptions = {
  exclude: [
    /\/node_modules\//,
    /\.preval\./
  ],
  include: [
    /\.(?:c|m)?(?:j|t)sx?$/
  ],
  module: 'esm',
  jsx: undefined,
  transformStrategy: 'ast',
  typescript: undefined,
};
