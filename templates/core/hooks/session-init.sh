#!/bin/bash
# Session initialization hook - loads project context at session start

set -euo pipefail

# Read JSON input
input=$(cat)

cwd=$(echo "$input" | jq -r '.cwd')

# Gather project context
echo "🚀 Initializing Claude Code session..."

# Git status
if [ -d "$cwd/.git" ]; then
  echo ""
  echo "📊 Git Status:"
  cd "$cwd" && git -c color.status=always status -sb
  echo ""
  echo "🌿 Current branch: $(git branch --show-current)"
fi

# Check for language-specific files
echo ""
echo "🔍 Detected project type:"
if [ -f "$cwd/package.json" ]; then
  echo "  - Node.js/TypeScript project"
  if command -v node &> /dev/null; then
    echo "  - Node version: $(node --version)"
  fi
elif [ -f "$cwd/go.mod" ]; then
  echo "  - Go project"
  if command -v go &> /dev/null; then
    echo "  - Go version: $(go version)"
  fi
elif [ -f "$cwd/Cargo.toml" ]; then
  echo "  - Rust project"
  if command -v rustc &> /dev/null; then
    echo "  - Rust version: $(rustc --version)"
  fi
elif [ -f "$cwd/pyproject.toml" ] || [ -f "$cwd/requirements.txt" ]; then
  echo "  - Python project"
  if command -v python3 &> /dev/null; then
    echo "  - Python version: $(python3 --version)"
  fi
fi

# Check for Claude Code configuration
if [ -d "$cwd/.claude" ]; then
  echo ""
  echo "✅ Claude Code configuration found"
  if [ -f "$cwd/.claude/CLAUDE.md" ]; then
    echo "  - Project memory loaded"
  fi
fi

echo ""
echo "✨ Session ready!"

exit 0
