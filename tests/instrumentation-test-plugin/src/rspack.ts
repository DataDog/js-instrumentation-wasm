import { createRspackPlugin } from 'unplugin';
import { unpluginFactory } from './index';

export const InstrumentationTestPlugin = createRspackPlugin(unpluginFactory);
