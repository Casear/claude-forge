---
description: Run all tests with coverage
allowed-tools: Bash
---

!npm test -- --coverage || jest --coverage

Run the full test suite with coverage reporting.

If tests fail, analyze the failures and suggest fixes.
