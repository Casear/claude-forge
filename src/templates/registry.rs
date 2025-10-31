use anyhow::{Context, Result};

use crate::core::Language;

pub struct TemplateRegistry;

impl TemplateRegistry {
    pub fn new() -> Self {
        Self
    }

    pub fn get_language_claude_md(&self, language: Language) -> Result<String> {
        // TODO: Load from embedded templates
        // For now, return a basic template
        let content = match language {
            Language::Rust => include_str!("../../templates/languages/rust/CLAUDE.md.template"),
            Language::TypeScript => include_str!("../../templates/languages/typescript/CLAUDE.md.template"),
            _ => Self::get_generic_template(language),
        };

        Ok(content.to_string())
    }

    fn get_generic_template(language: Language) -> &'static str {
        // Generic template fallback
        match language {
            Language::Python => r#"# Python Project Memory

## Build & Development Commands

- **Format**: `black .` and `ruff check --fix .`
- **Test**: `pytest`
- **Type Check**: `mypy .`

## Code Style Guidelines

- Use **4 spaces** for indentation (PEP 8)
- Use **Black** for formatting
- Follow **PEP 8** style guide

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

**File Viewing:**
- ✅ ALWAYS: `bat filename`
- ❌ NEVER: `cat filename`

**File Search:**
- ✅ ALWAYS: `fd pattern`
- ❌ NEVER: `find . -name pattern`

**Text Search:**
- ✅ ALWAYS: `rg pattern`
- ❌ NEVER: `grep pattern`
"#,
            _ => r#"# Project Memory

## Build & Development Commands

- Build: [Configure based on project]
- Test: [Configure based on project]

## Code Style Guidelines

- Follow language conventions
- Use consistent naming
- Keep functions small and focused

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
"#,
        }
    }

    pub fn get_claudeignore_template(&self) -> Result<String> {
        Ok(r#"# Claude Code ignore file
# Files and directories to ignore

# Dependencies
node_modules/
vendor/
target/
dist/
build/
.next/

# Environment files
.env
.env.local
.env.*.local

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Logs
*.log
logs/

# Test coverage
coverage/
.nyc_output/

# Build artifacts
*.pyc
__pycache__/
*.class
*.o
*.so
"#.to_string())
    }

    pub fn get_default_agent_template(&self, name: &str) -> Result<String> {
        let template = match name {
            "code-reviewer" => r#"---
name: code-reviewer
description: Reviews code for quality, best practices, and potential issues
tools:
  - Read
  - Grep
  - Glob
---

# Code Reviewer Agent

You are a code review specialist. Your role is to:

1. **Code Quality**: Review code for maintainability and readability
2. **Best Practices**: Ensure code follows language and framework best practices
3. **Bugs**: Identify potential bugs and edge cases
4. **Security**: Check for common security vulnerabilities
5. **Performance**: Suggest performance improvements where applicable

## Review Checklist

- [ ] Code follows project style guidelines
- [ ] Functions are well-named and focused
- [ ] Error handling is appropriate
- [ ] No obvious security vulnerabilities
- [ ] Test coverage is adequate
- [ ] Documentation is clear and complete

## Output Format

Provide feedback in this format:

**Issues Found:**
- [SEVERITY] Description and location

**Suggestions:**
- Improvement recommendations

**Approval:**
- ✅ Approved / ⚠️ Needs changes / ❌ Major issues
"#,
            "security-scanner" => r#"---
name: security-scanner
description: Scans code for security vulnerabilities
tools:
  - Read
  - Grep
---

# Security Scanner Agent

You are a security specialist. Your role is to identify security vulnerabilities.

## Security Checks

1. **Injection Attacks**: SQL, Command, XSS
2. **Authentication**: Password handling, session management
3. **Authorization**: Access control issues
4. **Data Exposure**: Sensitive data leaks
5. **Cryptography**: Weak encryption, hardcoded secrets

## Common Vulnerabilities

- Hardcoded credentials
- SQL injection vulnerabilities
- XSS vulnerabilities
- Insecure dependencies
- Missing input validation
"#,
            _ => r#"---
name: {name}
description: Custom agent
tools:
  - Read
  - Grep
---

# {name}

Describe the agent's purpose and capabilities here.
"#.replace("{name}", name),
        };

        Ok(template.to_string())
    }

    pub fn get_default_command_template(&self, name: &str) -> Result<String> {
        let template = match name {
            "analyze" => r#"---
description: Analyze codebase structure and complexity
allowed-tools: Read,Grep,Glob
---

Analyze the codebase and provide insights on:

1. Project structure
2. Code complexity
3. Dependencies
4. Potential improvements
"#,
            "refactor" => r#"---
description: Suggest refactoring opportunities
allowed-tools: Read,Grep,Glob
---

Identify and suggest refactoring opportunities:

1. Code duplication
2. Long functions
3. Complex conditionals
4. Naming improvements
"#,
            _ => r#"---
description: Custom command
allowed-tools: Read,Grep
---

Describe what this command does.
"#,
        };

        Ok(template.to_string())
    }

    pub fn get_default_hook_template(&self, name: &str, _event: Option<&str>) -> Result<String> {
        let template = match name {
            "format" | "prettier-format" => r#"#!/bin/bash
set -euo pipefail

# Format hook
input=$(cat)
file_path=$(echo "$input" | jq -r '.tool_input.file_path // .tool_input.file // ""')

if [ -n "$file_path" ]; then
    # Run formatter if applicable
    if command -v prettier &> /dev/null; then
        prettier --write "$file_path" 2>/dev/null || true
    fi
fi

exit 0
"#,
            "lint" | "eslint-check" => r#"#!/bin/bash
set -euo pipefail

# Lint check hook
input=$(cat)
file_path=$(echo "$input" | jq -r '.tool_input.file_path // .tool_input.file // ""')

if [ -n "$file_path" ]; then
    # Run linter if applicable
    if command -v eslint &> /dev/null; then
        eslint "$file_path" 2>/dev/null || true
    fi
fi

exit 0
"#,
            _ => r#"#!/bin/bash
set -euo pipefail

# Custom hook
input=$(cat)

# Process input as needed
echo "$input" | jq .

exit 0
"#,
        };

        Ok(template.to_string())
    }
}
"#