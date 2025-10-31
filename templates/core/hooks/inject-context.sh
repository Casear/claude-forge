#!/bin/bash
# Context injection hook - adds project context to user prompts

set -euo pipefail

# Read JSON input
input=$(cat)

cwd=$(echo "$input" | jq -r '.cwd')
prompt=$(echo "$input" | jq -r '.prompt')

# Only inject context for certain types of prompts
# Skip for simple queries or when context is already clear
if echo "$prompt" | grep -qiE "(what|how|why|explain|help)"; then
  # These are questions that might benefit from context
  :
else
  # For implementation requests, don't inject (it's already clear from CLAUDE.md)
  exit 0
fi

# Build context snippet
context=""

# Add current directory context
if [ -f "$cwd/package.json" ]; then
  pkg_name=$(jq -r '.name // "unknown"' "$cwd/package.json")
  context="${context}Current project: $pkg_name (Node.js)\n"
fi

# Output context as additional context to the prompt
if [ -n "$context" ]; then
  echo "$context"
fi

exit 0
