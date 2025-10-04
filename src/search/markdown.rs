use std::{io::{Read}, path::PathBuf};
use crate::error::Result;

pub struct MarkdownFile {
    pub path: PathBuf,
    pub name: String,
    pub content: Option<String>,
}

impl MarkdownFile {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Self {
            path: path.clone(),
            name,
            content: None,
        }
    }

    pub fn load_content(&mut self) -> Result<()> {
        let mut file = std::fs::File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.content = Some(content);
        Ok(())
    }
}
