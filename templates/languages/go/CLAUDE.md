# Go Project Memory

## Build & Development Commands

- **Format**: `go fmt ./...` or `gofmt -w .`
- **Test**: `go test ./...` or `go test -v ./...`
- **Vet**: `go vet ./...`
- **Lint**: `golangci-lint run`
- **Build**: `go build` or `go build -o app`
- **Run**: `go run main.go`

## Code Style Guidelines

### General
- Use **tabs** for indentation (Go standard)
- Use **gofmt** for formatting (automatic)
- Follow **Effective Go** guidelines
- Keep packages focused and small

### Go Specifics
- Error handling: always check errors
- Use meaningful variable names
- Keep functions small
- Use interfaces for abstraction
- Prefer composition over inheritance

### Naming Conventions
- **camelCase**: Unexported names
- **PascalCase**: Exported names
- **ALL_CAPS**: Never use (not idiomatic Go)
- Short names for short scopes: `i`, `err`, `ok`

### Error Handling
```go
result, err := operation()
if err != nil {
    return fmt.Errorf("operation failed: %w", err)
}
```

## Project Architecture

- **cmd/**: Main applications
- **pkg/**: Library code
- **internal/**: Private code
- **api/**: API definitions

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
