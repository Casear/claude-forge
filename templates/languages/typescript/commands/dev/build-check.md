---
description: Check if the project builds successfully
allowed-tools: Bash
---

!tsc --noEmit && npm run build

Tasks:
1. Run TypeScript type checking (tsc --noEmit)
2. Run the build command
3. Report any build errors clearly
4. If successful, report bundle sizes (if available)

If build fails, analyze errors and suggest fixes.
