// @ts-nocheck
import { type Todo, type Filter } from "../types";

import { validateTodo as __validateTodo, parseTodo as __parseTodo, parseTodos as __parseTodos, validate as __validate, display as __display, filterBy as __filterBy, remaining as __remaining, stats as __stats, search as __search } from "../todo";

const item = { id: "1", text: "Buy milk", done: false };

const _name = item.text;

const _done = item.done;

const _good = { id: "1", text: "hi", done: false };

function describeFilter(f: Filter): string {
  return f.tag === "All" ? "all" : f.tag === "Active" ? "active" : f.tag === "Completed" ? "completed" : (() => { throw new Error("non-exhaustive match"); })();
}

function abs(x: number): number {
  return x > 0 === true ? x : x > 0 === false ? 0 - x : (() => { throw new Error("non-exhaustive match"); })();
}

function double(x: number): number {
  return x * 2;
}

const _piped = double(5);

const myVal = 5;

function greet(): string {
  return "hello";
}
