type DictionaryItem = string | TemplateStringsArray;
type DictionaryState = {
  dictionary: Set<string>;
  dirty: boolean;
};

const MATCH_REGEX = /\p{Letter}+|\p{Symbol}+/gu;
const MAX_WORD_LENGTH = 20;

export function processStringToWords(str: string): string[] {
  const words = str.match(MATCH_REGEX);
  if (!words) {
    return [];
  }
  return words
    .filter((word) => word.length <= MAX_WORD_LENGTH)
    .map((word) => word.toLocaleLowerCase());
}

function addWordsToDictionary(words: string[]): void {
  for (const word of words) {
    dictionary.add(word);
  }
}

const dictionary = new Set<string>();
const dictionaryState: DictionaryState = {
  dictionary,
  dirty: false,
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
(globalThis as any).$DD_ALLOW = dictionaryState.dictionary;

export function getDictionaryState(): DictionaryState {
  return dictionaryState;
}

export function flushDictionary(): void {
  processDictionaryQueue();
}

const queue: Array<DictionaryItem[]> = [];
let processDictionaryQueueTimeoutID: ReturnType<typeof setTimeout> | undefined;

function processDictionaryQueue(): void {
  if (!dictionaryState.dirty) {
    return;
  }

  const queueLen = queue.length;
  for (let queueItem = 0; queueItem < queueLen; queueItem++) {
    const entry = queue[queueItem];
    const entryLen = entry.length;
    for (let entryItem = 0; entryItem < entryLen; entryItem++) {
      const value = entry[entryItem];
      if (typeof value === 'string') {
        // Handle strings.
        const words = processStringToWords(value);
        addWordsToDictionary(words);
      } else {
        // Handle tagged template quasi arrays.
        const len = value.length;
        for (let i = 0; i < len; i++) {
          const words = processStringToWords(value[i]);
          addWordsToDictionary(words);
        }
      }
    }
  }

  if (processDictionaryQueueTimeoutID !== undefined) {
    clearTimeout(processDictionaryQueueTimeoutID);
    processDictionaryQueueTimeoutID = undefined;
  }

  queue.length = 0;
  dictionaryState.dirty = false;
}

export function $(
  items: DictionaryItem[] | TemplateStringsArray
): (DictionaryItem[] | TemplateStringsArray) {
  if ((items as TemplateStringsArray).raw) {
    // We're being used as a template tag function. The invocation will look like this:
    //   const D = $('foo', $`bar${0}`, 'baz');
    // In this context, our only role is to extract the TemplateStringsArray array so that
    // the top-level call to $ can make use of it. So, we just need to return our first
    // argument.
    return items;
  }

  // We're being used to construct a dictionary. Add our arguments to the queue and
  // schedule a callback to process them if one isn't already scheduled.
  queue.push(items as DictionaryItem[]);
  if (!dictionaryState.dirty) {
    dictionaryState.dirty = true;
    processDictionaryQueueTimeoutID = setTimeout(processDictionaryQueue, 0);
  }

  return items;
}
