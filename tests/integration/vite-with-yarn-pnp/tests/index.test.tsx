import process from 'node:process';
import React from 'react';
import { render } from '@testing-library/react';
import { describe, expect, it, test } from 'vitest';

import App from '../app/src/App';

const UPDATING_SNAPSHOTS = process.env['UPDATING_SNAPSHOTS'] === 'true';
const ifInstrumentationIsEnabledIt = UPDATING_SNAPSHOTS
  ? test.fails
  : it;

describe('App', () => {
  it('should render the expected HTML', () => {
    const { container } = render(<App />);
    // Compare the output to a snapshot generated with the privacy plugin disabled.
    expect(container).toMatchSnapshot();
  });

  ifInstrumentationIsEnabledIt('should generate the expected dictionary', () => {
    // Note that this dictionary will even contain strings from the tests; to avoid
    // making the result of this test a tautology, we launder the expectation through eval,
    // which is ignored by the privacy plugin.
    const expected = eval(`[
      "about",
      "and",
      "app",
      "blank",
      "card",
      "characters",
      "click",
      "count",
      "dictionary",
      "docs",
      "edit",
      "escape",
      "expected",
      "generate",
      "hmr",
      "html",
      "is",
      "learn",
      "logo",
      "logos",
      "more",
      "on",
      "projects",
      "react",
      "read",
      "render",
      "save",
      "searching",
      "should",
      "snapshots",
      "special",
      "src",
      "test",
      "the",
      "these",
      "to",
      "true",
      "tsx",
      "updating",
      "use",
      "when",
    ]`);

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect((window as any).$DD_ALLOW).toEqual(new Set(expected));
  });
});
