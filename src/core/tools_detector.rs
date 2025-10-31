use std::collections::HashMap;
use anyhow::Result;

use crate::tools::checker::ToolsChecker;

/// 檢測已安裝的 CLI 工具並生成相應的配置文本
pub struct ToolsDetector {
    checker: ToolsChecker,
}

#[derive(Debug, Clone)]
pub struct InstalledTools {
    pub ripgrep: bool,
    pub fd: bool,
    pub bat: bool,
    pub eza: bool,
    pub dust: bool,
}

impl ToolsDetector {
    pub fn new() -> Self {
        Self {
            checker: ToolsChecker::new(),
        }
    }

    /// 檢測所有工具的安裝狀態
    pub fn detect(&self) -> Result<InstalledTools> {
        let results = self.checker.check_all()?;

        Ok(InstalledTools {
            ripgrep: *results.get("rg").unwrap_or(&false),
            fd: *results.get("fd").unwrap_or(&false),
            bat: *results.get("bat").unwrap_or(&false),
            eza: *results.get("eza").unwrap_or(&false),
            dust: *results.get("dust").unwrap_or(&false),
        })
    }

    /// 生成工具使用規則文本
    pub fn generate_tools_section(&self, tools: &InstalledTools) -> String {
        let mut sections = Vec::new();

        // 標題
        sections.push("## 🚫 CLI Tool Usage (When Using Bash)\n".to_string());
        sections.push("**IMPORTANT**: Claude Code has built-in tools (Grep, Glob, Read) that are already optimized.\n".to_string());
        sections.push("The rules below apply ONLY when Claude needs to use the Bash tool directly.\n".to_string());
        sections.push("\nWhen using the Bash tool for operations:\n".to_string());

        // 根據安裝狀態生成相應的規則
        if tools.eza {
            sections.push(r#"
**Directory Listing:**
- ✅ ALWAYS: `eza -la --icons --git`
- ❌ NEVER: `ls -la` or `ls`
"#.to_string());
        } else {
            sections.push(r#"
**Directory Listing:**
- ℹ️ Consider installing `eza` for better directory listing: `brew install eza`
- Current: Use `ls -la` (basic)
"#.to_string());
        }

        if tools.dust {
            sections.push(r#"
**Disk Usage:**
- ✅ ALWAYS: `dust -d 2`
- ❌ NEVER: `du -sh` or `du`
"#.to_string());
        } else {
            sections.push(r#"
**Disk Usage:**
- ℹ️ Consider installing `dust` for better disk usage: `brew install dust`
- Current: Use `du -sh` (basic)
"#.to_string());
        }

        if tools.bat {
            sections.push(r#"
**File Viewing (for display purposes):**
- ✅ ALWAYS: `bat filename`
- ❌ NEVER: `cat filename`
"#.to_string());
        } else {
            sections.push(r#"
**File Viewing:**
- ℹ️ Consider installing `bat` for syntax highlighting: `brew install bat`
- Current: Use `cat filename` (basic)
"#.to_string());
        }

        if tools.fd {
            sections.push(r#"
**File Search (if not using Glob tool):**
- ✅ ALWAYS: `fd pattern`
- ❌ NEVER: `find . -name pattern`
"#.to_string());
        } else {
            sections.push(r#"
**File Search:**
- ℹ️ Consider installing `fd` for faster file search: `brew install fd`
- Current: Use `find . -name pattern` (basic)
"#.to_string());
        }

        if tools.ripgrep {
            sections.push(r#"
**Text Search (if not using Grep tool):**
- ✅ ALWAYS: `rg pattern`
- ❌ NEVER: `grep pattern`
"#.to_string());
        } else {
            sections.push(r#"
**Text Search:**
- ℹ️ Consider installing `ripgrep` for faster text search: `brew install ripgrep`
- Current: Use `grep pattern` (basic)
"#.to_string());
        }

        // 添加安裝建議
        let missing_tools = self.get_missing_tools(tools);
        if !missing_tools.is_empty() {
            sections.push("\n### 📦 Recommended Tools to Install\n".to_string());
            sections.push("\nRun this command to install missing tools:\n```bash\n".to_string());
            sections.push(format!("claude-forge tools install\n"));
            sections.push("```\n".to_string());

            sections.push("\nOr install individually:\n```bash\n".to_string());
            for tool in &missing_tools {
                sections.push(format!("# {}\n", tool.description));
                sections.push(format!("{}\n", tool.install_cmd));
            }
            sections.push("```\n".to_string());
        }

        sections.join("")
    }

    /// 生成簡潔的工具狀態摘要
    pub fn generate_tools_status(&self, tools: &InstalledTools) -> String {
        let installed_count = self.count_installed(tools);
        let total = 5;

        if installed_count == total {
            format!("✅ All modern CLI tools installed ({}/{})", installed_count, total)
        } else if installed_count == 0 {
            format!("⚠️ No modern CLI tools installed (0/{}). Run `claude-forge tools install`", total)
        } else {
            format!("ℹ️ {}/{} modern CLI tools installed. Run `claude-forge tools check` to see details", installed_count, total)
        }
    }

    fn count_installed(&self, tools: &InstalledTools) -> usize {
        let mut count = 0;
        if tools.ripgrep { count += 1; }
        if tools.fd { count += 1; }
        if tools.bat { count += 1; }
        if tools.eza { count += 1; }
        if tools.dust { count += 1; }
        count
    }

    fn get_missing_tools(&self, tools: &InstalledTools) -> Vec<MissingTool> {
        let mut missing = Vec::new();

        if !tools.ripgrep {
            missing.push(MissingTool {
                name: "ripgrep",
                description: "Fast text search",
                install_cmd: "brew install ripgrep  # or: cargo install ripgrep",
            });
        }
        if !tools.fd {
            missing.push(MissingTool {
                name: "fd",
                description: "Fast file finder",
                install_cmd: "brew install fd  # or: cargo install fd-find",
            });
        }
        if !tools.bat {
            missing.push(MissingTool {
                name: "bat",
                description: "Cat with syntax highlighting",
                install_cmd: "brew install bat  # or: cargo install bat",
            });
        }
        if !tools.eza {
            missing.push(MissingTool {
                name: "eza",
                description: "Modern ls replacement",
                install_cmd: "brew install eza  # or: cargo install eza",
            });
        }
        if !tools.dust {
            missing.push(MissingTool {
                name: "dust",
                description: "Disk usage analyzer",
                install_cmd: "brew install dust  # or: cargo install du-dust",
            });
        }

        missing
    }
}

struct MissingTool {
    name: &'static str,
    description: &'static str,
    install_cmd: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tools_installed() {
        let detector = ToolsDetector::new();
        let tools = InstalledTools {
            ripgrep: true,
            fd: true,
            bat: true,
            eza: true,
            dust: true,
        };

        let section = detector.generate_tools_section(&tools);
        assert!(section.contains("ALWAYS: `eza"));
        assert!(section.contains("ALWAYS: `rg"));
        assert!(!section.contains("Consider installing"));
    }

    #[test]
    fn test_no_tools_installed() {
        let detector = ToolsDetector::new();
        let tools = InstalledTools {
            ripgrep: false,
            fd: false,
            bat: false,
            eza: false,
            dust: false,
        };

        let section = detector.generate_tools_section(&tools);
        assert!(section.contains("Consider installing"));
        assert!(section.contains("brew install"));
        assert!(!section.contains("ALWAYS:"));
    }

    #[test]
    fn test_tools_status() {
        let detector = ToolsDetector::new();

        let all_installed = InstalledTools {
            ripgrep: true,
            fd: true,
            bat: true,
            eza: true,
            dust: true,
        };
        assert!(detector.generate_tools_status(&all_installed).contains("5/5"));

        let none_installed = InstalledTools {
            ripgrep: false,
            fd: false,
            bat: false,
            eza: false,
            dust: false,
        };
        assert!(detector.generate_tools_status(&none_installed).contains("0/5"));
    }
}
