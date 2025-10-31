# TypeScript/JavaScript Project Memory

## Build & Development Commands

- **Build**: `npm run build` or `pnpm build` or `yarn build`
- **Test**: `npm test` or `pnpm test` or `yarn test`
- **Lint**: `npm run lint` or `eslint . --fix`
- **Type Check**: `tsc --noEmit`
- **Dev Server**: `npm run dev`

## Code Style Guidelines

### General
- Use **2 spaces** for indentation
- Use **ESLint** + **Prettier** for formatting
- Follow **TypeScript strict mode**
- Prefer `const` over `let`, avoid `var`

### TypeScript Specifics
- Always use type annotations for function parameters and return types
- Use interfaces for object shapes, types for unions/intersections
- Prefer `unknown` over `any`
- Use utility types: `Partial<T>`, `Pick<T>`, `Omit<T>`, etc.

### Naming Conventions
- **PascalCase**: Classes, Interfaces, Types, React Components
- **camelCase**: Variables, functions, methods
- **UPPER_SNAKE_CASE**: Constants
- Prefix interfaces with `I` only if there's ambiguity

### Import Style
- Use ES modules syntax: `import { x } from 'module'`
- Group imports: external → internal → relative
- Use path aliases (e.g., `@/` for src/) if configured

### React (if applicable)
- Functional components with hooks
- Use TypeScript for prop types (no PropTypes)
- Prefer named exports over default exports
- Keep components small and focused

## Project Architecture

- **Source code**: `src/`
- **Tests**: `__tests__/` or `*.test.ts` or `*.spec.ts`
- **Types**: `src/types/` or `*.d.ts`
- **Components**: `src/components/` (if React/Vue)
- **Utilities**: `src/utils/` or `src/lib/`
- **API**: `src/api/` or `src/services/`

## Common Patterns

### Error Handling
```typescript
try {
  await riskyOperation();
} catch (error) {
  if (error instanceof SpecificError) {
    // Handle specific error
  }
  throw error; // Re-throw if can't handle
}
```

### Async/Await
- Always use `async/await` over raw Promises
- Handle errors with try/catch
- Use `Promise.all()` for parallel operations

### TypeScript Best Practices
- Use strict null checks
- Avoid type assertions (`as`) unless necessary
- Use type guards for narrowing
- Leverage union types and discriminated unions

## Testing

- Write tests for all new features
- Aim for >80% code coverage
- Test file naming: `feature.test.ts` or `feature.spec.ts`
- Use descriptive test names: `it('should do X when Y happens')`

## Dependencies

- Keep dependencies up-to-date
- Avoid installing unnecessary packages
- Check bundle size impact for frontend projects
- Use dev dependencies appropriately

## Security

- Never commit `.env` files
- Validate all user inputs
- Sanitize data before rendering (XSS prevention)
- Use environment variables for secrets
- Keep dependencies updated for security patches

## Performance

- Avoid unnecessary re-renders (React)
- Use code splitting for large bundles
- Lazy load components/routes when appropriate
- Memoize expensive computations
- Optimize images and assets

## 🚫 CLI Tool Usage (When Using Bash)

**IMPORTANT**: Claude Code has built-in tools (Grep, Glob, Read) that are already optimized.
The rules below apply ONLY when Claude needs to use the Bash tool directly.

When using the Bash tool for operations:

**Directory Listing:**
- ✅ ALWAYS: `eza -la --icons --git`
- ❌ NEVER: `ls -la` or `ls`

**Disk Usage:**
- ✅ ALWAYS: `dust -d 2`
- ❌ NEVER: `du -sh` or `du`

**File Viewing (for display purposes):**
- ✅ ALWAYS: `bat filename`
- ❌ NEVER: `cat filename`

**File Search (if not using Glob tool):**
- ✅ ALWAYS: `fd pattern`
- ❌ NEVER: `find . -name pattern`

**Text Search (if not using Grep tool):**
- ✅ ALWAYS: `rg pattern`
- ❌ NEVER: `grep pattern`

For complete modern CLI tools reference: @../steering.md
