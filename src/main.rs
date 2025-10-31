mod cli;
mod core;
mod templates;
mod tools;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Execute the command
    cli.execute().await?;

    Ok(())
}
