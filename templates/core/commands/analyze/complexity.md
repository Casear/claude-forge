---
description: Analyze code complexity and identify complex functions/modules
argument-hint: [file-path]
---

Analyze code complexity. If a file path is provided in $ARGUMENTS, focus on that file.

Tasks:
1. Identify functions/methods with high cyclomatic complexity
2. Find deeply nested code blocks
3. Locate long functions (>50 lines)
4. Detect code smells:
   - Long parameter lists (>3-4 parameters)
   - Deep nesting (>3 levels)
   - Large classes/modules
   - God objects
5. Provide refactoring suggestions

For each complex piece of code, suggest:
- How to simplify
- Potential extraction of functions
- Design pattern opportunities

Use file references like `file.ts:123` for specific locations.

If analyzing the whole project, provide a summary of the top 10 most complex files.
