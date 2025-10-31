# Python Project Memory

## Build & Development Commands

- **Format**: `black .` and `ruff check --fix .`
- **Test**: `pytest` or `python -m pytest`
- **Type Check**: `mypy .`
- **Lint**: `ruff check .`
- **Install**: `pip install -r requirements.txt` or `poetry install`

## Code Style Guidelines

### General
- Use **4 spaces** for indentation (PEP 8)
- Use **Black** for code formatting
- Use **Ruff** for linting
- Follow **PEP 8** style guide

### Python Specifics
- Type hints required for all function signatures
- Use `pathlib` instead of `os.path`
- Prefer f-strings over `.format()` or `%`
- Use context managers (`with`) for resource handling
- List comprehensions for simple transformations

### Naming Conventions
- **snake_case**: Functions, variables, methods
- **PascalCase**: Classes
- **UPPER_SNAKE_CASE**: Constants
- **_leading_underscore**: Private methods/variables

### Import Style
```python
# Standard library
import os
from pathlib import Path

# Third-party
import requests
from fastapi import FastAPI

# Local
from .models import User
from .utils import helper_function
```

## Project Architecture

- **Source**: `src/` or package root
- **Tests**: `tests/` or `test_*.py`
- **Configuration**: `pyproject.toml` or `setup.py`
- **Scripts**: `scripts/`

## Common Patterns

### Error Handling
```python
try:
    result = risky_operation()
except SpecificError as e:
    logger.error(f"Operation failed: {e}")
    raise
```

### Async/Await
```python
async def fetch_data(url: str) -> dict:
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            return await response.json()
```

## Testing

- Use `pytest` for testing
- Test file naming: `test_*.py`
- Use fixtures for setup/teardown
- Aim for >80% coverage
- Use parametrize for multiple test cases

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
