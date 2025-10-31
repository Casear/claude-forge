---
description: Find duplicate code and suggest refactoring opportunities
---

Find duplicate or similar code blocks across the project.

Tasks:
1. Search for:
   - Exact duplicate code blocks (>5 lines)
   - Similar logic with minor variations
   - Duplicate utility functions
   - Repeated patterns that could be abstracted

2. For each duplicate found, provide:
   - Locations (file:line)
   - Code snippet
   - Refactoring suggestion (extract to shared utility, use existing function, etc.)

3. Identify opportunities for:
   - Shared utilities
   - Base classes/interfaces
   - Composition over duplication
   - Configuration over code

Use `rg` for pattern matching across files.

Prioritize by impact (most duplicated = highest priority).
