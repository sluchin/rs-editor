// line comment
/* block
   comment */
/** doc comment */
import { a } from "mod";
const s = 'single' + "double \" esc";
const t = `template ${x} and ${y + 1} end`;
let re = /ab+c[/]\d/gi;
if (x) { return 1; }
class Foo extends Bar implements Baz {
  private x: number = 1;
  public static readonly y = true;
  constructor(a: string) { super(); this.x = 1; }
}
interface Shape { area(): number }
type Id = string | number;
enum Color { Red, Green }
function hello(a: number, b: string): void { console.log(a); }
const f = (a: number) => a + 1;
let v = new Map();
if (v instanceof Map) {}
const n = null, u = undefined, i = Infinity;
export default class {}
@Component({})
class C {}
