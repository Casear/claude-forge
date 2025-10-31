---
description: Run linter and automatically fix issues
allowed-tools: Bash
---

!npm run lint -- --fix || eslint . --fix --ext .ts,.tsx,.js,.jsx

If the project uses ESLint, run the linter with auto-fix enabled.

If there are unfixable errors, report them clearly with file:line references.
