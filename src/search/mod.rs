pub mod markdown;

#[cfg(test)]
mod tests;

use std::path::PathBuf;

use walkdir::WalkDir;
use crate::error::Result;

pub use crate::search::markdown::MarkdownFile;

pub fn find_markdown_files(dir: &str) -> Result<Vec<MarkdownFile>> {
    let paths: Vec<PathBuf> = WalkDir::new(dir)
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
