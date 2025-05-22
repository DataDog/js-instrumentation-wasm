import { createRollupPlugin } from 'unplugin';
import { unpluginFactory } from './index';

export const InstrumentationTestPlugin = createRollupPlugin(unpluginFactory);
