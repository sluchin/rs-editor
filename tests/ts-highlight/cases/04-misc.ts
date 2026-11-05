#!/usr/bin/env node
// tslint:disable-next-line
/* tslint:enable */
const o = { class: 1, if: 2, name: "x", nested: { type: 3 } };
const total = a / b / c;
const r2 = x.replace(/\/+$/, "") + y.split(/[,;]/).length;
const m = str.match(/a\/b/g);
return /x/.test(s);
export function foo(a, b): void {}
export default function bar(x: Foo[], cb: (n: number) => void): Promise<void> { return null; }
function* gen() { yield 1; yield* other(); }
async function load(url: string) { const res = await fetch(url); return res.json(); }
class Repo<T extends Base> extends Store<T> implements IRepo, ISave {
  private readonly items: Item[] = [];
  static create(): Repo<Item> { return new Repo<Item>(); }
  get size(): number { return this.items.length; }
  add(item: Item, index?: number): void { this.items.push(item); }
  find(id: string): Item | undefined { return this.items.find(i => i.id === id); }
}
namespace NS { export const k = 1; }
declare module "x" { export function y(): void; }
abstract class Shape { abstract area(): number; }
let tpl = `a ${b} c ${`inner ${d}`} e`;
let obj = { fn: function () { return 1; }, other: function named(p) {} };
for (const x of list) { if (x === null) continue; }
for each (var k in obj) {}
try { throw new Error("bad"); } catch (e) { console.error(e); } finally {}
x = y ? foo(1) : bar(2);
const big = 1_000n + 0xff + 1e3;
@Injectable()
export class Svc { constructor(private http: HttpClient, public name: string) {} }
