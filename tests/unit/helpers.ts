import fs from 'node:fs/promises';
import path from 'node:path';
import url from 'node:url';
import { SourceMapConsumer } from 'source-map';
import { expect } from 'vitest';

export interface TestCase {
  code: string;
  config?: Record<string, unknown>;
  dir: string;
  name: string;
}

const CONFIG_FILE = 'config.json';
const INPUT_PREFIX = 'input.';

function readAsJson(
  dir: string,
  file: string,
): Promise<Record<string, unknown> | undefined> {
  const jsonFile = path.join(dir, file);
  return fs.readFile(jsonFile, 'utf-8').then(
    (json) => {
      return JSON.parse(json);
    },
    _ => undefined,
  );
}

function readAsString(
  dir: string,
  file: string,
): Promise<string | undefined> {
  const stringFile = path.join(dir, file);
  return fs.readFile(stringFile, 'utf-8');
}

async function readTestCase(
  baseDir: string,
  dir: string,
): Promise<TestCase> {
  const testCaseDir = path.join(baseDir, dir);
  const testCaseFiles = await fs.readdir(testCaseDir);

  let code: string | undefined;
  let config: Record<string, unknown> | undefined;
  let name: string | undefined;

  for (const file of testCaseFiles) {
    if (file === CONFIG_FILE) {
      config = await readAsJson(testCaseDir, file);
    } else if (file.startsWith(INPUT_PREFIX)) {
      code = await readAsString(testCaseDir, file);
      name = file;
    }
  }

  if (code === undefined || name === undefined) {
    throw new Error(`Found no input file for fixture: '${testCaseDir}'`);
  }

  return { code, config, dir, name };
}

export async function walkDir(
  dir: URL,
  callback: (testCase: TestCase) => Promise<void>,
): Promise<void> {
  const dirs = await fs.readdir(dir);
  const baseDir = url.fileURLToPath(dir);

  for (const dir of dirs) {
    try {
      const testCase = await readTestCase(baseDir, dir);
      await callback(testCase);
    } catch (e) {
      console.error(e);
    }
  }
}

export async function expectEveryPositionToHaveASourceMapping(
  testName: string,
  code: string,
  map: string
): Promise<void> {
  const sourceLines = code.split('\n');
  if (sourceLines.length > 0) {
    const lastLine = sourceLines[sourceLines.length - 1];
    if (lastLine === '' || lastLine.startsWith('//# sourceMapping')) {
      // It's expected that there is no source mapping for a totally empty final line, or
      // for a final line that is itself a source mapping comment.
      sourceLines.pop();
    }
  }

  await SourceMapConsumer.with(map, null, consumer => {
    sourceLines.forEach((line, lineIndex) => {
      for (let columnIndex = 0; columnIndex < line.length; columnIndex++) {
        const originalPosition = consumer.originalPositionFor({
          line: lineIndex + 1,
          column: columnIndex
        });
        const posContext = `line ${lineIndex + 1} column ${columnIndex}`;
        const context = `Source mapping for ${testName} at ${posContext} line '${line}'`;
        expect(originalPosition.source, context).not.toBe(null);
        expect(originalPosition.line, context).not.toBe(null);
        expect(originalPosition.column, context).not.toBe(null);
      }
    });
  });
}
