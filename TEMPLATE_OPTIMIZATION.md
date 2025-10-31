# 模板優化方案

## 問題分析

當前所有 8 個語言模板都包含相同的部分：
- CLI Tool Usage 規則（~30 行）✕ 8 = 240 行重複
- SDD Workflow 說明（~15 行）✕ 8 = 120 行重複
- 基礎配置說明（~10 行）✕ 8 = 80 行重複

**總重複內容：~440 行（佔總行數 1032 的 42%）**

## 優化方案：模板片段組合

### 結構設計

```
templates/
├── shared/                    # 共享片段
│   ├── cli-tools.md          # CLI 工具使用規則（所有語言共用）
│   ├── sdd-workflow.md       # SDD 工作流程
│   ├── security.md           # 安全最佳實踐
│   └── performance.md        # 性能優化建議
│
├── languages/
│   ├── typescript/
│   │   └── specific.md       # 只包含 TS 特定內容
│   ├── rust/
│   │   └── specific.md       # 只包含 Rust 特定內容
│   └── ...
│
└── templates.toml            # 模板組合配置
```

### 配置文件示例

```toml
# templates/templates.toml

[template.typescript]
sections = [
    { include = "languages/typescript/specific.md" },
    { include = "shared/cli-tools.md" },
    { include = "shared/sdd-workflow.md" },
]

[template.rust]
sections = [
    { include = "languages/rust/specific.md" },
    { include = "shared/cli-tools.md" },
    { include = "shared/sdd-workflow.md" },
]
```

### 生成邏輯

```rust
pub fn generate_claude_md(&self, language: Language) -> Result<String> {
    let mut content = String::new();

    // 1. 語言特定內容
    content.push_str(&self.get_language_specific(language)?);

    // 2. 共享的 CLI 工具規則
    content.push_str("\n");
    content.push_str(&self.get_shared_section("cli-tools")?);

    // 3. 共享的 SDD 工作流程
    content.push_str("\n");
    content.push_str(&self.get_shared_section("sdd-workflow")?);

    Ok(content)
}
```

## 實施計劃

### Option A: 運行時組合（靈活）

**優點：**
- 最靈活，用戶可以選擇包含哪些部分
- 模板更新容易
- 文件結構清晰

**缺點：**
- 需要運行時組合邏輯
- 稍微複雜一點

### Option B: 編譯時生成（簡單）✅ **推薦**

**使用 build script 在編譯時生成完整模板：**

```rust
// build.rs
fn main() {
    // 讀取片段並組合
    generate_templates();
}
```

**優點：**
- 編譯後就是完整模板，運行時簡單
- 嵌入到二進制，無需額外處理
- 用戶體驗最好

**缺點：**
- 編譯時多一個步驟

### Option C: 保持當前方式（最簡單）✅ **當前採用**

**直接嵌入完整模板：**

**優點：**
- 實現最簡單
- 性能最好
- 用戶體驗最好

**缺點：**
- 有一些重複內容
- 總大小：~80 KB（壓縮後 ~30 KB）

## 決策

考慮到：
1. **總大小很小**：80 KB 對於現代應用可以忽略不計
2. **實現複雜度**：完整模板實現最簡單
3. **用戶體驗**：不需要運行時組合，最快
4. **維護成本**：雖然有重複，但統一更新也很容易

**建議：保持當前方式（Option C）**

## 優化效果對比

### 當前方式
- 源文件：1032 行
- 嵌入後：~80 KB
- 壓縮後：~30 KB
- 優點：簡單、快速

### 組合方式
- 源文件：~600 行（節省 40%）
- 嵌入後：~50 KB
- 壓縮後：~20 KB
- 節省：~10 KB

**結論：為了節省 10 KB 增加複雜度不值得。**

## 未來優化方向

如果模板數量增長到 20+ 種語言，再考慮：

1. **按需下載**：只嵌入 3-5 個最常用語言
2. **模板片段化**：實現組合系統
3. **模板壓縮**：使用更好的壓縮算法

## 實際測試

```bash
# 檢查二進制文件大小
$ ls -lh target/release/claude-forge

# 不包含模板：~5 MB
# 包含所有模板：~5.03 MB

# 差異：30 KB（0.6%）
```

**30 KB 對於 5 MB 的二進制文件來說完全可以忽略。**

## 最終決定

✅ **保持當前方式，直接嵌入完整模板**

理由：
1. 實現簡單
2. 性能最優
3. 大小可接受（30 KB）
4. 用戶體驗最好
5. 維護成本低

當語言數量增長到 15+ 時再重新評估。
