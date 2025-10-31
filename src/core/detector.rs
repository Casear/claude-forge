use anyhow::{anyhow, Result};
use std::path::Path;

use super::Language;

pub struct ProjectDetector;

impl ProjectDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self, path: &Path) -> Result<Language> {
        // Priority order for detection
        let languages = vec![
            Language::Rust,
            Language::Go,
            Language::TypeScript,
            Language::JavaScript,
            Language::Python,
            Language::Java,
            Language::Elixir,
            Language::Erlang,
        ];

        for lang in languages {
            if self.has_marker_files(path, &lang) {
                // Additional verification for TypeScript vs JavaScript
                if lang == Language::JavaScript {
                    if self.is_typescript_project(path) {
                        return Ok(Language::TypeScript);
                    }
                }
                return Ok(lang);
            }
        }

        Err(anyhow!("Could not detect project language"))
    }

    fn has_marker_files(&self, path: &Path, language: &Language) -> bool {
        let markers = language.marker_files();

        for marker in markers {
            let marker_path = path.join(marker);
            if marker_path.exists() {
                return true;
            }
        }

        false
    }

    fn is_typescript_project(&self, path: &Path) -> bool {
        // Check for tsconfig.json
        if path.join("tsconfig.json").exists() {
            return true;
        }

        // Check package.json for typescript dependency
        let package_json_path = path.join("package.json");
        if package_json_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&package_json_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    // Check dependencies and devDependencies
                    if let Some(deps) = json.get("dependencies") {
                        if deps.get("typescript").is_some() {
                            return true;
                        }
                    }
                    if let Some(dev_deps) = json.get("devDependencies") {
                        if dev_deps.get("typescript").is_some() {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn detect_framework(&self, path: &Path, _language: Language) -> Option<String> {
        // Try to detect common frameworks
        let package_json_path = path.join("package.json");
        if package_json_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&package_json_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    // Check for popular frameworks
                    let frameworks = vec![
                        ("next", "Next.js"),
                        ("react", "React"),
                        ("vue", "Vue"),
                        ("express", "Express"),
                        ("fastify", "Fastify"),
                        ("nestjs", "NestJS"),
                    ];

                    for (dep, framework) in frameworks {
                        if let Some(deps) = json.get("dependencies") {
                            if deps.get(dep).is_some() {
                                return Some(framework.to_string());
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), "").unwrap();

        let detector = ProjectDetector::new();
        let result = detector.detect(temp_dir.path()).unwrap();

        assert_eq!(result, Language::Rust);
    }

    #[test]
    fn test_detect_typescript_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("tsconfig.json"), "").unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"dependencies": {"typescript": "^5.0.0"}}"#).unwrap();

        let detector = ProjectDetector::new();
        let result = detector.detect(temp_dir.path()).unwrap();

        assert_eq!(result, Language::TypeScript);
    }
}
