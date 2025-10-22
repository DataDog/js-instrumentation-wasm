import { describe, expect, it, test } from 'vitest';
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
      "a",
      "about",
      "add",
      "and",
      "app",
      "attributes",
      "between",
      "blank",
      "characters",
      "click",
      "count",
      "edit",
      "escape",
      "extra",
      "for",
      "hello",
      "hmr",
      "in",
      "is",
      "it",
      "jsx",
      "learn",
      "logo",
      "logos",
      "more",
      "multiline",
      "n",
      "not",
      "on",
      "projects",
      "react",
      "save",
      "searching",
      "should",
      "special",
      "src",
      "test",
      "text",
      "the",
      "these",
      "this",
      "to",
      "tsx",
      "use",
      "when",
      "world",
    ]`);

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect((window as any).$DD_ALLOW).toEqual(new Set(expected));
  });
});
