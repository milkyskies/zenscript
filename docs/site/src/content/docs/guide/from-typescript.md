---
title: Migrating from TypeScript
---

Floe is designed to be familiar to TypeScript developers. This guide covers the key differences.

## What Stays the Same

- Import/export syntax
- Arrow functions
- Template literals
- JSX
- Async/await
- Type annotations
- Generics

## What Changes

### `const` only

```typescript
// TypeScript
let count = 0
count += 1

// Floe — no let, no mutation
const count = 0
const newCount = count + 1
```

### `bool` instead of `boolean`

```typescript
// TypeScript
const active: boolean = true

// Floe
const active: bool = true
```

### `==` is `===`

Floe's `==` compiles to `===`. There is no loose equality.

```floe
// Floe
x == y    // compiles to: x === y
x != y    // compiles to: x !== y
```

### Pipes instead of method chains

```typescript
// TypeScript
const result = users
  .filter(u => u.active)
  .map(u => u.name)
  .join(", ")

// Floe
const result = users
  |> filter(u => u.active)
  |> map(u => u.name)
  |> join(", ")
```

### Pattern matching instead of switch

```typescript
// TypeScript
switch (action.type) {
  case "increment": return state + 1
  case "decrement": return state - 1
  default: return state
}

// Floe
match action.type {
  "increment" -> state + 1,
  "decrement" -> state - 1,
  _ -> state,
}
```

### Result instead of try/catch

```typescript
// TypeScript
try {
  const data = await fetchData()
  return data
} catch (e) {
  return null
}

// Floe
match await fetchData() {
  Ok(data) -> Some(data),
  Err(_) -> None,
}
```

### Option instead of null

```typescript
// TypeScript
function find(id: string): User | null {
  return users.find(u => u.id === id) ?? null
}

// Floe
function find(id: string): Option<User> {
  match users |> find(u => u.id == id) {
    Some(user) -> Some(user),
    None -> None,
  }
}
```

## What's Removed

| Feature | Why | Alternative |
|---------|-----|-------------|
| `let` / `var` | Mutation bugs | `const` only |
| `class` | Complex inheritance hierarchies | Functions + records |
| `this` | Implicit context bugs | Explicit parameters |
| `any` | Type safety escape | `unknown` + narrowing |
| `null` / `undefined` | Billion-dollar mistake | `Option<T>` |
| `enum` | Quirky JS behavior | Union types |
| `interface` | Redundant | `type` |
| `switch` | No exhaustiveness, fall-through | `match` |
| `for` / `while` | Mutation-heavy | Pipes + map/filter/reduce |
| `throw` | Invisible error paths | `Result<T, E>` |

## Incremental Adoption

Floe compiles to `.ts/.tsx`, so you can adopt it file by file:

1. Add `floe` to your project
2. Write new files as `.fl`
3. Compile them alongside your existing `.ts` files
4. Your build tool (Vite, Next.js) treats the output as normal TypeScript
