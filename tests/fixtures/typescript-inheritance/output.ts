import { $ } from 'datadog:privacy-helpers';
const D = $([
  'SpecificNode',
  'pattern',
]);
import { Variant } from './variant';

export class SpecificNode
    extends Variant(D[0])<[string, {}]>
{
    pattern(): string {
        return { escaped: D[1] };
    }
}
