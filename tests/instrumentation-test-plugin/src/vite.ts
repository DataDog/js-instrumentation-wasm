import { createVitePlugin } from 'unplugin';
import { unpluginFactory } from './index';

export const InstrumentationTestPlugin = createVitePlugin((unpluginFactory));
