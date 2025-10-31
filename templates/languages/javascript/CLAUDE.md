# JavaScript/Node.js Project Memory

## Build & Development Commands

- **Start Dev**: `npm run dev` or `node --watch index.js`
- **Start**: `npm start` or `node index.js`
- **Test**: `npm test` or `jest`
- **Lint**: `npm run lint` or `eslint . --fix`
- **Build**: `npm run build` (if using bundler)

## Code Style Guidelines

### General
- Use **2 spaces** for indentation
- Use **ESLint** + **Prettier** for formatting
- Prefer `const` over `let`, avoid `var`
- Use modern ES6+ features

### JavaScript Specifics
- Use ES modules (`import/export`) when possible
- Use async/await over callbacks and raw Promises
- Destructure objects and arrays
- Use template literals for string interpolation
- Prefer arrow functions for callbacks

### Naming Conventions
- **camelCase**: Variables, functions, methods
- **PascalCase**: Classes, React Components
- **UPPER_SNAKE_CASE**: Constants
- **kebab-case**: File names (optional)

### Import Style
```javascript
// ES Modules (preferred)
import { someFunction } from 'module';
import defaultExport from 'module';

// CommonJS (Node.js)
const { someFunction } = require('module');
const express = require('express');

// Group imports
// 1. Node.js built-ins
// 2. External packages
// 3. Internal modules
```

### Common Patterns

#### Error Handling
```javascript
// Async/await with try-catch
async function fetchData() {
  try {
    const response = await fetch(url);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Failed to fetch:', error);
    throw error;
  }
}

// Promise chains
fetchData()
  .then(data => processData(data))
  .catch(error => handleError(error));
```

#### Async Operations
```javascript
// Always prefer async/await
async function getData() {
  const result = await someAsyncOperation();
  return result;
}

// Parallel operations with Promise.all
const [users, posts, comments] = await Promise.all([
  fetchUsers(),
  fetchPosts(),
  fetchComments()
]);
```

#### Null/Undefined Checks
```javascript
// Optional chaining
const userName = user?.profile?.name;

// Nullish coalescing
const port = process.env.PORT ?? 3000;

// Default parameters
function greet(name = 'Guest') {
  return `Hello, ${name}`;
}
```

## Project Architecture

- **Source code**: `src/` or root
- **Tests**: `test/`, `__tests__/`, or `*.test.js`
- **Config**: Root directory (.eslintrc.js, .prettierrc, etc.)
- **Build output**: `dist/`, `build/`
- **Node modules**: `node_modules/` (gitignore)

### Common Project Types

#### Express.js Server
```
src/
├── routes/        # API routes
├── controllers/   # Request handlers
├── models/        # Data models
├── middleware/    # Custom middleware
├── config/        # Configuration
└── app.js         # App setup
```

#### CLI Tool
```
src/
├── commands/      # CLI commands
├── utils/         # Utilities
└── index.js       # Entry point
```

#### Library/Package
```
src/
├── index.js       # Main entry
└── lib/           # Library code
```

## Package Management

### npm Scripts (package.json)
```json
{
  "scripts": {
    "start": "node src/index.js",
    "dev": "node --watch src/index.js",
    "test": "jest",
    "test:watch": "jest --watch",
    "lint": "eslint .",
    "lint:fix": "eslint . --fix",
    "format": "prettier --write ."
  }
}
```

### Dependencies
- Keep `dependencies` minimal (production only)
- Use `devDependencies` for tools (ESLint, Jest, etc.)
- Lock dependencies with `package-lock.json` or `yarn.lock`
- Regularly update packages: `npm outdated`

## Testing

### Jest Configuration
```javascript
// jest.config.js
module.exports = {
  testEnvironment: 'node',
  coverageThreshold: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80
    }
  }
};
```

### Test File Naming
- `feature.test.js` or `feature.spec.js`
- Place tests next to code or in `__tests__/` directory

### Test Structure
```javascript
describe('Feature', () => {
  beforeEach(() => {
    // Setup
  });

  it('should do something', () => {
    expect(result).toBe(expected);
  });

  it('should handle edge case', () => {
    expect(() => fn()).toThrow();
  });
});
```

## Node.js Best Practices

### Environment Variables
```javascript
// Use dotenv for development
require('dotenv').config();

const PORT = process.env.PORT || 3000;
const DB_URL = process.env.DATABASE_URL;

// Never commit .env files
```

### Error Handling
```javascript
// Global error handler (Express)
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Something went wrong!' });
});

// Unhandled rejection handler
process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled Rejection at:', promise, 'reason:', reason);
  process.exit(1);
});
```

### Async Best Practices
```javascript
// ✅ Good: Handle errors
async function goodExample() {
  try {
    await riskyOperation();
  } catch (error) {
    console.error(error);
  }
}

// ❌ Bad: Unhandled promise
async function badExample() {
  await riskyOperation(); // Error not caught!
}
```

## Security

### Input Validation
```javascript
// Use validation libraries
const Joi = require('joi');

const schema = Joi.object({
  email: Joi.string().email().required(),
  password: Joi.string().min(8).required()
});

const { error, value } = schema.validate(req.body);
```

### SQL Injection Prevention
```javascript
// ✅ Good: Parameterized queries
db.query('SELECT * FROM users WHERE id = ?', [userId]);

// ❌ Bad: String concatenation
db.query(`SELECT * FROM users WHERE id = ${userId}`);
```

### XSS Prevention
```javascript
// Sanitize user input
const sanitizeHtml = require('sanitize-html');
const clean = sanitizeHtml(userInput);
```

### Environment Secrets
```javascript
// Never commit secrets
// Use environment variables
// Use secret management (AWS Secrets Manager, etc.)

// ❌ Bad
const API_KEY = 'sk-1234567890';

// ✅ Good
const API_KEY = process.env.API_KEY;
```

## Performance

### Avoid Blocking Operations
```javascript
// ✅ Good: Async file operations
const fs = require('fs').promises;
const data = await fs.readFile('file.txt', 'utf8');

// ❌ Bad: Sync operations (blocks event loop)
const data = fs.readFileSync('file.txt', 'utf8');
```

### Use Streams for Large Data
```javascript
const fs = require('fs');

// For large files
const readStream = fs.createReadStream('large-file.txt');
readStream.pipe(writeStream);
```

### Caching
```javascript
// Memoization
const cache = new Map();

function expensiveOperation(key) {
  if (cache.has(key)) {
    return cache.get(key);
  }
  const result = computeExpensiveResult(key);
  cache.set(key, result);
  return result;
}
```

## Common Packages

### Web Frameworks
- **Express** - Minimal web framework
- **Fastify** - Fast web framework
- **Koa** - Modern middleware framework

### Databases
- **mongoose** - MongoDB ODM
- **pg** - PostgreSQL client
- **mysql2** - MySQL client
- **prisma** - Modern ORM

### Testing
- **Jest** - Testing framework
- **Mocha** + **Chai** - Alternative testing
- **Supertest** - HTTP testing

### Utilities
- **lodash** - Utility functions
- **date-fns** - Date utilities
- **axios** - HTTP client
- **dotenv** - Environment variables

## ESLint Configuration

```javascript
// .eslintrc.js
module.exports = {
  env: {
    node: true,
    es2021: true,
    jest: true
  },
  extends: [
    'eslint:recommended',
    'prettier'
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  rules: {
    'no-console': 'off',
    'no-unused-vars': ['error', { argsIgnorePattern: '^_' }]
  }
};
```

## Prettier Configuration

```javascript
// .prettierrc
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5"
}
```

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

## Common Gotchas

### 1. Asynchronous Execution
```javascript
// ❌ Bad: forEach doesn't wait
items.forEach(async item => {
  await process(item); // Won't wait!
});

// ✅ Good: for...of waits
for (const item of items) {
  await process(item);
}

// ✅ Good: Promise.all for parallel
await Promise.all(items.map(item => process(item)));
```

### 2. Variable Scoping
```javascript
// ❌ Bad: var is function-scoped
for (var i = 0; i < 3; i++) {
  setTimeout(() => console.log(i), 100); // Prints 3, 3, 3
}

// ✅ Good: let is block-scoped
for (let i = 0; i < 3; i++) {
  setTimeout(() => console.log(i), 100); // Prints 0, 1, 2
}
```

### 3. Floating Promises
```javascript
// ❌ Bad: Promise not handled
async function bad() {
  doAsyncThing(); // Floating promise!
}

// ✅ Good: Await or handle
async function good() {
  await doAsyncThing();
  // or
  doAsyncThing().catch(err => console.error(err));
}
```

## Debugging

### Node.js Debugger
```bash
# Start with inspector
node --inspect index.js

# Or use built-in debugger
node inspect index.js
```

### Console Debugging
```javascript
console.log('Simple log');
console.error('Error message');
console.table([{ name: 'Alice' }, { name: 'Bob' }]);
console.time('operation');
// ... code ...
console.timeEnd('operation');
```

## Deployment

### Production Checklist
- [ ] Set `NODE_ENV=production`
- [ ] Use process manager (PM2, systemd)
- [ ] Enable error logging
- [ ] Set up monitoring
- [ ] Use HTTPS
- [ ] Rate limiting
- [ ] Input validation
- [ ] Security headers

### PM2 Example
```bash
npm install -g pm2
pm2 start src/index.js --name my-app
pm2 logs
pm2 restart my-app
```

---

**Pro Tips**:
- Use `node --watch` for auto-reload in development (Node 18+)
- Enable strict mode: `'use strict';`
- Use `nodemon` for older Node versions
- Keep your Node.js version up to date
- Use `nvm` to manage Node versions
