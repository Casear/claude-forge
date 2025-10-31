# Elixir Project Memory

## Build & Development Commands

- **Format**: `mix format`
- **Test**: `mix test`
- **Lint**: `mix credo`
- **Compile**: `mix compile`
- **Run**: `mix run` or `iex -S mix`

## Code Style

- Use **2 spaces** indentation
- Use **snake_case** for functions/variables
- Use **PascalCase** for modules
- Follow **Elixir Style Guide**
- Use pattern matching extensively

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
