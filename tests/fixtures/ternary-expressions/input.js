const tag = () => 'result';

export const arnold = "arnold" ? "betty" : "charles";
export const danielle = tag`danielle` ? tag`emmett` : tag`francine`;
export const george = 'george' ? ('harry', 'irina') : ('jack', 'katherine');
export const leopold = { 'marcia': 'nathan' } ? { 'ophelia': 'peter' } : { 'quinn': 'richard' };
