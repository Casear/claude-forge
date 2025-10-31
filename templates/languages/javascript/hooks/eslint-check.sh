#!/bin/bash
# Run ESLint after editing JavaScript files

set -euo pipefail

input=$(cat)
file_path=$(echo "$input" | jq -r '.tool_input.file_path // .tool_input.file // ""')

# Only lint JS files
if echo "$file_path" | grep -qE '\.(js|jsx|mjs|cjs)$'; then
  if command -v eslint &> /dev/null; then
    if ! eslint "$file_path" 2>&1; then
      echo "⚠️  ESLint issues found in $file_path" >&2
      echo "Run /lint-fix to automatically fix issues" >&2
    fi
  fi
fi

exit 0
