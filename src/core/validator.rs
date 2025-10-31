use anyhow::Result;
use std::path::Path;

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, claude_dir: &Path) -> Result<Vec<String>> {
        let mut errors = Vec::new();

        // Check if .claude directory exists
        if !claude_dir.exists() {
            errors.push(".claude directory not found".to_string());
            return Ok(errors);
        }

        // Check CLAUDE.md
        let claude_md = claude_dir.join("CLAUDE.md");
        if !claude_md.exists() {
            errors.push("CLAUDE.md not found".to_string());
        }

        // Check config.json (optional, but validate if exists)
        let config_json = claude_dir.join("config.json");
        if config_json.exists() {
            if let Err(e) = self.validate_config_json(&config_json) {
                errors.push(format!("config.json validation failed: {}", e));
            }
        }

        // Check directory structure
        for dir in &["agents", "commands", "hooks"] {
            let dir_path = claude_dir.join(dir);
            if !dir_path.exists() {
                errors.push(format!("{} directory not found", dir));
            }
        }

        Ok(errors)
    }

    fn validate_config_json(&self, config_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(config_path)?;
        serde_json::from_str::<serde_json::Value>(&content)?;
        Ok(())
    }
}
