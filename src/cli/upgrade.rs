use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use serde::Deserialize;
use std::process::Command;

const GITHUB_REPO: &str = "Casear/claude-forge";
const GITHUB_API_URL: &str = "https://api.github.com/repos";

/// Upgrade claude-forge to the latest version
#[derive(Args)]
pub struct UpgradeCommand {
    /// Show what would be done without actually upgrading
    #[arg(long)]
    dry_run: bool,

    /// Upgrade to a specific version
    #[arg(short, long)]
    version: Option<String>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    yes: bool,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    name: String,
    body: Option<String>,
    prerelease: bool,
    published_at: String,
}

impl UpgradeCommand {
    pub async fn execute(self) -> Result<()> {
        println!("{}", "🔄 Checking for updates...\n".bright_blue());

        // Get current version
        let current_version = env!("CARGO_PKG_VERSION");
        println!("Current version: {}", current_version.bright_yellow());

        // Get latest release info
        let latest = self.get_latest_release().await?;

        if let Some(target_version) = &self.version {
            println!("Target version: {}", target_version.bright_yellow());
        } else {
            println!("Latest version: {}", latest.tag_name.bright_yellow());
        }

        // Compare versions
        let target = self.version.as_deref().unwrap_or(&latest.tag_name);
        if current_version == target {
            println!("\n{}", "✓ You are already on the latest version!".green());
            return Ok(());
        }

        println!("\n{}", format!("New version available: {} → {}", current_version, target).bright_cyan());

        if let Some(body) = &latest.body {
            println!("\n{}", "Release Notes:".bright_cyan().bold());
            println!("{}", body.lines().take(10).collect::<Vec<_>>().join("\n"));
            if body.lines().count() > 10 {
                println!("... (view full release notes at GitHub)");
            }
        }

        // Confirm upgrade
        if !self.yes && !self.dry_run {
            use dialoguer::Confirm;
            let confirm = Confirm::new()
                .with_prompt("Do you want to upgrade?")
                .default(true)
                .interact()?;

            if !confirm {
                println!("Upgrade cancelled.");
                return Ok(());
            }
        }

        if self.dry_run {
            println!("\n{}", "Dry run mode - no changes made".yellow());
            println!("Would download and install version: {}", target);
            return Ok(());
        }

        // Perform upgrade
        self.perform_upgrade(target).await?;

        println!("\n{}", "✓ Upgrade completed successfully!".green().bold());
        println!("\nRun {} to verify the new version.", "claude-forge --version".bright_yellow());

        Ok(())
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

    async fn perform_upgrade(&self, version: &str) -> Result<()> {
        println!("\n{}", "📥 Downloading new version...".bright_blue());

        // Detect platform
        let target = self.detect_platform_target()?;
        println!("Platform: {}", target.bright_yellow());

        // Download URL
        let download_url = format!(
            "https://github.com/{}/releases/download/{}/claude-forge-{}.tar.gz",
            GITHUB_REPO, version, target
        );

        println!("Downloading from: {}", download_url.dimmed());

        // Download the file
        let client = reqwest::Client::builder()
            .user_agent("claude-forge")
            .build()?;

        let response = client
            .get(&download_url)
            .send()
            .await
            .context("Failed to download release")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download: HTTP {}", response.status());
        }

        let bytes = response.bytes().await?;

        // Save to temp file
        let temp_dir = std::env::temp_dir();
        let tar_path = temp_dir.join(format!("claude-forge-{}.tar.gz", version));
        std::fs::write(&tar_path, bytes)?;

        println!("✓ Downloaded to: {}", tar_path.display().to_string().dimmed());

        // Extract
        println!("\n{}", "📦 Extracting...".bright_blue());
        let extract_dir = temp_dir.join("claude-forge-extract");
        std::fs::create_dir_all(&extract_dir)?;

        let status = Command::new("tar")
            .args(&["xzf", tar_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
            .status()
            .context("Failed to extract archive")?;

        if !status.success() {
            anyhow::bail!("Failed to extract archive");
        }

        // Install
        println!("\n{}", "🔧 Installing...".bright_blue());

        let binary_path = extract_dir.join("claude-forge");
        let install_path = self.get_install_path()?;

        // Check if we need sudo
        let needs_sudo = self.needs_sudo(&install_path)?;

        if needs_sudo {
            println!("Installing to {} (requires sudo)", install_path.display().to_string().bright_yellow());

            let status = Command::new("sudo")
                .args(&["mv", binary_path.to_str().unwrap(), install_path.to_str().unwrap()])
                .status()?;

            if !status.success() {
                anyhow::bail!("Failed to install binary");
            }

            // Set permissions
            let _ = Command::new("sudo")
                .args(&["chmod", "+x", install_path.to_str().unwrap()])
                .status();
        } else {
            std::fs::copy(&binary_path, &install_path)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&install_path)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&install_path, perms)?;
            }
        }

        println!("✓ Installed to: {}", install_path.display().to_string().bright_yellow());

        // Cleanup
        let _ = std::fs::remove_file(&tar_path);
        let _ = std::fs::remove_dir_all(&extract_dir);

        Ok(())
    }

    fn detect_platform_target(&self) -> Result<String> {
        let target = if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") {
                "aarch64-apple-darwin"
            } else {
                "x86_64-apple-darwin"
            }
        } else if cfg!(target_os = "linux") {
            if cfg!(target_arch = "aarch64") {
                "aarch64-unknown-linux-gnu"
            } else {
                "x86_64-unknown-linux-gnu"
            }
        } else {
            anyhow::bail!("Unsupported platform");
        };

        Ok(target.to_string())
    }

    fn get_install_path(&self) -> Result<std::path::PathBuf> {
        // Try to find where claude-forge is currently installed
        if let Ok(current_path) = which::which("claude-forge") {
            return Ok(current_path);
        }

        // Default installation paths
        let default_paths = [
            "/usr/local/bin/claude-forge",
            "/usr/bin/claude-forge",
        ];

        for path in &default_paths {
            let p = std::path::PathBuf::from(path);
            if p.parent().map(|p| p.exists()).unwrap_or(false) {
                return Ok(p);
            }
        }

        Ok(std::path::PathBuf::from("/usr/local/bin/claude-forge"))
    }

    fn needs_sudo(&self, path: &std::path::Path) -> Result<bool> {
        // Check if the parent directory is writable
        if let Some(parent) = path.parent() {
            match std::fs::metadata(parent) {
                Ok(_) => {
                    // Try to create a test file
                    let test_file = parent.join(".claude-forge-test");
                    match std::fs::write(&test_file, "") {
                        Ok(_) => {
                            let _ = std::fs::remove_file(&test_file);
                            Ok(false) // No sudo needed
                        }
                        Err(_) => Ok(true), // Sudo needed
                    }
                }
                Err(_) => Ok(true), // Sudo needed
            }
        } else {
            Ok(true)
        }
    }
}
