import { Variant } from './variant';

export class SpecificNode
    extends Variant('SpecificNode')<[string, {}]>
{
    pattern(): string {
        return { escaped: 'pattern' };
    }
}
