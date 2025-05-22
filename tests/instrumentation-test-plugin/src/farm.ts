import { createFarmPlugin } from 'unplugin';
import { unpluginFactory } from './index';

export const InstrumentationTestPlugin = createFarmPlugin(unpluginFactory);
