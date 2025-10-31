---
description: Run ESLint and automatically fix issues
allowed-tools: Bash
---

!npm run lint -- --fix || npx eslint . --fix

Run ESLint with auto-fix enabled for JavaScript files.

If there are unfixable errors, report them clearly with file:line references.
