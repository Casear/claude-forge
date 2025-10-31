# 🔨 Claude Forge

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org)

A powerful CLI tool for managing [Claude Code](https://claude.com/claude-code) configurations and templates. Initialize projects, manage components, install modern CLI tools, and configure MCP servers with a single command.

## ✨ Features

- 🚀 **One-Command Init**: Initialize Claude Code configuration in any project
- 🔍 **Auto-Detection**: Automatically detect language and framework
- 📦 **Tool Management**: Install and manage modern CLI tools (rg, fd, bat, eza, dust)
- 🤖 **Component Management**: Add agents, commands, and hooks easily
- 🔌 **MCP Integration**: Configure Model Context Protocol servers
- 🎨 **Multiple Languages**: Support for TypeScript, JavaScript, Python, Go, Rust, Java, Elixir, Erlang
- ⚡ **Fast & Reliable**: Built with Rust for performance and reliability

## 📦 Installation

### From Source (Cargo)

```bash
cargo install --git https://github.com/Casear/claude-forge
```

### From Binary (Coming Soon)

```bash
# macOS/Linux
curl -sSL https://raw.githubusercontent.com/Casear/claude-forge/main/install.sh | bash

# Or download from releases
# https://github.com/Casear/claude-forge/releases
```

## 🚀 Quick Start

### Initialize a Project

```bash
# Interactive mode (recommended for first time)
claude-forge init

# Quick setup for TypeScript project
claude-forge init --lang typescript --yes

# Minimal setup
claude-forge init --minimal --no-tools
```

### Add Components

```bash
# Add a custom subagent
claude-forge add agent my-reviewer

# Add a slash command
claude-forge add command deploy

# Add a hook
claude-forge add hook pre-commit --event PreToolUse
```

### Manage Tools

```bash
# Check installed tools
claude-forge tools check -v

# Install all modern CLI tools
claude-forge tools install

# Update tools
claude-forge tools update
```

### Manage MCP Servers

```bash
# List available MCP servers
claude-forge mcp list

# Add an MCP server
claude-forge mcp add filesystem --args /path/to/files

# Show current MCP configuration
claude-forge mcp show
```

### Configuration Management

```bash
# Validate configuration
claude-forge config validate

# Show current configuration
claude-forge config show

# Reset to defaults
claude-forge config reset
```

## 📖 Usage Examples

### Example 1: Initialize a TypeScript React Project

```bash
cd my-react-app
claude-forge init --lang typescript
# ✓ Detected language: TypeScript
# ✓ Detected framework: React
# ✓ Configuration files generated
# ✓ Modern CLI tools installed
# 🎉 Success!
```

### Example 2: Add Custom Agent for API Testing

```bash
claude-forge add agent api-tester
# 🤖 Adding agent: api-tester
# ✓ Agent created: .claude/agents/api-tester.md
```

### Example 3: Setup Rust Project with Full Configuration

```bash
cd my-rust-project
claude-forge init --lang rust --yes
claude-forge add agent code-reviewer
claude-forge add command analyze
claude-forge tools install
# All done! Your Rust project is configured with Claude Code
```

## 🏗️ Project Structure

After running `claude-forge init`, your project will have:

```
your-project/
├── .claude/
│   ├── CLAUDE.md              # Project memory and guidelines
│   ├── config.json            # Claude Code configuration
│   ├── agents/                # Custom subagents
│   │   ├── code-reviewer.md
│   │   └── security-scanner.md
│   ├── commands/              # Slash commands
│   │   ├── analyze.md
│   │   └── refactor.md
│   └── hooks/                 # Lifecycle hooks
│       ├── format.sh
│       └── lint.sh
└── .claudeignore              # Files to ignore
```

## 🎯 Supported Languages

| Language   | Status | Template | Modern CLI Tools |
|------------|--------|----------|------------------|
| TypeScript | ✅     | ✅       | ✅               |
| JavaScript | ✅     | ✅       | ✅               |
| Python     | ✅     | ✅       | ✅               |
| Go         | ✅     | ✅       | ✅               |
| Rust       | ✅     | ✅       | ✅               |
| Java       | ✅     | ✅       | ✅               |
| Elixir     | ✅     | ✅       | ✅               |
| Erlang     | ✅     | ✅       | ✅               |

## 🔧 Modern CLI Tools

Claude Forge helps you install and configure these powerful CLI tools:

- **ripgrep (rg)**: Lightning-fast text search
- **fd**: Fast and user-friendly find alternative
- **bat**: Cat with syntax highlighting
- **eza**: Modern ls replacement with icons
- **dust**: Intuitive disk usage analyzer

## 🔌 MCP Servers

Recommended MCP servers for different use cases:

- **Essential**: filesystem, git, github
- **Database**: postgres, sqlite, mongodb
- **Development**: docker, kubernetes, sentry
- **API**: fetch, google-maps, slack
- **Productivity**: brave-search, memory, todoist

## 🛠️ Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build from Source

```bash
git clone https://github.com/Casear/claude-forge.git
cd claude-forge
cargo build --release

# Binary will be at target/release/claude-forge
```

### Run Tests

```bash
cargo test
```

### Development Mode

```bash
cargo run -- init --help
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Claude Code](https://claude.com/claude-code) by Anthropic
- Inspired by the amazing Rust CLI ecosystem
- All contributors and users of this tool

## 📚 Documentation

- [Complete Planning Document](docs/RUST_CLI_PLAN.md)
- [Claude Code Documentation](https://docs.claude.com/claude-code)
- [MCP Documentation](https://modelcontextprotocol.io)

## 🔗 Links

- [GitHub Repository](https://github.com/Casear/claude-forge)
- [Issue Tracker](https://github.com/Casear/claude-forge/issues)
- [Changelog](CHANGELOG.md)

---

**Made with ❤️ and 🦀 Rust**
