const keys = {
  "1": 1,
  "2": 2,
  "3": 3,
  "4": 12,
  q: 4,
  w: 5,
  e: 6,
  r: 13,
  a: 7,
  s: 8,
  d: 9,
  f: 14,
  z: 10,
  x: 0,
  c: 11,
  v: 15
};
export function translateKey(key) {
  return keys[key];
}
