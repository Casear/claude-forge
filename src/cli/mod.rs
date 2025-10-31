mod init;
mod add;
mod tools;
mod mcp;
mod config;
mod upgrade;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

pub use init::InitCommand;
pub use add::AddCommand;
pub use tools::ToolsCommand;
pub use mcp::McpCommand;
pub use config::ConfigCommand;
pub use upgrade::UpgradeCommand;
pub use update::UpdateCommand;

/// Claude Forge - A CLI tool for managing Claude Code configurations
#[derive(Parser)]
#[command(
    name = "claude-forge",
    about = "Manage Claude Code configurations and templates",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Claude Code configuration for a project
    #[command(alias = "i")]
    Init(InitCommand),

    /// Add components (agents, commands, hooks)
    #[command(alias = "a")]
    Add(AddCommand),

    /// Manage modern CLI tools
    #[command(alias = "t")]
    Tools(ToolsCommand),

    /// Manage MCP servers
    #[command(alias = "m")]
    Mcp(McpCommand),

    /// Configuration management
    #[command(alias = "c")]
    Config(ConfigCommand),

    /// Upgrade claude-forge to latest version
    #[command(alias = "up")]
    Upgrade(UpgradeCommand),

    /// Update templates to latest version
    #[command(alias = "upd")]
    Update(UpdateCommand),
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        // Set up logging based on verbosity
        if self.verbose {
            std::env::set_var("RUST_LOG", "debug");
        }

        // Print banner
        self.print_banner();

        // Execute the appropriate command
        match self.command {
            Commands::Init(cmd) => cmd.execute().await,
            Commands::Add(cmd) => cmd.execute().await,
            Commands::Tools(cmd) => cmd.execute().await,
            Commands::Mcp(cmd) => cmd.execute().await,
            Commands::Config(cmd) => cmd.execute().await,
            Commands::Upgrade(cmd) => cmd.execute().await,
            Commands::Update(cmd) => cmd.execute().await,
        }
    }

    fn print_banner(&self) {
        println!("{}", "🔨 Claude Forge".bright_cyan().bold());
        println!("{}\n", "   Manage Claude Code configurations".dimmed());
    }
}
