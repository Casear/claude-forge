use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

const GITHUB_REPO: &str = "Casear/claude-forge";
const GITHUB_API_URL: &str = "https://api.github.com/repos";
const TEMPLATES_ARCHIVE: &str = "templates.tar.gz";

/// Update templates to the latest version
#[derive(Args)]
pub struct UpdateCommand {
    /// Show what would be done without actually updating
    #[arg(long)]
    dry_run: bool,

    /// Update to a specific version
    #[arg(short, long)]
    version: Option<String>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    yes: bool,

    /// Force update even if already on latest version
    #[arg(short, long)]
    force: bool,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    name: String,
    body: Option<String>,
}

impl UpdateCommand {
    pub async fn execute(self) -> Result<()> {
        println!("{}", "🔄 Checking for template updates...\n".bright_blue());

        // Get current template version
        let current_version = self.get_current_template_version()?;
        if let Some(ref ver) = current_version {
            println!("Current template version: {}", ver.bright_yellow());
        } else {
            println!("Current template version: {}", "unknown".dimmed());
        }

        // Get latest release info
        let latest = self.get_latest_release().await?;
        let target_version = self.version.as_deref().unwrap_or(&latest.tag_name);

        println!("Latest version: {}", target_version.bright_yellow());

        // Check if update is needed
        if !self.force {
            if let Some(ref current) = current_version {
                if current == target_version {
                    println!("\n{}", "✓ Templates are already up to date!".green());
                    return Ok(());
                }
            }
        }

        println!(
            "\n{}",
            format!(
                "Update available: {} → {}",
                current_version.as_deref().unwrap_or("unknown"),
                target_version
            )
            .bright_cyan()
        );

        if let Some(body) = &latest.body {
            println!("\n{}", "Release Notes:".bright_cyan().bold());
            println!("{}", body.lines().take(10).collect::<Vec<_>>().join("\n"));
            if body.lines().count() > 10 {
                println!("... (view full release notes at GitHub)");
            }
        }

        // Confirm update
        if !self.yes && !self.dry_run {
            use dialoguer::Confirm;
            let confirm = Confirm::new()
                .with_prompt("Do you want to update templates?")
                .default(true)
                .interact()?;

            if !confirm {
                println!("Update cancelled.");
                return Ok(());
            }
        }

        if self.dry_run {
            println!("\n{}", "Dry run mode - no changes made".yellow());
            println!("Would download and extract templates version: {}", target_version);
            return Ok(());
        }

        // Perform update
        self.perform_update(target_version).await?;

        println!("\n{}", "✓ Templates updated successfully!".green().bold());
        println!(
            "\nRun {} to initialize projects with the latest templates.",
            "claude-forge init".bright_yellow()
        );

        Ok(())
    }

    fn get_current_template_version(&self) -> Result<Option<String>> {
        let version_file = self.get_templates_dir()?.join("VERSION");

        if version_file.exists() {
            let version = std::fs::read_to_string(&version_file)
                .context("Failed to read template version")?;
            Ok(Some(version.trim().to_string()))
        } else {
            // Fallback to binary version for first-time users
            Ok(Some(env!("CARGO_PKG_VERSION").to_string()))
        }
    }

    async fn get_latest_release(&self) -> Result<Release> {
        let url = format!("{}/{}/releases/latest", GITHUB_API_URL, GITHUB_REPO);

        let client = reqwest::Client::builder()
            .user_agent("claude-forge")
            .build()?;

        let response = client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch latest release")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch release info: HTTP {}", response.status());
        }

        let release: Release = response
            .json()
            .await
            .context("Failed to parse release info")?;

        Ok(release)
    }

    async fn perform_update(&self, version: &str) -> Result<()> {
        println!("\n{}", "📥 Downloading latest templates...".bright_blue());

        let download_url = format!(
            "https://github.com/{}/archive/refs/tags/{}.tar.gz",
            GITHUB_REPO, version
        );

        println!("Downloading from: {}", download_url.dimmed());

        // Download the archive
        let client = reqwest::Client::builder()
            .user_agent("claude-forge")
            .build()?;

        let response = client
            .get(&download_url)
            .send()
            .await
            .context("Failed to download templates")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download: HTTP {}", response.status());
        }

        let bytes = response.bytes().await?;

        // Save to temp file
        let temp_dir = std::env::temp_dir();
        let tar_path = temp_dir.join(format!("claude-forge-templates-{}.tar.gz", version));
        std::fs::write(&tar_path, bytes)?;

        println!("✓ Downloaded to: {}", tar_path.display().to_string().dimmed());

        // Extract
        println!("\n{}", "📦 Extracting templates...".bright_blue());
        let extract_dir = temp_dir.join("claude-forge-templates-extract");

        // Remove old extract dir if exists
        if extract_dir.exists() {
            std::fs::remove_dir_all(&extract_dir)?;
        }
        std::fs::create_dir_all(&extract_dir)?;

        let status = Command::new("tar")
            .args(&[
                "xzf",
                tar_path.to_str().unwrap(),
                "-C",
                extract_dir.to_str().unwrap(),
                "--strip-components=1",
            ])
            .status()
            .context("Failed to extract archive")?;

        if !status.success() {
            anyhow::bail!("Failed to extract archive");
        }

        // Install templates
        println!("\n{}", "🔧 Installing templates...".bright_blue());

        let templates_src = extract_dir.join("templates");
        let templates_dest = self.get_templates_dir()?;

        // Ensure destination exists
        std::fs::create_dir_all(&templates_dest)?;

        // Copy templates
        self.copy_dir_recursive(&templates_src, &templates_dest)?;

        // Write version file
        let version_file = templates_dest.join("VERSION");
        std::fs::write(&version_file, version)?;

        println!("✓ Installed to: {}", templates_dest.display().to_string().bright_yellow());

        // Show what was updated
        self.show_updated_templates(&templates_dest)?;

        // Cleanup
        let _ = std::fs::remove_file(&tar_path);
        let _ = std::fs::remove_dir_all(&extract_dir);

        Ok(())
    }

    fn get_templates_dir(&self) -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?;

        Ok(config_dir.join("claude-forge").join("templates"))
    }

    fn copy_dir_recursive(&self, src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
        if !dst.exists() {
            std::fs::create_dir_all(dst)?;
        }

        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if ty.is_dir() {
                self.copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    fn show_updated_templates(&self, templates_dir: &std::path::Path) -> Result<()> {
        println!("\n{}", "📋 Updated templates:".bright_cyan());

        // Count templates
        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        if let Ok(entries) = std::fs::read_dir(templates_dir.join("languages")) {
            for entry in entries.flatten() {
                if entry.file_type()?.is_dir() {
                    *counts.entry("languages".to_string()).or_insert(0) += 1;
                }
            }
        }

        if let Ok(entries) = std::fs::read_dir(templates_dir.join("core")) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if entry.file_type()?.is_dir() {
                    *counts.entry(name_str.to_string()).or_insert(0) += 1;
                }
            }
        }

        for (category, _count) in counts {
            println!("  {} {} templates", "✓".green(), category);
        }

        Ok(())
    }
}
