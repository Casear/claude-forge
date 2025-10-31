use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use colored::Colorize;
use std::path::{Path, PathBuf};

use crate::templates::registry::TemplateRegistry;
use crate::utils::fs::ensure_directory;

/// Add components (agents, commands, hooks) to the project
#[derive(Args)]
pub struct AddCommand {
    #[command(subcommand)]
    component: Component,
}

#[derive(Subcommand)]
enum Component {
    /// Add a custom subagent
    Agent {
        /// Name of the agent
        name: String,

        /// Custom template file path
        #[arg(short, long)]
        template: Option<PathBuf>,

        /// Open in editor after creation
        #[arg(short, long)]
        edit: bool,
    },

    /// Add a slash command
    Command {
        /// Name of the command
        name: String,

        /// Custom template file path
        #[arg(short, long)]
        template: Option<PathBuf>,

        /// Open in editor after creation
        #[arg(short, long)]
        edit: bool,
    },

    /// Add a hook
    Hook {
        /// Name of the hook
        name: String,

        /// Hook event type
        #[arg(short, long, value_name = "EVENT")]
        event: Option<String>,

        /// Custom template file path
        #[arg(short, long)]
        template: Option<PathBuf>,

        /// Open in editor after creation
        #[arg(short, long)]
        edit: bool,
    },
}

impl AddCommand {
    pub async fn execute(self) -> Result<()> {
        match self.component {
            Component::Agent { ref name, ref template, edit } => {
                self.add_agent(name, template.as_deref(), edit).await
            }
            Component::Command { ref name, ref template, edit } => {
                self.add_command(name, template.as_deref(), edit).await
            }
            Component::Hook { ref name, ref event, ref template, edit } => {
                self.add_hook(name, event.as_deref(), template.as_deref(), edit).await
            }
        }
    }

    async fn add_agent(&self, name: &str, template: Option<&Path>, edit: bool) -> Result<()> {
        println!("{}", format!("🤖 Adding agent: {}", name).bright_blue());

        let claude_dir = PathBuf::from(".claude");
        if !claude_dir.exists() {
            anyhow::bail!("No .claude directory found. Run 'claude-forge init' first.");
        }

        let agents_dir = claude_dir.join("agents");
        ensure_directory(&agents_dir)?;

        let agent_file = agents_dir.join(format!("{}.md", name));

        if agent_file.exists() {
            anyhow::bail!("Agent '{}' already exists", name);
        }

        // Load template
        let registry = TemplateRegistry::new();
        let content = if let Some(template_path) = template {
            std::fs::read_to_string(template_path)
                .context("Failed to read template file")?
        } else {
            registry.get_default_agent_template(name)?
        };

        // Write agent file
        std::fs::write(&agent_file, content)
            .context("Failed to write agent file")?;

        println!("✓ Agent created: {}", agent_file.display().to_string().bright_yellow());

        if edit {
            self.open_in_editor(&agent_file)?;
        }

        Ok(())
    }

    async fn add_command(&self, name: &str, template: Option<&Path>, edit: bool) -> Result<()> {
        println!("{}", format!("⚡ Adding command: {}", name).bright_blue());

        let claude_dir = PathBuf::from(".claude");
        if !claude_dir.exists() {
            anyhow::bail!("No .claude directory found. Run 'claude-forge init' first.");
        }

        let commands_dir = claude_dir.join("commands");
        ensure_directory(&commands_dir)?;

        let command_file = commands_dir.join(format!("{}.md", name));

        if command_file.exists() {
            anyhow::bail!("Command '{}' already exists", name);
        }

        // Load template
        let registry = TemplateRegistry::new();
        let content = if let Some(template_path) = template {
            std::fs::read_to_string(template_path)
                .context("Failed to read template file")?
        } else {
            registry.get_default_command_template(name)?
        };

        // Write command file
        std::fs::write(&command_file, content)
            .context("Failed to write command file")?;

        println!("✓ Command created: {}", command_file.display().to_string().bright_yellow());

        if edit {
            self.open_in_editor(&command_file)?;
        }

        Ok(())
    }

    async fn add_hook(&self, name: &str, event: Option<&str>, template: Option<&Path>, edit: bool) -> Result<()> {
        println!("{}", format!("🪝 Adding hook: {}", name).bright_blue());

        let claude_dir = PathBuf::from(".claude");
        if !claude_dir.exists() {
            anyhow::bail!("No .claude directory found. Run 'claude-forge init' first.");
        }

        let hooks_dir = claude_dir.join("hooks");
        ensure_directory(&hooks_dir)?;

        let hook_file = hooks_dir.join(format!("{}.sh", name));

        if hook_file.exists() {
            anyhow::bail!("Hook '{}' already exists", name);
        }

        // Load template
        let registry = TemplateRegistry::new();
        let content = if let Some(template_path) = template {
            std::fs::read_to_string(template_path)
                .context("Failed to read template file")?
        } else {
            registry.get_default_hook_template(name, event)?
        };

        // Write hook file
        std::fs::write(&hook_file, content)
            .context("Failed to write hook file")?;

        // Make executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&hook_file)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&hook_file, perms)?;
        }

        println!("✓ Hook created: {}", hook_file.display().to_string().bright_yellow());

        if edit {
            self.open_in_editor(&hook_file)?;
        }

        Ok(())
    }

    fn open_in_editor(&self, path: &PathBuf) -> Result<()> {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        println!("Opening in {}...", editor);

        std::process::Command::new(editor)
            .arg(path)
            .status()
            .context("Failed to open editor")?;

        Ok(())
    }
}
