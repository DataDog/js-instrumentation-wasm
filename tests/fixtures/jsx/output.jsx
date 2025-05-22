import { $ } from 'datadog:privacy-helpers';
const D = $([
  "Text content. With more than one line.",
  " entity reference",
  'quotes"inside',
  'with"\'quotes',
  '--top-offset',
  "some content",
  "something'",
  'error-info',
  'value',
  `--is-stuck`,
  `px`,
]);
import {
  AnotherComponent,
  ErrorHandler,
  Flex,
  SomeComponent,
  SomethingElse,
  BLOCK,
  classNames,
  isStuck,
  reportError,
  topOffset,
} from 'framework';

export function MyComponent(props) {
  return (
    <SomeComponent attr={D[6]} with={D[2]}>
      <AnotherComponent attr={D[3]} />
      <SomethingElse style={{
        key: D[8],
        anotherKey: props.key2,
      }}
      >{D[0]}</SomethingElse>
      <ErrorHandler
        onError={(info) => {
          reportError({ [D[7]]: info });
        }}
      />
      <Flex
        style={
          {
            [D[4]]: `${topOffset}px`,
          }
        }
        className={classNames(BLOCK, {
          [`${BLOCK}${D[9]}`]: isStuck,
        })}
      />
      <svg
        fill={props.fill}
        content={(() => { return D[5]; })()}
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 192 192"
        data={D[1]}
      >
        <g>
          <rect x='84' y='156' width='20' height='20' />
        </g>
        <path
          d="M102,140H86v-20c0-13.5,10.1-24,22.9-24c13.2,0,23-3,28.9-9c5.9-5.9,6.1-12.9,6.1-13l0-0.4l0-0.4c0-0.4,0.8-11.6-7.6-20.8
C128.8,44.2,115.9,40,98,40c-20.9,0-35.6,5.7-43.7,16.9c-6,8.3-6.3,17-6.3,17.1L32,74c0-1.3,0.2-13.3,8.7-25.5
C54.6,28.2,79.5,24,98,24c22.7,0,39.7,6,50.5,17.9c12.2,13.4,11.7,29.6,11.5,32.5c-0.1,2.5-0.9,14-10.8,23.9
c-9.1,9.1-22.6,13.7-40.3,13.7c-4.6,0-6.9,4-6.9,8V140z"
        />
      </svg>
    </SomeComponent>
  );
}
