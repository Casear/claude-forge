---
description: Extract code into a reusable function
argument-hint: <file-path:line-range>
---

Extract the specified code block into a well-named, reusable function.

Expected format: `file.ts:10-20` or provide the file path and I'll help identify the code to extract.

Process:
1. Analyze the code block in $ARGUMENTS
2. Identify:
   - What the code does (for naming)
   - Input parameters needed
   - Return value
   - Side effects
3. Suggest a descriptive function name following project conventions
4. Create the extracted function with:
   - Clear parameter names
   - Appropriate return type
   - Documentation/comments
5. Replace the original code with function call
6. Consider where to place the function (same file, utility file, etc.)

Follow the project's code style from CLAUDE.md.
