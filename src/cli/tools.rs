use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;

use crate::tools::{checker::ToolsChecker, installer::ToolsInstaller};

/// Manage modern CLI tools (rg, fd, bat, eza, dust)
#[derive(Args)]
pub struct ToolsCommand {
    #[command(subcommand)]
    action: ToolsAction,
}

#[derive(Subcommand)]
enum ToolsAction {
    /// Install modern CLI tools
    Install {
        /// Skip specific tools (comma-separated)
        #[arg(long, value_delimiter = ',')]
        skip: Vec<String>,

        /// Show what would be installed without installing
        #[arg(long)]
        dry_run: bool,
    },

    /// Check if tools are installed
    Check {
        /// Show detailed version information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Update tools to latest version
    Update {
        /// Skip specific tools (comma-separated)
        #[arg(long, value_delimiter = ',')]
        skip: Vec<String>,
    },
}

impl ToolsCommand {
    pub async fn execute(self) -> Result<()> {
        match self.action {
            ToolsAction::Install { ref skip, dry_run } => {
                self.install_tools(skip, dry_run).await
            }
            ToolsAction::Check { verbose } => {
                self.check_tools(verbose).await
            }
            ToolsAction::Update { ref skip } => {
                self.update_tools(skip).await
            }
        }
    }

    async fn install_tools(&self, skip: &[String], dry_run: bool) -> Result<()> {
        println!("{}", "📦 Installing modern CLI tools...\n".bright_blue());

        if dry_run {
            println!("{}", "🔍 Dry run mode - no changes will be made\n".yellow());
        }

        let installer = ToolsInstaller::new();
        installer.install_all_with_options(&skip, dry_run).await?;

        if !dry_run {
            println!("\n{}", "✓ Installation complete!".green());
            self.print_next_steps();
        }

        Ok(())
    }

    async fn check_tools(&self, verbose: bool) -> Result<()> {
        println!("{}", "🔍 Checking modern CLI tools...\n".bright_blue());

        let checker = ToolsChecker::new();
        let results = checker.check_all()?;

        let mut all_installed = true;

        for (tool, installed) in &results {
            let status = if *installed {
                "✓".green().to_string()
            } else {
                "✗".red().to_string()
            };

            print!("{} {} ", status, tool.bright_yellow());

            if *installed && verbose {
                if let Some(version) = checker.get_version(tool)? {
                    print!("({})", version.dimmed());
                }
            }

            println!();

            if !installed {
                all_installed = false;
            }
        }

        if all_installed {
            println!("\n{}", "🎉 All tools are installed!".green().bold());
        } else {
            println!("\n{}", "⚠ Some tools are missing.".yellow());
            println!("Run {} to install them.", "claude-forge tools install".bright_yellow());
        }

        Ok(())
    }

    async fn update_tools(&self, skip: &[String]) -> Result<()> {
        println!("{}", "🔄 Updating modern CLI tools...\n".bright_blue());

        let installer = ToolsInstaller::new();
        installer.update_all_with_options(&skip).await?;

        println!("\n{}", "✓ Update complete!".green());

        Ok(())
    }

    fn print_next_steps(&self) {
        println!("\n{}", "📝 Next steps:".bright_cyan().bold());
        println!("  1. Run {} to verify installation", "claude-forge tools check -v".bright_yellow());
        println!("  2. Add the tools to your PATH if needed");
        println!("  3. Restart your terminal to use the new tools");

        println!("\n{}", "💡 Quick start:".bright_cyan().bold());
        println!("  • Search files: {}", "rg 'pattern'".bright_yellow());
        println!("  • Find files: {}", "fd 'filename'".bright_yellow());
        println!("  • View files: {}", "bat file.txt".bright_yellow());
        println!("  • List files: {}", "eza -la --icons".bright_yellow());
        println!("  • Disk usage: {}", "dust".bright_yellow());
    }
}
