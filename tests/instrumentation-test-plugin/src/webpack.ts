import { createWebpackPlugin } from 'unplugin';
import { unpluginFactory } from './index';

export const InstrumentationTestPlugin = createWebpackPlugin(unpluginFactory);
