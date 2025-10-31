# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-31

### Added

#### Core Features
- Complete Rust CLI tool for managing Claude Code configurations
- Multi-language support (8 languages):
  - TypeScript/JavaScript - Full configuration with agents, commands, hooks
  - Python - CLAUDE.md + format command
  - Go - CLAUDE.md + development guidelines
  - Rust - CLAUDE.md + toolchain config
  - Java - CLAUDE.md + build tools
  - Elixir - CLAUDE.md + mix integration
  - Erlang - CLAUDE.md + rebar config
  - Generic - Universal fallback template

#### Commands
- `claude-forge init` - Initialize Claude Code configuration
  - Auto-detect project language
  - Interactive prompts for configuration
  - Optional CLI tools installation
  - Optional MCP server setup
  - Supports `--minimal` mode for basic setup
  - Supports `--yes` for non-interactive mode

- `claude-forge add` - Add components
  - Add custom subagents
  - Add slash commands
  - Add lifecycle hooks
  - Template-based generation with interactive editing

- `claude-forge tools` - Manage modern CLI tools
  - Check installation status of rg, fd, bat, eza, dust
  - Install missing tools via package managers
  - Update existing tools
  - Platform support: macOS (Homebrew/Cargo), Linux (Cargo)

- `claude-forge mcp` - Manage MCP servers
  - List 18+ recommended MCP servers
  - Add MCP servers to configuration
  - Show current MCP configuration
  - Categorized by: essential, database, dev tools, API, productivity, enterprise

- `claude-forge config` - Configuration management
  - Show current configuration structure
  - Validate configuration files
  - Reset configuration to defaults

- `claude-forge upgrade` - Self-update functionality
  - Download latest release from GitHub
  - Platform detection (macOS ARM64/Intel, Linux x86_64/ARM64)
  - Automatic binary replacement
  - Handle sudo permissions when needed
  - Supports `--dry-run`, `--version`, `--yes` flags

- `claude-forge update` - Update templates
  - Download latest templates from GitHub
  - Store templates in `~/.config/claude-forge/templates/`
  - Independent version tracking from binary
  - Supports `--dry-run`, `--version`, `--force` flags

#### Smart Detection Features
- **Language Detection**
  - Automatic detection based on project files
  - Cargo.toml for Rust
  - package.json for TypeScript/JavaScript
  - go.mod for Go
  - pyproject.toml/setup.py for Python
  - pom.xml/build.gradle for Java
  - mix.exs for Elixir
  - rebar.config for Erlang

- **CLI Tools Detection**
  - Runtime detection of installed modern CLI tools
  - Generates personalized CLAUDE.md based on environment
  - Three modes: all installed, partial, none installed
  - Shows installation suggestions for missing tools
  - Customizes file operation recommendations

#### Templates (38 files)
- **Core Templates**
  - 4 universal subagents (code-reviewer, security-scanner, performance-optimizer, documentation-agent)
  - 6 generic commands (analyze, refactor, docs generation)
  - 3 core hooks (security-check, session-init, inject-context)
  - Configuration templates (CLAUDE.md, .claudeignore)

- **Language-Specific Templates**
  - TypeScript: Full config with React/Next.js support
  - Python: PEP 8 formatting and linting
  - Go: Module structure and best practices
  - Rust: Cargo integration and ownership guidelines
  - Java: Maven/Gradle build tools
  - Elixir: Mix tasks and OTP patterns
  - Erlang: Rebar3 and release handling

#### CI/CD
- **GitHub Actions Workflows**
  - CI pipeline (`ci.yml`) - Tests, clippy, formatting checks
  - Release pipeline (`release.yml`) - Multi-platform binary builds
  - Automated release on version tags (format: X.Y.Z)
  - Cross-compilation for 4 platforms

#### Documentation
- README.md - Project overview and features
- QUICK_START.md - 5-minute getting started guide (Traditional Chinese)
- FAQ.md - Comprehensive Q&A
- CONTRIBUTING.md - Contribution guidelines
- PROJECT_SUMMARY.md - Complete project summary
- MCP_RECOMMENDATIONS.md - 18+ recommended MCP servers with setup
- Multiple guides (SDD workflow, hooks, MCP integration)

### Technical Details

#### Dependencies
- clap 4.5 - CLI argument parsing with derive macros
- tokio 1.35 - Async runtime
- serde 1.0 - Serialization/deserialization
- tera 3.1 - Template engine
- colored 2.1 - Terminal colors
- dialoguer 0.11 - Interactive prompts
- reqwest 0.12 - HTTP client for API calls
- which 7.0 - Find executables in PATH

#### Project Structure
```
claude-forge/
├── src/
│   ├── cli/          # Command implementations (6 commands)
│   ├── core/         # Core logic (language detection, generation)
│   ├── templates/    # Embedded templates (38 files)
│   ├── tools/        # CLI tools management
│   └── utils/        # Utility functions
├── .github/
│   └── workflows/    # CI/CD pipelines
└── templates/        # Source templates for embedding
```

#### Performance
- Binary size: ~5MB (release build)
- Embedded templates: ~30KB (compressed)
- Startup time: <100ms
- Template generation: <500ms

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed
- N/A (initial release)

### Security
- No known security issues

## Release Notes

This is the first release of Claude Forge, a comprehensive CLI tool for managing Claude Code configurations. It provides:

1. **Easy Initialization** - One command to set up complete Claude Code configuration
2. **Multi-Language Support** - 8 languages with smart detection
3. **Smart Environment Detection** - Adapts to your installed tools
4. **Self-Updating** - Keep both binary and templates up to date
5. **Extensible** - Easy to add custom agents, commands, and hooks
6. **Well-Documented** - Complete guides and examples

### Installation

#### From Source
```bash
git clone https://github.com/Casear/claude-forge.git
cd claude-forge
cargo build --release
sudo cp target/release/claude-forge /usr/local/bin/
```

#### From GitHub Releases (Coming Soon)
```bash
# macOS Apple Silicon
curl -L https://github.com/Casear/claude-forge/releases/download/0.1.0/claude-forge-aarch64-apple-darwin.tar.gz | tar xz
sudo mv claude-forge /usr/local/bin/

# macOS Intel
curl -L https://github.com/Casear/claude-forge/releases/download/0.1.0/claude-forge-x86_64-apple-darwin.tar.gz | tar xz
sudo mv claude-forge /usr/local/bin/

# Linux x86_64
curl -L https://github.com/Casear/claude-forge/releases/download/0.1.0/claude-forge-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv claude-forge /usr/local/bin/
```

#### From crates.io (Coming Soon)
```bash
cargo install claude-forge
```

### Quick Start

```bash
# Initialize a Rust project
cd my-rust-project
claude-forge init

# Or specify language
claude-forge init --lang typescript

# Check CLI tools
claude-forge tools check

# Add custom agent
claude-forge add agent my-agent

# Check for updates
claude-forge upgrade
claude-forge update
```

### What's Next

Future releases will focus on:
- Windows support
- More language templates (C++, C#, Ruby, PHP, etc.)
- Plugin system for custom templates
- Configuration profiles for teams
- IDE integration (VS Code, JetBrains)

---

[Unreleased]: https://github.com/Casear/claude-forge/compare/0.1.0...HEAD
[0.1.0]: https://github.com/Casear/claude-forge/releases/tag/0.1.0
