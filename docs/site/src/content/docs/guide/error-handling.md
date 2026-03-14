---
title: Error Handling
---

Floe replaces exceptions with `Result<T, E>` and replaces null checks with `Option<T>`. Every error path is visible in the type system.

## Result

```floe
fn divide(a: number, b: number) -> Result<number, string> {
  match b {
    0 -> Err("division by zero"),
    _ -> Ok(a / b),
  }
}
```

You **must** handle the result:

```floe
match divide(10, 3) {
  Ok(value) -> console.log(value),
  Err(msg) -> console.error(msg),
}
```

Ignoring a `Result` is a compile error:

```floe
// Error: Result must be handled
divide(10, 3)
```

## The `?` Operator

Propagate errors early instead of nesting matches:

```floe
fn processOrder(id: string) -> Result<Receipt, Error> {
  const order = fetchOrder(id)?       // returns Err early if it fails
  const payment = chargeCard(order)?  // same here
  return Ok(Receipt(order, payment))
}
```

The `?` operator:
- On `Ok(value)`: unwraps to `value`
- On `Err(e)`: returns `Err(e)` from the enclosing function

Using `?` outside a function that returns `Result` is a compile error.

## Option

```floe
fn findUser(id: string) -> Option<User> {
  match users |> find(.id == id) {
    Some(user) -> Some(user),
    None -> None,
  }
}
```

Handle with match:

```floe
match findUser("123") {
  Some(user) -> greet(user.name),
  None -> greet("stranger"),
}
```

## npm Interop

When importing from npm packages, Floe automatically wraps nullable types:

```floe
import { getElementById } from "some-dom-lib"
// .d.ts says: getElementById(id: string): Element | null
// Floe sees: getElementById(id: string): Option<Element>
```

The boundary wrapping also converts:
- `T | undefined` to `Option<T>`
- `any` to `unknown`

This means npm libraries work transparently with Floe's type system.

## Comparison with TypeScript

| TypeScript | Floe |
|---|---|
| `T \| null` | `Option<T>` |
| `try/catch` | `Result<T, E>` |
| `?.` optional chain | `match` on `Option` |
| `!` non-null assertion | Not available (handle the case) |
| `throw new Error()` | `Err(...)` |
