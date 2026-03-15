---
paths:
  - "**/*.fl"
---

# Floe File Quality Gate

When creating or modifying `.fl` files, **always run these commands** before considering the work done:

```bash
floe fmt <file-or-directory>
floe check <file-or-directory>
floe build <file-or-directory>
```

Order: fmt -> check -> build. Fix any errors before proceeding.

This applies to example apps, test fixtures, and any other `.fl` files touched during a task.
