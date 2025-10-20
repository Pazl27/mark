pub mod background;
pub mod markdown;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod background_tests;

use std::env;
use std::path::PathBuf;

use crate::error::Result;
use walkdir::WalkDir;

pub use crate::search::markdown::MarkdownFile;

pub fn find_markdown_files(dir: &str) -> Result<Vec<MarkdownFile>> {
    find_markdown_files_with_ignored(dir, &[])
}

pub fn find_markdown_files_with_ignored(
    dir: &str,
    ignored_dirs: &[String],
) -> Result<Vec<MarkdownFile>> {
    let expanded_dir = expand_tilde(dir)?;
    let paths: Vec<PathBuf> = WalkDir::new(expanded_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| {
            // Skip if path contains any ignored directories
            !e.path().components().any(|component| {
                component
                    .as_os_str()
                    .to_str()
                    .map(|s| ignored_dirs.contains(&s.to_string()))
                    .unwrap_or(false)
            })
        })
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect();

    Ok(convert_to_files(paths))
}

pub fn find_all_markdown_files_unfiltered(dir: &str) -> Result<Vec<MarkdownFile>> {
    let expanded_dir = expand_tilde(dir)?;
    let paths: Vec<PathBuf> = WalkDir::new(expanded_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect();

    Ok(convert_to_files(paths))
}

pub fn find_markdown_files_without_hidden(dir: &str) -> Result<Vec<MarkdownFile>> {
    find_markdown_files_without_hidden_with_ignored(dir, &[])
}

pub fn find_markdown_files_without_hidden_with_ignored(
    dir: &str,
    ignored_dirs: &[String],
) -> Result<Vec<MarkdownFile>> {
    let expanded_dir = expand_tilde(dir)?;
    let search_root = expanded_dir.clone();
    let paths: Vec<PathBuf> = WalkDir::new(expanded_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| {
            // Skip if the file is inside a hidden directory (relative to search root)
            let path = e.path();

            // Get the relative path from search root
            if let Ok(relative_path) = path.strip_prefix(&search_root) {
                // Check if any component in the relative path is a hidden directory
                for component in relative_path.components() {
                    if let Some(name_str) = component.as_os_str().to_str() {
                        if name_str.starts_with('.') && name_str != "." && name_str != ".." {
                            return false; // Skip this file - it's in a hidden directory
                        }
                    }
                }
            }
            true // Include this file
        })
        .filter(|e| {
            // Skip if path contains any ignored directories
            !e.path().components().any(|component| {
                component
                    .as_os_str()
                    .to_str()
                    .map(|s| ignored_dirs.contains(&s.to_string()))
                    .unwrap_or(false)
            })
        })
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
        let home = env::var("HOME").map_err(|_| {
            crate::error::MarkError::search("Could not find HOME environment variable")
        })?;
        Ok(PathBuf::from(home).join(&path[2..]))
    } else if path == "~" {
        let home = env::var("HOME").map_err(|_| {
            crate::error::MarkError::search("Could not find HOME environment variable")
        })?;
        Ok(PathBuf::from(home))
    } else {
        Ok(PathBuf::from(path))
    }
}
