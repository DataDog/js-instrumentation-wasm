const tag = () => 'result';

export const foo = (value) => {
  const emmett = 'emmett';

  switch (value) {
    case "arnold":
      return 1;

    case 'betty':
      return 2;

    case `charles`:
      return 3;

    case tag`danielle`:
      return 4;

    case emmett:
      return 5;

    case'francis':
      return 6;

    default:
      return 7;
  }
};
