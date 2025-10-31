# 模板策略文檔

## 設計決策：混合方案

Claude Forge 使用**混合模板策略**，結合嵌入式模板和動態更新的優點。

## 架構設計

### 1. 嵌入式模板（編譯時）

**包含內容：**
- 核心配置模板（CLAUDE.md, .claudeignore, config.json）
- 8 種語言的基礎模板
- 默認 agents (code-reviewer, security-scanner)
- 默認 commands (analyze, refactor)
- 默認 hooks (format, lint)

**實現：**
```rust
use include_dir::{include_dir, Dir};

static EMBEDDED_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
```

**優點：**
- 離線可用
- 安裝後立即可用
- 版本一致性保證

### 2. 動態模板更新（運行時）

**支持命令：**
```bash
# 更新所有模板到最新版本
claude-forge templates update

# 列出可用模板
claude-forge templates list

# 安裝特定語言模板
claude-forge templates install --lang rust

# 從自定義源安裝
claude-forge templates install --source https://github.com/user/custom-templates
```

**存儲位置：**
- macOS/Linux: `~/.config/claude-forge/templates/`
- Windows: `%APPDATA%\claude-forge\templates\`

### 3. 模板優先級

```
1. 用戶自定義模板（~/.config/claude-forge/templates/）
   ↓
2. 項目本地模板（./claude-forge-templates/）
   ↓
3. 嵌入式模板（編譯到二進制）
```

## 實現計劃

### Phase 1: 嵌入式模板（當前）✅

```rust
// src/templates/embedded.rs
use include_dir::{include_dir, Dir};

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn get_embedded_template(path: &str) -> Option<&'static str> {
    TEMPLATES
        .get_file(path)
        .and_then(|f| f.contents_utf8())
}
```

### Phase 2: 模板管理命令（未來）

```bash
claude-forge templates update      # 更新到最新版本
claude-forge templates list        # 列出所有可用模板
claude-forge templates show rust   # 顯示 Rust 模板詳情
claude-forge templates reset       # 重置為嵌入版本
```

### Phase 3: 自定義模板源（未來）

```toml
# ~/.config/claude-forge/config.toml
[templates]
sources = [
    "https://github.com/Casear/claude-forge-templates",
    "https://github.com/my-team/custom-templates"
]

auto_update = true
update_interval = "7d"
```

## 用戶體驗

### 場景 1: 新用戶首次使用

```bash
$ claude-forge init
🚀 Initializing Claude Code configuration...
✓ Detected language: Rust
✓ Using embedded template (v0.1.0)
✓ Configuration files generated
🎉 Success!
```

### 場景 2: 用戶想要最新模板

```bash
$ claude-forge templates update
📦 Checking for template updates...
✓ Found updates for 3 languages
  • TypeScript: v0.1.0 → v0.2.0
  • Rust: v0.1.0 → v0.2.0
  • Python: v0.1.0 → v0.1.1

Download updates? [Y/n] y
✓ Templates updated successfully!

$ claude-forge init
🚀 Initializing Claude Code configuration...
✓ Detected language: Rust
✓ Using updated template (v0.2.0)
```

### 場景 3: 離線使用

```bash
$ claude-forge init
🚀 Initializing Claude Code configuration...
⚠ Network unavailable, using embedded templates
✓ Detected language: Rust
✓ Using embedded template (v0.1.0)
✓ Configuration files generated
```

### 場景 4: 企業自定義模板

```bash
$ claude-forge templates add-source https://github.com/company/templates
✓ Added template source: company/templates

$ claude-forge init
🚀 Initializing Claude Code configuration...
✓ Detected language: Rust
? Which template source?
  > company/templates (v1.0.0)
    claude-forge (v0.2.0)
    embedded (v0.1.0)
```

## 模板版本管理

### 版本號格式
```
major.minor.patch
  │     │     └─ Bug fixes, typo corrections
  │     └─ New features, template additions
  └─ Breaking changes
```

### 模板元數據
```toml
# templates/metadata.toml
[template]
version = "0.1.0"
author = "Casear"
repository = "https://github.com/Casear/claude-forge"

[languages.rust]
version = "0.1.0"
last_updated = "2025-01-31"
maintainer = "Casear"

[languages.typescript]
version = "0.1.0"
last_updated = "2025-01-31"
```

## 文件結構

### 嵌入式模板結構
```
templates/
├── metadata.toml           # 模板版本信息
├── core/                   # 核心通用模板
│   ├── CLAUDE.md.template
│   ├── .claudeignore
│   ├── config.json.template
│   ├── agents/
│   │   ├── code-reviewer.md
│   │   ├── security-scanner.md
│   │   └── performance-optimizer.md
│   ├── commands/
│   │   ├── analyze.md
│   │   ├── refactor.md
│   │   └── docs.md
│   └── hooks/
│       ├── format.sh
│       ├── lint.sh
│       └── security-check.sh
└── languages/
    ├── typescript/
    │   ├── CLAUDE.md.template
    │   ├── commands/
    │   └── hooks/
    ├── javascript/
    ├── python/
    ├── go/
    ├── rust/
    ├── java/
    ├── elixir/
    └── erlang/
```

## 存儲大小估算

### 嵌入式模板
- 每個語言模板：~5-10 KB
- 8 種語言：~60 KB
- 核心模板：~20 KB
- 總計：**~80 KB**

壓縮後在二進制文件中：**~30 KB**

### 結論
嵌入模板的大小可以忽略不計，對二進制文件大小影響很小。

## 實施步驟

### 立即實施（v0.1.0）
1. ✅ 從 claude-code-setting 複製所有模板
2. ✅ 使用 include_dir 嵌入模板
3. ✅ 更新 TemplateRegistry 使用嵌入式模板

### 短期計劃（v0.2.0）
1. 添加 `templates` 命令組
2. 實現模板更新功能
3. 添加模板列表和顯示

### 長期計劃（v0.3.0+）
1. 自定義模板源支持
2. 模板市場
3. 社區模板共享

## 安全考慮

1. **模板驗證**：下載的模板需要驗證簽名
2. **沙箱執行**：Hook 腳本在沙箱中執行
3. **用戶確認**：更新模板前需要用戶確認
4. **版本鎖定**：支持鎖定特定模板版本

## 測試策略

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_templates_exist() {
        assert!(get_embedded_template("languages/rust/CLAUDE.md.template").is_some());
        assert!(get_embedded_template("core/agents/code-reviewer.md").is_some());
    }

    #[test]
    fn test_template_priority() {
        // User template > Project template > Embedded template
    }
}
```

---

**推薦方案：** 立即實施嵌入式模板，為未來的動態更新預留接口。

這樣用戶安裝後立即可用，同時保留了擴展性。
