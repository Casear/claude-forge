---
description: Generate API documentation
argument-hint: [file-or-directory]
---

Generate API documentation for the project. If $ARGUMENTS specifies a file or directory, focus on that.

For each API endpoint/function:

### Endpoint/Function Name

**Description**: What it does

**Parameters/Arguments**:
| Name | Type | Required | Description |
|------|------|----------|-------------|
| ... | ... | ... | ... |

**Returns**: Return type and description

**Example**:
```language
// Usage example with actual code
```

**Errors/Exceptions**: Possible error conditions

Tasks:
1. Scan code for API endpoints, public functions, exported modules
2. Extract function signatures and documentation
3. Generate usage examples
4. Document error handling
5. Include authentication requirements (if API)
6. Add rate limiting info (if applicable)

Format output in Markdown with clear sections and tables.

Use actual code analysis, not placeholders.
