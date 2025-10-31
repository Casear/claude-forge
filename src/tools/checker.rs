use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;

pub struct ToolsChecker;

impl ToolsChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check_all(&self) -> Result<HashMap<String, bool>> {
        let tools = vec!["rg", "fd", "bat", "eza", "dust"];
        let mut results = HashMap::new();

        for tool in tools {
            results.insert(tool.to_string(), self.is_installed(tool));
        }

        Ok(results)
    }

    pub fn is_installed(&self, tool: &str) -> bool {
        which::which(tool).is_ok()
    }

    pub fn get_version(&self, tool: &str) -> Result<Option<String>> {
        if !self.is_installed(tool) {
            return Ok(None);
        }

        let output = Command::new(tool)
            .arg("--version")
            .output()?;

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.lines().next().unwrap_or("unknown").to_string();
            Ok(Some(version))
        } else {
            Ok(None)
        }
    }
}
