---
description: Analyze project dependencies and their relationships
argument-hint: [package-name]
---

Analyze the project's dependencies. If a specific package name is provided in $ARGUMENTS, focus on that package.

Tasks:
1. Identify all dependencies (production and development)
2. Check for:
   - Outdated packages
   - Security vulnerabilities
   - Unused dependencies
   - Duplicate dependencies (same package, different versions)
   - Heavy packages that could be replaced with lighter alternatives
3. Analyze dependency tree depth
4. Identify circular dependencies
5. Provide recommendations for optimization

If analyzing a specific package ($ARGUMENTS), provide:
- Why it's used (which files import it)
- Version information
- Size impact
- Alternatives if applicable

Use modern CLI tools: `rg` for searching, `fd` for finding files.
