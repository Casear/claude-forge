#!/bin/bash
# Auto-format TypeScript/JavaScript files with Prettier before editing

set -euo pipefail

input=$(cat)
file_path=$(echo "$input" | jq -r '.tool_input.file_path // .tool_input.file // ""')

# Only format TS/JS files
if echo "$file_path" | grep -qE '\.(ts|tsx|js|jsx|mjs|cjs)$'; then
  if command -v prettier &> /dev/null; then
    prettier --write "$file_path" 2>/dev/null || true
  elif command -v npx &> /dev/null; then
    npx -y prettier --write "$file_path" 2>/dev/null || true
  fi
fi

exit 0
