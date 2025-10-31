use anyhow::{Context, Result};
use std::path::Path;

use super::Language;
use crate::templates::registry::TemplateRegistry;
use crate::utils::fs::write_file;

pub struct ConfigGenerator {
    language: Language,
    minimal: bool,
    registry: TemplateRegistry,
}

impl ConfigGenerator {
    pub fn new(language: Language, minimal: bool) -> Self {
        Self {
            language,
            minimal,
            registry: TemplateRegistry::new(),
        }
    }

    pub async fn generate(&self, target_path: &Path) -> Result<()> {
        let claude_dir = target_path.join(".claude");

        // Generate CLAUDE.md
        self.generate_claude_md(&claude_dir)?;

        // Generate config.json (if not minimal)
        if !self.minimal {
            self.generate_config_json(&claude_dir)?;
        }

        // Generate .claudeignore
        self.generate_claudeignore(&claude_dir)?;

        // Copy default agents (if not minimal)
        if !self.minimal {
            self.copy_default_agents(&claude_dir)?;
        }

        // Copy default commands (if not minimal)
        if !self.minimal {
            self.copy_default_commands(&claude_dir)?;
        }

        Ok(())
    }

    fn generate_claude_md(&self, claude_dir: &Path) -> Result<()> {
        let content = self.registry.get_language_claude_md(self.language)?;
        let file_path = claude_dir.join("CLAUDE.md");
        write_file(&file_path, &content)?;
        Ok(())
    }

    fn generate_config_json(&self, claude_dir: &Path) -> Result<()> {
        let config = serde_json::json!({
            "version": "1.0",
            "language": self.language.to_string().to_lowercase(),
            "features": {
                "sdd_workflow": true,
                "modern_cli_tools": true
            },
            "hooks": {
                "PreToolUse": {
                    "Read": ".claude/hooks/security-check.sh",
                },
                "SessionStart": ".claude/hooks/session-init.sh"
            }
        });

        let content = serde_json::to_string_pretty(&config)?;
        let file_path = claude_dir.join("config.json");
        write_file(&file_path, &content)?;

        Ok(())
    }

    fn generate_claudeignore(&self, claude_dir: &Path) -> Result<()> {
        let content = self.registry.get_claudeignore_template()?;
        let file_path = claude_dir.parent().unwrap().join(".claudeignore");
        write_file(&file_path, &content)?;
        Ok(())
    }

    fn copy_default_agents(&self, claude_dir: &Path) -> Result<()> {
        let agents = vec!["code-reviewer", "security-scanner"];

        for agent in agents {
            let content = self.registry.get_default_agent_template(agent)?;
            let file_path = claude_dir.join("agents").join(format!("{}.md", agent));
            write_file(&file_path, &content)
                .context(format!("Failed to create agent: {}", agent))?;
        }

        Ok(())
    }

    fn copy_default_commands(&self, claude_dir: &Path) -> Result<()> {
        let commands = vec!["analyze", "refactor"];

        for command in commands {
            let content = self.registry.get_default_command_template(command)?;
            let file_path = claude_dir.join("commands").join(format!("{}.md", command));
            write_file(&file_path, &content)
                .context(format!("Failed to create command: {}", command))?;
        }

        Ok(())
    }
}
