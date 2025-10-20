use crate::error::Result;
use crate::search::{expand_tilde, MarkdownFile};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub enum SearchMessage {
    FileFound(MarkdownFile),
    Finished,
    Error(String),
}

pub struct BackgroundSearcher {
    receiver: Receiver<SearchMessage>,
    _handle: thread::JoinHandle<()>,
    pub is_complete: bool,
}

impl BackgroundSearcher {
    pub fn new(
        directory: &str,
        ignored_dirs: Vec<String>,
        show_hidden: bool,
        show_all: bool,
    ) -> Result<Self> {
        let (tx, rx) = mpsc::channel();
        let dir = directory.to_string();

        let handle = thread::spawn(move || {
            if let Err(e) = Self::search_files(&tx, &dir, ignored_dirs, show_hidden, show_all) {
                let _ = tx.send(SearchMessage::Error(e.to_string()));
            }
            let _ = tx.send(SearchMessage::Finished);
        });

        Ok(Self {
            receiver: rx,
            _handle: handle,
            is_complete: false,
        })
    }

    pub fn try_recv(&mut self) -> Vec<SearchMessage> {
        let mut messages = Vec::new();

        while let Ok(message) = self.receiver.try_recv() {
            match &message {
                SearchMessage::Finished => self.is_complete = true,
                _ => {}
            }
            messages.push(message);
        }

        messages
    }

    fn search_files(
        tx: &Sender<SearchMessage>,
        directory: &str,
        ignored_dirs: Vec<String>,
        show_hidden: bool,
        show_all: bool,
    ) -> Result<()> {
        let expanded_dir = expand_tilde(directory)?;
        let search_root = expanded_dir.clone();

        for entry in WalkDir::new(expanded_dir) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue, // Skip inaccessible files/directories
            };

            let path = entry.path();

            // Check if it's a markdown file
            if !path.extension().map(|ext| ext == "md").unwrap_or(false) {
                continue;
            }

            // Skip if path contains any ignored directories (unless show_all is true)
            if !show_all
                && path.components().any(|component| {
                    component
                        .as_os_str()
                        .to_str()
                        .map(|s| ignored_dirs.contains(&s.to_string()))
                        .unwrap_or(false)
                })
            {
                continue;
            }

            // Handle hidden files unless show_all is true
            if !show_all && !show_hidden {
                // Skip if the file is inside a hidden directory (relative to search root)
                if let Ok(relative_path) = path.strip_prefix(&search_root) {
                    // Check if any component in the relative path is a hidden directory
                    let mut should_skip = false;
                    for component in relative_path.components() {
                        if let Some(name_str) = component.as_os_str().to_str() {
                            if name_str.starts_with('.') && name_str != "." && name_str != ".." {
                                should_skip = true;
                                break;
                            }
                        }
                    }
                    if should_skip {
                        continue;
                    }
                }
            }

            // Create MarkdownFile and send it
            let markdown_file = MarkdownFile::new(path.to_path_buf());
            if tx.send(SearchMessage::FileFound(markdown_file)).is_err() {
                // Receiver has been dropped, stop searching
                break;
            }

            // Add a small delay to prevent overwhelming the UI and make loading animation visible
            thread::sleep(std::time::Duration::from_millis(5));
        }

        Ok(())
    }
}
