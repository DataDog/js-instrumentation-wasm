import { createEsbuildPlugin } from 'unplugin';
import { unpluginFactory } from './index';

export const InstrumentationTestPlugin = createEsbuildPlugin(unpluginFactory);
