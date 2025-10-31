use anyhow::{Context, Result};
use std::path::Path;

pub fn ensure_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .context(format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure_directory(parent)?;
    }

    std::fs::write(path, content)
        .context(format!("Failed to write file: {}", path.display()))?;

    Ok(())
}

pub fn read_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .context(format!("Failed to read file: {}", path.display()))
}
