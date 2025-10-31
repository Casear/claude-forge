pub mod detector;
pub mod generator;
pub mod validator;
pub mod tools_detector;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    TypeScript,
    JavaScript,
    Python,
    Go,
    Rust,
    Java,
    Elixir,
    Erlang,
}

impl Language {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "typescript" | "ts" => Ok(Language::TypeScript),
            "javascript" | "js" => Ok(Language::JavaScript),
            "python" | "py" => Ok(Language::Python),
            "go" | "golang" => Ok(Language::Go),
            "rust" | "rs" => Ok(Language::Rust),
            "java" => Ok(Language::Java),
            "elixir" | "ex" => Ok(Language::Elixir),
            "erlang" | "erl" => Ok(Language::Erlang),
            _ => Err(anyhow!("Unsupported language: {}", s)),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Language::TypeScript => "TypeScript",
            Language::JavaScript => "JavaScript",
            Language::Python => "Python",
            Language::Go => "Go",
            Language::Rust => "Rust",
            Language::Java => "Java",
            Language::Elixir => "Elixir",
            Language::Erlang => "Erlang",
        }
    }

    pub fn file_extensions(&self) -> Vec<&'static str> {
        match self {
            Language::TypeScript => vec!["ts", "tsx"],
            Language::JavaScript => vec!["js", "jsx", "mjs", "cjs"],
            Language::Python => vec!["py"],
            Language::Go => vec!["go"],
            Language::Rust => vec!["rs"],
            Language::Java => vec!["java"],
            Language::Elixir => vec!["ex", "exs"],
            Language::Erlang => vec!["erl", "hrl"],
        }
    }

    pub fn marker_files(&self) -> Vec<&'static str> {
        match self {
            Language::TypeScript => vec!["tsconfig.json", "package.json"],
            Language::JavaScript => vec!["package.json"],
            Language::Python => vec!["pyproject.toml", "setup.py", "requirements.txt"],
            Language::Go => vec!["go.mod", "go.sum"],
            Language::Rust => vec!["Cargo.toml", "Cargo.lock"],
            Language::Java => vec!["pom.xml", "build.gradle", "build.gradle.kts"],
            Language::Elixir => vec!["mix.exs"],
            Language::Erlang => vec!["rebar.config", "rebar.lock"],
        }
    }
}
