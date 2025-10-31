# 🚀 Claude Forge - 快速開始

## 安裝

### 從源碼安裝（當前）

```bash
# 克隆 repository
git clone git@github.com:Casear/claude-forge.git
cd claude-forge

# 編譯 release 版本
cargo build --release

# 複製到 PATH（可選）
sudo cp target/release/claude-forge /usr/local/bin/

# 或創建 alias
echo 'alias claude-forge="/path/to/claude-forge/target/release/claude-forge"' >> ~/.zshrc
source ~/.zshrc
```

### 從 crates.io 安裝（未來）

```bash
cargo install claude-forge
```

## 5 分鐘快速開始

### 1. 初始化一個 Rust 專案

```bash
# 進入你的 Rust 專案
cd my-rust-project

# 初始化 Claude Code 配置
claude-forge init

# 或指定語言
claude-forge init --lang rust
```

輸出：
```
🔨 Claude Forge
   Manage Claude Code configurations

🚀 Initializing Claude Code configuration...

✓ Detected language: Rust
ℹ️ 3/5 modern CLI tools installed
? Install modern CLI tools (rg, fd, bat, eza, dust)? (Y/n)
```

### 2. 查看生成的文件

```bash
ls -la .claude/
```

你會看到：
```
.claude/
├── CLAUDE.md              # 專案記憶和配置
├── config.json            # Claude Code 設定
├── agents/                # 自定義 subagents
│   ├── code-reviewer.md
│   └── security-scanner.md
├── commands/              # Slash commands
│   ├── analyze.md
│   └── refactor.md
└── hooks/                 # Lifecycle hooks
```

### 3. 檢查 CLI 工具狀態

```bash
claude-forge tools check -v
```

輸出：
```
🔍 Checking modern CLI tools...

✓ rg (ripgrep 14.1.0)
✓ fd (fd 9.0.0)
✓ bat (bat 0.24.0)
✗ eza
✗ dust

ℹ️ 3/5 modern CLI tools installed. Run `claude-forge tools check` to see details
```

### 4. 安裝缺失的工具

```bash
claude-forge tools install
```

### 5. 添加自定義組件

```bash
# 添加一個專門檢查 API 的 agent
claude-forge add agent api-tester

# 添加一個部署命令
claude-forge add command deploy

# 添加一個 pre-commit hook
claude-forge add hook pre-commit
```

## 常見使用場景

### 場景 1: TypeScript/React 專案

```bash
cd my-react-app
claude-forge init --lang typescript

# 生成的配置會包含:
# - TypeScript 最佳實踐
# - React 特定建議
# - ESLint/Prettier 配置
# - 智能的 CLI 工具建議
```

### 場景 2: Python 專案

```bash
cd my-python-project
claude-forge init --lang python

# 添加專門的格式化命令
claude-forge add command format
```

在 `.claude/commands/format.md`:
```markdown
---
description: Format Python code with black and ruff
allowed-tools: Bash
---

!black . && ruff check --fix .
```

### 場景 3: 多語言專案

```bash
# 主要是 TypeScript
claude-forge init --lang typescript

# 但也有 Python 腳本
# 手動編輯 .claude/CLAUDE.md 添加 Python 規則
```

### 場景 4: 驗證配置

```bash
# 檢查配置是否正確
claude-forge config validate

# 顯示配置結構
claude-forge config show

# 如果出錯，重置配置
claude-forge config reset
```

## 高級用法

### 使用最小模板

```bash
# 只生成必要的文件
claude-forge init --minimal
```

### 非交互式模式

```bash
# 接受所有默認值
claude-forge init --yes
```

### 跳過工具安裝

```bash
# 不安裝 CLI 工具
claude-forge init --no-tools
```

### 跳過 MCP 配置

```bash
# 不配置 MCP
claude-forge init --no-mcp
```

### 組合使用

```bash
# 快速初始化，不安裝任何額外內容
claude-forge init --lang rust --yes --no-tools --no-mcp --minimal
```

## MCP 服務器管理

### 列出可用的 MCP 服務器

```bash
claude-forge mcp list
```

### 添加常用的 MCP 服務器

```bash
# 文件系統訪問
claude-forge mcp add filesystem --args /path/to/project

# Git 操作
claude-forge mcp add git --args .

# GitHub 集成
claude-forge mcp add github
```

### 查看當前 MCP 配置

```bash
claude-forge mcp show
```

## 工具管理

### 只安裝特定工具

```bash
# 只安裝 ripgrep 和 fd
claude-forge tools install --skip bat,eza,dust
```

### 更新工具

```bash
# 更新所有工具到最新版本
claude-forge tools update

# 跳過某些工具
claude-forge tools update --skip eza
```

### 查看詳細版本信息

```bash
claude-forge tools check --verbose
```

## 故障排除

### 問題 1: 找不到 .claude 目錄

```bash
# 確保在專案根目錄
pwd

# 重新初始化
claude-forge init
```

### 問題 2: 工具安裝失敗

```bash
# macOS: 確保安裝了 Homebrew
brew --version

# 手動安裝
brew install ripgrep fd bat eza dust

# 或使用 cargo
cargo install ripgrep fd-find bat eza du-dust
```

### 問題 3: 權限錯誤

```bash
# Hooks 沒有執行權限
chmod +x .claude/hooks/*.sh

# 或讓 claude-forge 重新生成
claude-forge add hook my-hook
```

### 問題 4: 配置驗證失敗

```bash
# 查看詳細錯誤
claude-forge config validate

# 重置配置
claude-forge config reset

# 重新初始化
claude-forge init
```

## 實用技巧

### 技巧 1: 快速檢查狀態

創建 alias:
```bash
echo 'alias ccf="claude-forge"' >> ~/.zshrc
source ~/.zshrc

# 現在可以用
ccf tools check
ccf config validate
```

### 技巧 2: 團隊共享配置

```bash
# 將 .claude/ 目錄加入 git
git add .claude/
git commit -m "Add Claude Code configuration"

# 團隊成員克隆後立即可用
git clone ...
cd project
# .claude/ 已經在這裡了！
```

### 技巧 3: 項目模板

```bash
# 創建一個標準模板專案
mkdir my-template
cd my-template
claude-forge init --lang typescript
git init
git add .
git commit -m "Initial commit with Claude Code config"

# 以後創建新專案時
cp -r my-template my-new-project
```

### 技巧 4: 批量初始化

```bash
# 為多個專案初始化
for dir in project1 project2 project3; do
  cd $dir
  claude-forge init --yes --lang rust
  cd ..
done
```

## 下一步

1. **閱讀生成的 CLAUDE.md**
   - 了解專案特定的配置
   - 自定義規則和建議

2. **探索 agents 和 commands**
   - 嘗試使用 `/analyze` 命令
   - 讓 `code-reviewer` agent 審查代碼

3. **配置 hooks**
   - 設置自動格式化
   - 添加安全檢查

4. **集成到工作流程**
   - 在 CI/CD 中使用
   - 團隊協作配置

## 獲取幫助

```bash
# 查看幫助
claude-forge --help
claude-forge init --help
claude-forge tools --help

# 查看版本
claude-forge --version

# 查看文檔
cat README.md
cat SMART_TOOLS_DETECTION.md
cat PROJECT_SUMMARY.md
```

## 資源鏈接

- **GitHub**: https://github.com/Casear/claude-forge
- **Issues**: https://github.com/Casear/claude-forge/issues
- **Claude Code 文檔**: https://docs.claude.com/claude-code

---

**享受使用 Claude Forge！** 🎉

如有問題或建議，歡迎提 Issue！
