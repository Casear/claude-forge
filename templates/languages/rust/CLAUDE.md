# Rust Project Memory

## Build & Development Commands

- **Format**: `cargo fmt`
- **Test**: `cargo test`
- **Clippy**: `cargo clippy -- -D warnings`
- **Build**: `cargo build` or `cargo build --release`
- **Run**: `cargo run`
- **Check**: `cargo check`

## Code Style Guidelines

### General
- Use **4 spaces** for indentation
- Use **rustfmt** for formatting (automatic)
- Follow **Rust API Guidelines**
- Leverage the type system

### Rust Specifics
- Use `Result<T, E>` for error handling
- Prefer `Option<T>` over null patterns
- Use iterators and combinators
- Leverage ownership and borrowing
- Use trait bounds appropriately

### Naming Conventions
- **snake_case**: Functions, variables, modules
- **PascalCase**: Types, traits
- **SCREAMING_SNAKE_CASE**: Constants
- **'a**: Lifetime parameters

### Error Handling
```rust
fn operation() -> Result<T, Error> {
    let result = risky_operation()?;
    Ok(result)
}
```

## Project Architecture

- **src/main.rs**: Binary entry
- **src/lib.rs**: Library entry
- **src/bin/**: Additional binaries
- **tests/**: Integration tests

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
