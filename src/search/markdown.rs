use chrono::{DateTime, Local};

use crate::error::Result;
use std::{io::Read, path::PathBuf};

pub struct MarkdownFile {
    pub path: PathBuf,
    pub name: String,
    pub content: Option<String>,
    pub created_at: Option<String>,
}

impl MarkdownFile {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let created_at = std::fs::metadata(&path)
            .and_then(|meta| meta.created())
            .ok()
            .and_then(|time| {
                let datetime: DateTime<Local> = time.into();
                Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
            });

        Self {
            path: path.clone(),
            name,
            content: None,
            created_at,
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
