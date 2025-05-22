import { describe, it, expect, test } from 'vitest';
import { render } from '@testing-library/react';
import React from 'react';
import { App } from '../dist';

const UPDATING_SNAPSHOTS = process.env['UPDATING_SNAPSHOTS'] === 'true';
const ifInstrumentationIsEnabledIt = UPDATING_SNAPSHOTS
  ? test.fails
  : it;

const expected = eval(`[
  'click',
  'on',
  'the',
  'react',
  'logos',
  'to',
  'learn',
  'more',
  'about',
  'these',
  'projects',
  'and',
  'save',
  'test',
  'hmr',
  'read',
  'docs',
  'src',
  'app',
  'tsx',
  'logo',
  'count',
  'is',
  'blank',
  'edit',
  'card'
]`);

describe('Built Page test', () => {
  it('should render the expected HTML', () => {
    const { container } = render(<App />);
    // Compare the output to a snapshot generated with the privacy plugin disabled.
    expect(container).toMatchSnapshot();
  });

  ifInstrumentationIsEnabledIt('should generate the expected dictionary', async () => {
    expect((window as any).$DD_ALLOW).toEqual(new Set(expected));
  });
});
