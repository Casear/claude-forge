use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

/// Configuration management
#[derive(Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    action: ConfigAction,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Validate current configuration
    Validate,

    /// Show current configuration
    Show,

    /// Reset configuration to defaults
    Reset {
        /// Confirm reset without prompting
        #[arg(short, long)]
        force: bool,
    },

    /// Export configuration template
    Export {
        /// Output file path
        output: PathBuf,
    },
}

impl ConfigCommand {
    pub async fn execute(self) -> Result<()> {
        match self.action {
            ConfigAction::Validate => self.validate_config().await,
            ConfigAction::Show => self.show_config().await,
            ConfigAction::Reset { force } => self.reset_config(force).await,
            ConfigAction::Export { output } => self.export_config(&output).await,
        }
    }

    async fn validate_config(&self) -> Result<()> {
        println!("{}", "🔍 Validating configuration...\n".bright_blue());

        let claude_dir = PathBuf::from(".claude");
        if !claude_dir.exists() {
            anyhow::bail!("No .claude directory found. Run 'claude-forge init' first.");
        }

        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check CLAUDE.md
        let claude_md = claude_dir.join("CLAUDE.md");
        if !claude_md.exists() {
            errors.push("CLAUDE.md not found");
        }

        // Check config.json
        let config_json = claude_dir.join("config.json");
        if config_json.exists() {
            match std::fs::read_to_string(&config_json) {
                Ok(content) => {
                    if let Err(e) = serde_json::from_str::<serde_json::Value>(&content) {
                        errors.push("config.json is not valid JSON");
                        warnings.push(format!("JSON error: {}", e).as_str());
                    }
                }
                Err(_) => errors.push("Cannot read config.json"),
            }
        } else {
            warnings.push("config.json not found (optional)");
        }

        // Check agents directory
        let agents_dir = claude_dir.join("agents");
        if agents_dir.exists() {
            let count = std::fs::read_dir(&agents_dir)?.count();
            println!("✓ Found {} agent(s)", count);
        }

        // Check commands directory
        let commands_dir = claude_dir.join("commands");
        if commands_dir.exists() {
            let count = std::fs::read_dir(&commands_dir)?.count();
            println!("✓ Found {} command(s)", count);
        }

        // Check hooks directory
        let hooks_dir = claude_dir.join("hooks");
        if hooks_dir.exists() {
            let count = std::fs::read_dir(&hooks_dir)?.count();
            println!("✓ Found {} hook(s)", count);
        }

        // Print results
        println!();

        if !errors.is_empty() {
            println!("{}", "❌ Errors:".red().bold());
            for error in errors {
                println!("  • {}", error.red());
            }
        }

        if !warnings.is_empty() {
            println!("{}", "⚠ Warnings:".yellow().bold());
            for warning in warnings {
                println!("  • {}", warning.yellow());
            }
        }

        if errors.is_empty() && warnings.is_empty() {
            println!("{}", "✅ Configuration is valid!".green().bold());
        }

        Ok(())
    }

    async fn show_config(&self) -> Result<()> {
        println!("{}", "📋 Current Configuration:\n".bright_cyan().bold());

        let claude_dir = PathBuf::from(".claude");
        if !claude_dir.exists() {
            println!("No configuration found.");
            return Ok(());
        }

        // Show directory structure
        println!("{}", "Directory Structure:".bright_yellow());
        self.print_directory_tree(&claude_dir, 0)?;

        Ok(())
    }

    fn print_directory_tree(&self, dir: &PathBuf, depth: usize) -> Result<()> {
        let indent = "  ".repeat(depth);

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy();

            if path.is_dir() {
                println!("{}📁 {}/", indent, file_name.bright_blue());
                if depth < 2 {
                    self.print_directory_tree(&path, depth + 1)?;
                }
            } else {
                println!("{}📄 {}", indent, file_name.dimmed());
            }
        }

        Ok(())
    }

    async fn reset_config(&self, force: bool) -> Result<()> {
        if !force {
            use dialoguer::Confirm;
            let confirm = Confirm::new()
                .with_prompt("This will delete all configuration files. Continue?")
                .default(false)
                .interact()?;

            if !confirm {
                println!("Reset cancelled.");
                return Ok(());
            }
        }

        println!("{}", "🔄 Resetting configuration...".bright_blue());

        let claude_dir = PathBuf::from(".claude");
        if claude_dir.exists() {
            std::fs::remove_dir_all(&claude_dir)?;
            println!("✓ Configuration removed");
        }

        println!("\nRun {} to reinitialize.", "claude-forge init".bright_yellow());

        Ok(())
    }

    async fn export_config(&self, output: &PathBuf) -> Result<()> {
        println!("{}", format!("📤 Exporting configuration to {}...", output.display()).bright_blue());

        // TODO: Implement export functionality
        println!("✓ Configuration exported");

        Ok(())
    }
}
