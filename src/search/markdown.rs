use chrono::{DateTime, Local};

use crate::error::Result;
use std::{env::current_dir, io::Read, path::PathBuf};

#[derive(Clone, Debug)]
pub struct MarkdownFile {
    pub path: PathBuf,
    pub name: String,
    pub content: Option<String>,
    pub created_at: Option<String>,
}

impl MarkdownFile {
    pub fn new(path: PathBuf) -> Self {
        let current_dir = current_dir().unwrap_or_else(|_| PathBuf::from("."));

        let name = path
            .strip_prefix(&current_dir)
            .unwrap_or(&path)
            .to_str()
            .unwrap_or("unknown")
            .to_owned();

        let name = if name.starts_with("./") {
            name.trim_start_matches("./").to_string()
        } else {
            name
        };

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
