import { describe, it, expect, test } from 'vitest';
import { render } from '@testing-library/react';
import React from 'react';
import { App } from '../dist';

const UPDATING_SNAPSHOTS = process.env['UPDATING_SNAPSHOTS'] === 'true';
const ifInstrumentationIsEnabledIt = UPDATING_SNAPSHOTS
  ? test.fails
  : it;

describe('Built Page test', () => {
  it('should render the expected HTML', () => {
    const { container } = render(<App />);
    // Compare the output to a snapshot generated with the privacy plugin disabled.
    expect(container).toMatchSnapshot();
  });

  ifInstrumentationIsEnabledIt('should generate the expected dictionary', async () => {
    // Note that this dictionary will even contain strings from the tests; to avoid
    // making the result of this test a tautology, we launder the expectation through eval,
    // which is ignored by the privacy plugin.
    const expected = eval(`[
      "about",
      "and",
      "app",
      "blank",
      "characters",
      "click",
      "count",
      "edit",
      "escape",
      "hmr",
      "is",
      "learn",
      "logo",
      "logos",
      "more",
      "on",
      "projects",
      "react",
      "save",
      "searching",
      "special",
      "src",
      "test",
      "the",
      "these",
      "to",
      "tsx",
      "use",
      "when",
    ]`);

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect((window as any).$DD_ALLOW).toEqual(new Set(expected));
  });
});
