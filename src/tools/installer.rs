use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

use super::checker::ToolsChecker;

pub struct ToolsInstaller {
    checker: ToolsChecker,
}

impl ToolsInstaller {
    pub fn new() -> Self {
        Self {
            checker: ToolsChecker::new(),
        }
    }

    pub async fn check_and_install(&self) -> Result<()> {
        let tools = vec![
            ("rg", "ripgrep"),
            ("fd", "fd-find"),
            ("bat", "bat"),
            ("eza", "eza"),
            ("dust", "dust"),
        ];

        for (binary, package) in tools {
            if self.checker.is_installed(binary) {
                println!("✓ {} is already installed", binary.green());
            } else {
                println!("⚠ {} not found, installing...", binary.yellow());
                self.install_tool(package).await?;
            }
        }

        Ok(())
    }

    pub async fn install_all_with_options(&self, skip: &[String], dry_run: bool) -> Result<()> {
        let tools = vec![
            ("rg", "ripgrep", "Fast text search"),
            ("fd", "fd-find", "Fast file finder"),
            ("bat", "bat", "Cat with syntax highlighting"),
            ("eza", "eza", "Modern ls replacement"),
            ("dust", "dust", "Disk usage analyzer"),
        ];

        for (binary, package, description) in tools {
            if skip.contains(&binary.to_string()) {
                println!("⊘ Skipping {}", binary.dimmed());
                continue;
            }

            print!("  {} - {}: ", binary.bright_yellow(), description.dimmed());

            if self.checker.is_installed(binary) {
                println!("{}", "already installed".green());
                continue;
            }

            if dry_run {
                println!("{}", "would install".blue());
                continue;
            }

            match self.install_tool(package).await {
                Ok(_) => println!("{}", "installed".green()),
                Err(e) => println!("{}: {}", "failed".red(), e),
            }
        }

        Ok(())
    }

    pub async fn update_all_with_options(&self, skip: &[String]) -> Result<()> {
        let platform = self.detect_platform();

        match platform.as_str() {
            "macos" => self.update_with_brew(skip).await,
            "linux" => self.update_with_apt_or_cargo(skip).await,
            _ => {
                println!("⚠ Platform not supported for automatic updates");
                println!("Please update tools manually");
                Ok(())
            }
        }
    }

    async fn install_tool(&self, package: &str) -> Result<()> {
        let platform = self.detect_platform();

        match platform.as_str() {
            "macos" => self.install_with_brew(package).await,
            "linux" => self.install_with_apt_or_cargo(package).await,
            _ => self.install_with_cargo(package).await,
        }
    }

    async fn install_with_brew(&self, package: &str) -> Result<()> {
        let output = Command::new("brew")
            .args(&["install", package])
            .output()
            .context("Failed to run brew install")?;

        if !output.status.success() {
            anyhow::bail!(
                "brew install failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    async fn install_with_apt_or_cargo(&self, package: &str) -> Result<()> {
        // Try apt first
        if which::which("apt").is_ok() {
            match Command::new("sudo")
                .args(&["apt", "install", "-y", package])
                .output()
            {
                Ok(output) if output.status.success() => return Ok(()),
                _ => {}
            }
        }

        // Fallback to cargo
        self.install_with_cargo(package).await
    }

    async fn install_with_cargo(&self, package: &str) -> Result<()> {
        let output = Command::new("cargo")
            .args(&["install", package])
            .output()
            .context("Failed to run cargo install")?;

        if !output.status.success() {
            anyhow::bail!(
                "cargo install failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    async fn update_with_brew(&self, skip: &[String]) -> Result<()> {
        let tools = vec!["ripgrep", "fd", "bat", "eza", "dust"];

        for tool in tools {
            if skip.contains(&tool.to_string()) {
                continue;
            }

            println!("Updating {}...", tool.bright_yellow());
            let _ = Command::new("brew")
                .args(&["upgrade", tool])
                .output();
        }

        Ok(())
    }

    async fn update_with_apt_or_cargo(&self, _skip: &[String]) -> Result<()> {
        println!("Update via apt/cargo coming soon...");
        Ok(())
    }

    fn detect_platform(&self) -> String {
        if cfg!(target_os = "macos") {
            "macos".to_string()
        } else if cfg!(target_os = "linux") {
            "linux".to_string()
        } else if cfg!(target_os = "windows") {
            "windows".to_string()
        } else {
            "unknown".to_string()
        }
    }
}
