// src/markdown.rs

use std::fs;
use std::path::Path;

pub fn read_markdown_file(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("Markdown file not found: {}", path.display()));
    }

    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to read Markdown file: {}", e)),
    }
}
