#!/bin/bash
# Security check hook - prevents editing sensitive files

set -euo pipefail

# Read JSON input from stdin
input=$(cat)

# Extract file path from tool input
file_path=$(echo "$input" | jq -r '.tool_input.file_path // .tool_input.file // ""')

# List of sensitive file patterns
SENSITIVE_PATTERNS=(
  "\.env$"
  "\.env\..*"
  "credentials\.json"
  "secrets\..*"
  "\.pem$"
  "\.key$"
  "id_rsa"
  "\.password"
)

# Check if file matches sensitive patterns
for pattern in "${SENSITIVE_PATTERNS[@]}"; do
  if echo "$file_path" | grep -qE "$pattern"; then
    echo "❌ BLOCKED: Attempting to modify sensitive file: $file_path" >&2
    echo "This file contains sensitive information and should not be modified by AI." >&2
    echo "If you need to modify it, please edit it manually." >&2
    exit 2  # Exit code 2 = blocking error
  fi
done

# Check for path traversal attempts
if echo "$file_path" | grep -q "\.\."; then
  echo "❌ BLOCKED: Path traversal detected in: $file_path" >&2
  exit 2
fi

exit 0
