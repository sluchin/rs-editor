/**
 * Doc @param {string} name the name
 * @returns {number}
 */
function doc(name: string): number { return 1; }
const s1 = 'it\'s', s2 = "multi
line";
const t1 = `line1
line2 ${a + `x${b}`} \${not} end`;
const t2 = `${x}${y}`;
const d = a / 2 / 3;
const e = (a) / b;
const rx = [/a/, /b\]/ , /[/]/g];
x = y /* c */ / z;
  /re/.test(q);
const ünï = "ü";
const { aa: bb, cc: [dd, ee] } = obj, ff = 1;
let [g1, , h1] = arr;
function f(a = 1, { b, c }: Opts, ...rest: number[]) {}
const fnExpr = function (p: number, q?: string) { return p; };
foo(function (inner) { return inner; });
el.addEventListener("click", (ev) => { handle(ev); });
x.y.z(1)(2);
const arrow2 = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 10));
class A { m(a: number): void; m(a: string): void; m(a: any) {} }
let u: undefined | null = null;
let n = a ? b : c ? d : e;
const o2 = { async method(arg: number) { return arg; }, get prop() { return 1; } };
