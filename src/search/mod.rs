pub mod markdown;

#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::env;

use walkdir::WalkDir;
use crate::error::Result;

pub use crate::search::markdown::MarkdownFile;

pub fn find_markdown_files(dir: &str) -> Result<Vec<MarkdownFile>> {
    let expanded_dir = expand_tilde(dir)?;
    let paths: Vec<PathBuf> = WalkDir::new(expanded_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect();

    Ok(convert_to_files(paths))
}

fn convert_to_files(paths: Vec<PathBuf>) -> Vec<MarkdownFile> {
    paths.into_iter().map(MarkdownFile::new).collect()
}

/// Expand tilde (~) to home directory path
pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    if path.starts_with("~/") {
        let home = env::var("HOME")
            .map_err(|_| crate::error::MarkError::search("Could not find HOME environment variable"))?;
        Ok(PathBuf::from(home).join(&path[2..]))
    } else if path == "~" {
        let home = env::var("HOME")
            .map_err(|_| crate::error::MarkError::search("Could not find HOME environment variable"))?;
        Ok(PathBuf::from(home))
    } else {
        Ok(PathBuf::from(path))
    }
}
