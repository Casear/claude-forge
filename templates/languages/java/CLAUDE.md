# Java Project Memory

## Build & Development Commands

- **Build**: `mvn clean install` or `gradle build`
- **Test**: `mvn test` or `gradle test`
- **Format**: `mvn formatter:format` or use IDE formatter
- **Run**: `mvn spring-boot:run` or `java -jar target/app.jar`

## Code Style

- Use **4 spaces** indentation
- Follow **Java Code Conventions**
- Use **camelCase** for variables/methods
- Use **PascalCase** for classes
- Use **UPPER_SNAKE_CASE** for constants

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
