# Example App as Integration Test

When adding or modifying language features, **always update the todo app example** at `examples/todo-app/` to exercise the new feature.

This serves as a real-world integration test — if the example app doesn't pass `floe check`, the feature isn't done.

## Workflow

1. Implement the feature (lexer, parser, checker, codegen)
2. Update the example app to use it
3. Run `floe check examples/todo-app/src/` and verify no errors
4. Commit the example app changes in the same PR

## Files

- `examples/todo-app/src/types.fl` — type definitions (Todo, Filter, Validation)
- `examples/todo-app/src/todo.fl` — for-block functions and helpers
- `examples/todo-app/src/pages/home.fl` — main component using all features

## What to test

- New syntax should appear naturally in the app, not forced
- If the feature doesn't fit the todo app, add a new example file instead
- The app should always pass `floe check` on every file
