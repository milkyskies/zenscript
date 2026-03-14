---
title: Syntax Reference
---

## Comments

```floe
// Line comment
/* Block comment */
/* Nested /* block */ comments */
```

## Declarations

### Const

```floe
const x = 42
const name: string = "hello"
export const PI = 3.14159

// Destructuring
const [a, b] = pair
const { name, age } = user
```

### Function

```floe
function name(param: Type): ReturnType {
  body
}

export function name(param: Type): ReturnType {
  body
}

async function name(): Promise<T> {
  await expr
}
```

### Type

```floe
// Record
type User = {
  name: string,
  email: string,
}

// Union
type Shape =
  | Circle(radius: number)
  | Rectangle(width: number, height: number)

// Alias
type Name = string

// Brand
type UserId = Brand<string, "UserId">

// Opaque
opaque type Email = string
```

## Expressions

### Literals

```floe
42              // number
3.14            // number
"hello"         // string
`hello ${name}` // template literal
true            // bool
false           // bool
[1, 2, 3]      // array
```

### Operators

```floe
a + b    a - b    a * b    a / b    a % b   // arithmetic
a == b   a != b   a < b    a > b             // comparison
a <= b   a >= b                               // comparison
a && b   a || b   !a                          // logical
a |> f                                        // pipe
expr?                                         // unwrap
```

### Pipe

```floe
value |> transform
value |> f(other_arg, _)   // placeholder
a |> b |> c                // chaining
```

### Match

```floe
match expr {
  pattern -> body,
  pattern -> body,
  _ -> default,
}
```

### If/Else

```floe
if condition {
  then_expr
} else {
  else_expr
}
```

### Function Call

```floe
f(a, b)
f(name: value)     // named argument
Constructor(a: 1)  // record constructor
Constructor(..existing, a: 2)  // spread + update
```

### Built-in Constructors

```floe
Ok(value)     // Result success
Err(error)    // Result failure
Some(value)   // Option present
None          // Option absent
```

### Arrow Function

```floe
(a, b) => a + b
x => x * 2
(x: number): number => x + 1
```

### JSX

```floe
<Component prop={value}>children</Component>
<div className="box">text</div>
<Input />
<>fragment</>
```

## Imports

```floe
import { name } from "module"
import { name as alias } from "module"
import { a, b, c } from "module"
```

## Patterns

```floe
42                    // literal
"hello"               // string literal
true                  // boolean literal
x                     // binding
_                     // wildcard
Ok(x)                 // variant
Some(inner)           // option
{ field, other }      // record destructure
1..10                 // range
```
