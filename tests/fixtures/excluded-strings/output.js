// URLS (excluded by URL_STRINGS_REGEX):
console.log(
  'http://www.excluded.com',
  'https://www.excluded.com',
  'data://www.excluded.com',
  'url(https://www.excluded.com)',
  '//www.excluded.com',
);

// File names (excluded by FILE_NAME_REGEX):
const fileNames = {
  'image.png': 'image.gif',
  'foo.jpg': 'bar.jpeg',
  'something.svg': 'something.webp',
  'script-file.js': 'module.cjs',
  'source.mjs': 'some/file.ts',
  '../file.cts': '/any/file.mts',
};

// File names (excluded by FILE_NAME_REGEX):
const numericStrings = [
  '123',
  '123 456 789',
  '123-456-789',
  '123,456,789',
  '123, 456, 789',
  '123.456.789',
  '123.456,789',
  '123. 456, 789',
  '123 . 456 , 789',
  '[7890]',
  '[1234] [6789]',
  '[1234]-[6789]',
  '[1234].[6789]',
  '[1234],[6789]',
  '[1234] , [6789]',
];

// Excluded JSX elements:
const excludedElements = (
  <div>
    <g attr="something">
      <p thing="some data">Some text.</p>
    </g>
    <path attr={`M 10,30
              A 20,20 0,0,1 50,30
              A 20,20 0,0,1 90,30
              Q 90,60 50,90
              Q 10,60 10,30 z`} />
    <path
      attr="M102,140H86v-20c0-13.5,10.1-24,22.9-24c13.2,0,23-3,28.9-9c5.9-5.9,6.1-12.9,6.1-13l0-0.4l0-0.4c0-0.4,0.8-11.6-7.6-20.8
C128.8,44.2,115.9,40,98,40c-20.9,0-35.6,5.7-43.7,16.9c-6,8.3-6.3,17-6.3,17.1L32,74c0-1.3,0.2-13.3,8.7-25.5
C54.6,28.2,79.5,24,98,24c22.7,0,39.7,6,50.5,17.9c12.2,13.4,11.7,29.6,11.5,32.5c-0.1,2.5-0.9,14-10.8,23.9
c-9.1,9.1-22.6,13.7-40.3,13.7c-4.6,0-6.9,4-6.9,8V140z"
    />
  </div>
);

// Excluded JSX elements, after transpilation with 'jsx: react'.
const excludedElementsAfterJsxReact = (
  React.createElement("div", null,
    React.createElement("g", { attr: "something" },
      React.createElement("p", { thing: "some data" }, "Some text.")),
    React.createElement("path", {
      attr: `M 10,30
              A 20,20 0,0,1 50,30
              A 20,20 0,0,1 90,30
              Q 90,60 50,90
              Q 10,60 10,30 z` }),
    React.createElement("path", { attr: "M102,140H86v-20c0-13.5,10.1-24,22.9-24c13.2,0,23-3,28.9-9c5.9-5.9,6.1-12.9,6.1-13l0-0.4l0-0.4c0-0.4,0.8-11.6-7.6-20.8\nC128.8,44.2,115.9,40,98,40c-20.9,0-35.6,5.7-43.7,16.9c-6,8.3-6.3,17-6.3,17.1L32,74c0-1.3,0.2-13.3,8.7-25.5\nC54.6,28.2,79.5,24,98,24c22.7,0,39.7,6,50.5,17.9c12.2,13.4,11.7,29.6,11.5,32.5c-0.1,2.5-0.9,14-10.8,23.9\nc-9.1,9.1-22.6,13.7-40.3,13.7c-4.6,0-6.9,4-6.9,8V140z" })
  )
);

// Excluded JSX elements, after transpilation with 'jsx: react-jsx'.
const excludedElementsAfterJsxReactJsx = (
  _jsxs("div", {
    children: [
      _jsx("g", {
        attr: "something", children: _jsx("p", { thing: "some data", children: "Some text." })
      }),
      _jsx("path", {
        attr: `M 10,30
              A 20,20 0,0,1 50,30
              A 20,20 0,0,1 90,30
              Q 90,60 50,90
              Q 10,60 10,30 z` }),
      _jsx("path", { attr: "M102,140H86v-20c0-13.5,10.1-24,22.9-24c13.2,0,23-3,28.9-9c5.9-5.9,6.1-12.9,6.1-13l0-0.4l0-0.4c0-0.4,0.8-11.6-7.6-20.8\nC128.8,44.2,115.9,40,98,40c-20.9,0-35.6,5.7-43.7,16.9c-6,8.3-6.3,17-6.3,17.1L32,74c0-1.3,0.2-13.3,8.7-25.5\nC54.6,28.2,79.5,24,98,24c22.7,0,39.7,6,50.5,17.9c12.2,13.4,11.7,29.6,11.5,32.5c-0.1,2.5-0.9,14-10.8,23.9\nc-9.1,9.1-22.6,13.7-40.3,13.7c-4.6,0-6.9,4-6.9,8V140z" })
    ]
  })
);

// Excluded JSX elements, after transpilation with 'jsx: react-jsxdev'.
const excludedElementsAfterJsxReactJsxDev = (
  _jsxDEV("div", {
    children: [
      _jsxDEV("g", {
        attr: "something", children:
          _jsxDEV("p", { thing: "some data", children: "Some text." }, void 0, false, { fileName: _jsxFileName, lineNumber: 4, columnNumber: 7 }, this)
      }, void 0, false, { fileName: _jsxFileName, lineNumber: 3, columnNumber: 5 }, this),
      _jsxDEV("path", {
        attr: `M 10,30
              A 20,20 0,0,1 50,30
              A 20,20 0,0,1 90,30
              Q 90,60 50,90
              Q 10,60 10,30 z` }, void 0, false, { fileName: _jsxFileName, lineNumber: 6, columnNumber: 5 }, this),
      _jsxDEV("path", { attr: "M102,140H86v-20c0-13.5,10.1-24,22.9-24c13.2,0,23-3,28.9-9c5.9-5.9,6.1-12.9,6.1-13l0-0.4l0-0.4c0-0.4,0.8-11.6-7.6-20.8\nC128.8,44.2,115.9,40,98,40c-20.9,0-35.6,5.7-43.7,16.9c-6,8.3-6.3,17-6.3,17.1L32,74c0-1.3,0.2-13.3,8.7-25.5\nC54.6,28.2,79.5,24,98,24c22.7,0,39.7,6,50.5,17.9c12.2,13.4,11.7,29.6,11.5,32.5c-0.1,2.5-0.9,14-10.8,23.9\nc-9.1,9.1-22.6,13.7-40.3,13.7c-4.6,0-6.9,4-6.9,8V140z" }, void 0, false, { fileName: _jsxFileName, lineNumber: 11, columnNumber: 5 }, this)]
  }, void 0, true, { fileName: _jsxFileName, lineNumber: 1, columnNumber: 40 }, this));

// Excluded JSX attributes:
const excludedAttributes = (
  <div>
    <p class="excluded" />
    <p d="excluded attribute d"></p>
    <p
      d="M 10,50
           Q 25,25 40,50
           t 30,0 30,0 30,0 30,0 30,0" />
    <p
      d="M 15,1
           l -4,8 8,0 -4,-8" />
    <p d="M150 5 L75 200 L225 200 Z" />
    <p id="excluded" />
    <p src="www.excluded.com" />
    <p srcset="www.excluded.com,alsoexcluded.com" />
    <p style="excluded attribute style"></p>
  </div>
);

// Excluded JSX attributes, after transpilation with 'jsx: react'.
const excludedAttributesAfterJsxReact = (
  React.createElement("div", null,
    React.createElement("p", { class: "excluded" }),
    React.createElement("p", { d: "excluded attribute d" }),
    React.createElement("p", { d: "M 10,50\n           Q 25,25 40,50\n           t 30,0 30,0 30,0 30,0 30,0" }),
    React.createElement("p", { d: "M 15,1\n           l -4,8 8,0 -4,-8" }),
    React.createElement("p", { d: "M150 5 L75 200 L225 200 Z" }),
    React.createElement("p", { id: "excluded" }),
    React.createElement("p", { src: "www.excluded.com" }),
    React.createElement("p", { srcset: "www.excluded.com,alsoexcluded.com" }),
    React.createElement("p", { style: "excluded attribute style" }))
);

// Excluded JSX attributes, after transpilation with 'jsx: react-jsx'.
const excludedAttributesAfterJsxReactJsx = (
  _jsxs("div", {
    children: [
      _jsx("p", { class: "excluded" }),
      _jsx("p", { d: "excluded attribute d" }),
      _jsx("p", { d: "M 10,50\n           Q 25,25 40,50\n           t 30,0 30,0 30,0 30,0 30,0" }),
      _jsx("p", { d: "M 15,1\n           l -4,8 8,0 -4,-8" }),
      _jsx("p", { d: "M150 5 L75 200 L225 200 Z" }),
      _jsx("p", { id: "excluded" }),
      _jsx("p", { src: "www.excluded.com" }),
      _jsx("p", { srcset: "www.excluded.com,alsoexcluded.com" }),
      _jsx("p", { style: "excluded attribute style" })
    ]
  })
);

// Excluded JSX attributes, after transpilation with 'jsx: react-jsxdev'.
const excludedAttributesAfterJsxReact = (
  _jsxDEV("div", {
    children: [
      _jsxDEV("p", { class: "excluded" }, void 0, false, { fileName: _jsxFileName, lineNumber: 3, columnNumber: 5 }, this),
      _jsxDEV("p", { d: "excluded attribute d" }, void 0, false, { fileName: _jsxFileName, lineNumber: 4, columnNumber: 5 }, this),
      _jsxDEV("p", { d: "M 10,50\n           Q 25,25 40,50\n           t 30,0 30,0 30,0 30,0 30,0" }, void 0, false, { fileName: _jsxFileName, lineNumber: 5, columnNumber: 5 }, this),
      _jsxDEV("p", { d: "M 15,1\n           l -4,8 8,0 -4,-8" }, void 0, false, { fileName: _jsxFileName, lineNumber: 9, columnNumber: 5 }, this),
      _jsxDEV("p", { d: "M150 5 L75 200 L225 200 Z" }, void 0, false, { fileName: _jsxFileName, lineNumber: 12, columnNumber: 5 }, this),
      _jsxDEV("p", { id: "excluded" }, void 0, false, { fileName: _jsxFileName, lineNumber: 13, columnNumber: 5 }, this),
      _jsxDEV("p", { src: "www.excluded.com" }, void 0, false, { fileName: _jsxFileName, lineNumber: 14, columnNumber: 5 }, this),
      _jsxDEV("p", { srcset: "www.excluded.com,alsoexcluded.com" }, void 0, false, { fileName: _jsxFileName, lineNumber: 15, columnNumber: 5 }, this),
      _jsxDEV("p", { style: "excluded attribute style" }, void 0, false, { fileName: _jsxFileName, lineNumber: 16, columnNumber: 5 }, this)]
  }, void 0, true, { fileName: _jsxFileName, lineNumber: 1, columnNumber: 42 }, this)
);

// Excluded code-like strings:
const excludedCode = [
  `"use strict"; function foo(){}`
];

// Excluded large strings:
const excludedLargeStrings = [
  'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'
];
