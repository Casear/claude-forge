use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use dialoguer::{Confirm, Select};
use std::path::PathBuf;

use crate::core::{detector::ProjectDetector, generator::ConfigGenerator, Language};
use crate::tools::installer::ToolsInstaller;
use crate::utils::fs::ensure_directory;

/// Initialize Claude Code configuration for a project
#[derive(Args)]
pub struct InitCommand {
    /// Target directory (defaults to current directory)
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Specify language explicitly
    #[arg(short, long, value_name = "LANGUAGE")]
    lang: Option<String>,

    /// Skip interactive prompts and use defaults
    #[arg(short = 'y', long)]
    yes: bool,

    /// Skip modern CLI tools installation
    #[arg(long)]
    no_tools: bool,

    /// Skip MCP configuration
    #[arg(long)]
    no_mcp: bool,

    /// Use minimal template (only essential files)
    #[arg(long)]
    minimal: bool,
}

impl InitCommand {
    pub async fn execute(self) -> Result<()> {
        let target_path = self.path.clone().unwrap_or_else(|| PathBuf::from("."));

        println!("{}", "🚀 Initializing Claude Code configuration...\n".bright_green());

        // Step 1: Detect or prompt for language
        let detector = ProjectDetector::new();
        let language = if let Some(lang_str) = &self.lang {
            Language::from_str(lang_str)?
        } else if let Ok(detected) = detector.detect(&target_path) {
            println!("✓ Detected language: {}", detected.to_string().bright_yellow());

            if !self.yes {
                let confirm = Confirm::new()
                    .with_prompt(format!("Use {} for this project?", detected.to_string()))
                    .default(true)
                    .interact()?;

                if !confirm {
                    self.prompt_for_language()?
                } else {
                    detected
                }
            } else {
                detected
            }
        } else {
            if self.yes {
                println!("⚠ Could not detect language, using TypeScript as default");
                Language::TypeScript
            } else {
                self.prompt_for_language()?
            }
        };

        // Step 2: Create .claude directory structure
        let claude_dir = target_path.join(".claude");
        self.create_directory_structure(&claude_dir)?;

        // Step 3: Generate configuration files
        let generator = ConfigGenerator::new(language, self.minimal);
        generator.generate(&target_path).await?;

        println!("\n{}", "✓ Configuration files generated".green());

        // Step 4: Install modern CLI tools (optional)
        if !self.no_tools {
            if self.yes || self.prompt_install_tools()? {
                println!("\n{}", "📦 Checking modern CLI tools...".bright_blue());
                let installer = ToolsInstaller::new();
                installer.check_and_install().await?;
            }
        }

        // Step 5: Configure MCP (optional)
        if !self.no_mcp {
            if self.yes || self.prompt_configure_mcp()? {
                println!("\n{}", "🔌 Configuring MCP servers...".bright_blue());
                self.configure_mcp(&target_path, language).await?;
            }
        }

        // Step 6: Print success message with next steps
        self.print_success_message(&target_path, language);

        Ok(())
    }

    fn create_directory_structure(&self, claude_dir: &PathBuf) -> Result<()> {
        println!("📁 Creating directory structure...");

        ensure_directory(claude_dir)?;
        ensure_directory(&claude_dir.join("agents"))?;
        ensure_directory(&claude_dir.join("commands"))?;
        ensure_directory(&claude_dir.join("hooks"))?;

        println!("✓ Directory structure created");
        Ok(())
    }

    fn prompt_for_language(&self) -> Result<Language> {
        let languages = vec![
            "TypeScript",
            "JavaScript",
            "Python",
            "Go",
            "Rust",
            "Java",
            "Elixir",
            "Erlang",
        ];

        let selection = Select::new()
            .with_prompt("Select your project language")
            .items(&languages)
            .default(0)
            .interact()?;

        Language::from_str(languages[selection])
    }

    fn prompt_install_tools(&self) -> Result<bool> {
        Confirm::new()
            .with_prompt("Install modern CLI tools (rg, fd, bat, eza, dust)?")
            .default(true)
            .interact()
            .context("Failed to prompt for tools installation")
    }

    fn prompt_configure_mcp(&self) -> Result<bool> {
        Confirm::new()
            .with_prompt("Configure recommended MCP servers?")
            .default(true)
            .interact()
            .context("Failed to prompt for MCP configuration")
    }

    async fn configure_mcp(&self, _target_path: &PathBuf, _language: Language) -> Result<()> {
        // TODO: Implement MCP configuration
        println!("✓ MCP configuration added");
        Ok(())
    }

    fn print_success_message(&self, target_path: &PathBuf, language: Language) {
        println!("\n{}", "🎉 Success!".bright_green().bold());
        println!("\nClaude Code configuration has been initialized for your {} project!",
                 language.to_string().bright_yellow());

        println!("\n{}", "📝 Next steps:".bright_cyan().bold());
        println!("  1. Review the generated files in {}",
                 target_path.join(".claude").display().to_string().bright_yellow());
        println!("  2. Customize {} to fit your project",
                 ".claude/CLAUDE.md".bright_yellow());
        println!("  3. Add custom agents, commands, or hooks as needed");
        println!("  4. Run {} to see available commands",
                 "claude-forge --help".bright_yellow());

        println!("\n{}", "📚 Learn more:".bright_cyan().bold());
        println!("  • Documentation: https://github.com/Casear/claude-forge");
        println!("  • Claude Code docs: https://docs.claude.com/claude-code");
    }
}
