import type { Foo, Bar } from './types';
import * as path from "path";
export type Handler<T = unknown> = (req: Request, res: Response) => Promise<T>;
export interface Options extends Base {
  readonly name?: string;
  retries: number;
  onDone(result: Result): void;
  [key: string]: unknown;
}
export enum Level { Debug = 1, Info, Warn }
const DEFAULTS: Readonly<Options> = { name: "app", retries: 3, onDone() {} };
export async function* stream<T>(items: AsyncIterable<T>, opts?: Partial<Options>): AsyncGenerator<T> {
  for await (const item of items) {
    yield item;
  }
}
export class Queue<T> {
  #items: T[] = [];
  private static instance?: Queue<unknown>;
  constructor(private readonly limit: number = 10) {}
  push = (value: T): boolean => {
    if (this.#items.length >= this.limit) return false;
    this.#items.push(value);
    return true;
  };
  public async drain(cb: (v: T) => Promise<void>): Promise<void> {
    while (this.#items.length > 0) {
      await cb(this.#items.shift()!);
    }
  }
  static get<U>(): Queue<U> { return (Queue.instance ??= new Queue()) as Queue<U>; }
}
function isString(x: unknown): x is string { return typeof x === "string"; }
const result = items.filter((x) => x != null).map(x => x.toString()).join(", ");
const value = obj?.a?.[0] ?? defaultValue;
switch (kind) { case "a": break; default: throw new TypeError(`bad ${kind}`); }
label: for (let i = 0, j = 10; i < j; i++) { continue label; }
type Mapped = { [K in keyof T]?: T[K] };
let tuple: [string, number] = ["a", 1];
const enum E { A }
declare const VERSION: string;
export { stream as default };
if (a) { b(); } else if (c) { d(); } else { e(); }
