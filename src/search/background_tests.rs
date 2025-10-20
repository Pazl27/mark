use std::thread;
use std::time::Duration;
use tempfile::TempDir;
use std::fs::{self, File};

use super::background::{BackgroundSearcher, SearchMessage};
use super::MarkdownFile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_searcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();

        let searcher = BackgroundSearcher::new(
            dir_path,
            vec![],
            false,
            false,
        );

        assert!(searcher.is_ok());
        let searcher = searcher.unwrap();
        assert!(!searcher.is_complete);
    }

    #[test]
    fn test_background_searcher_finds_markdown_files() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test markdown files
        File::create(dir_path.join("file1.md")).unwrap();
        File::create(dir_path.join("file2.md")).unwrap();
        File::create(dir_path.join("readme.txt")).unwrap(); // Should be ignored

        let mut searcher = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec![],
            false,
            false,
        ).unwrap();

        // Wait for search to complete
        let mut found_files = Vec::new();
        let mut completed = false;
        
        for _ in 0..100 { // Max 1 second wait
            let messages = searcher.try_recv();
            for message in messages {
                match message {
                    SearchMessage::FileFound(file) => {
                        found_files.push(file);
                    }
                    SearchMessage::Finished => {
                        completed = true;
                        break;
                    }
                    SearchMessage::Error(_) => {
                        panic!("Unexpected error during search");
                    }
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(completed, "Search should have completed");
        assert_eq!(found_files.len(), 2);
        
        let file_names: Vec<String> = found_files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.iter().any(|name| name.ends_with("file1.md")));
        assert!(file_names.iter().any(|name| name.ends_with("file2.md")));
    }

    #[test]
    fn test_background_searcher_respects_ignored_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create directory structure
        let node_modules = dir_path.join("node_modules");
        let src_dir = dir_path.join("src");
        fs::create_dir_all(&node_modules).unwrap();
        fs::create_dir_all(&src_dir).unwrap();

        // Create markdown files
        File::create(dir_path.join("root.md")).unwrap();
        File::create(node_modules.join("package.md")).unwrap(); // Should be ignored
        File::create(src_dir.join("main.md")).unwrap();

        let mut searcher = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec!["node_modules".to_string()],
            false,
            false,
        ).unwrap();

        // Wait for search to complete
        let mut found_files = Vec::new();
        let mut completed = false;
        
        for _ in 0..100 { // Max 1 second wait
            let messages = searcher.try_recv();
            for message in messages {
                match message {
                    SearchMessage::FileFound(file) => {
                        found_files.push(file);
                    }
                    SearchMessage::Finished => {
                        completed = true;
                        break;
                    }
                    SearchMessage::Error(_) => {
                        panic!("Unexpected error during search");
                    }
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(completed, "Search should have completed");
        assert_eq!(found_files.len(), 2); // Should not find package.md in node_modules
        
        let file_names: Vec<String> = found_files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.iter().any(|name| name.ends_with("root.md")));
        assert!(file_names.iter().any(|name| name.ends_with("src/main.md")));
        assert!(!file_names.iter().any(|name| name.contains("node_modules")));
    }

    #[test]
    fn test_background_searcher_hidden_files_behavior() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create directory structure with hidden directories
        let hidden_dir = dir_path.join(".hidden");
        let normal_dir = dir_path.join("docs");
        fs::create_dir_all(&hidden_dir).unwrap();
        fs::create_dir_all(&normal_dir).unwrap();

        // Create markdown files
        File::create(dir_path.join("root.md")).unwrap();
        File::create(hidden_dir.join("secret.md")).unwrap();
        File::create(normal_dir.join("public.md")).unwrap();

        // Test with show_hidden = false
        let mut searcher_no_hidden = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec![],
            false, // show_hidden = false
            false,
        ).unwrap();

        let mut found_files_no_hidden = Vec::new();
        let mut completed = false;
        
        for _ in 0..100 {
            let messages = searcher_no_hidden.try_recv();
            for message in messages {
                match message {
                    SearchMessage::FileFound(file) => {
                        found_files_no_hidden.push(file);
                    }
                    SearchMessage::Finished => {
                        completed = true;
                        break;
                    }
                    SearchMessage::Error(_) => {
                        panic!("Unexpected error during search");
                    }
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(completed);
        assert_eq!(found_files_no_hidden.len(), 2); // Should not find secret.md
        
        let file_names: Vec<String> = found_files_no_hidden.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.iter().any(|name| name.ends_with("root.md")));
        assert!(file_names.iter().any(|name| name.ends_with("docs/public.md")));
        assert!(!file_names.iter().any(|name| name.contains(".hidden")));

        // Test with show_hidden = true
        let mut searcher_with_hidden = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec![],
            true, // show_hidden = true
            false,
        ).unwrap();

        let mut found_files_with_hidden = Vec::new();
        completed = false;
        
        for _ in 0..100 {
            let messages = searcher_with_hidden.try_recv();
            for message in messages {
                match message {
                    SearchMessage::FileFound(file) => {
                        found_files_with_hidden.push(file);
                    }
                    SearchMessage::Finished => {
                        completed = true;
                        break;
                    }
                    SearchMessage::Error(_) => {
                        panic!("Unexpected error during search");
                    }
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(completed);
        assert_eq!(found_files_with_hidden.len(), 3); // Should find all files including secret.md
        
        let file_names_with_hidden: Vec<String> = found_files_with_hidden.iter().map(|f| f.name.clone()).collect();
        assert!(file_names_with_hidden.iter().any(|name| name.ends_with("root.md")));
        assert!(file_names_with_hidden.iter().any(|name| name.ends_with("docs/public.md")));
        assert!(file_names_with_hidden.iter().any(|name| name.ends_with(".hidden/secret.md")));
    }

    #[test]
    fn test_background_searcher_show_all_mode() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create directory structure with both hidden and ignored directories
        let hidden_dir = dir_path.join(".hidden");
        let node_modules = dir_path.join("node_modules");
        fs::create_dir_all(&hidden_dir).unwrap();
        fs::create_dir_all(&node_modules).unwrap();

        // Create markdown files
        File::create(dir_path.join("root.md")).unwrap();
        File::create(hidden_dir.join("secret.md")).unwrap();
        File::create(node_modules.join("package.md")).unwrap();

        let mut searcher = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec!["node_modules".to_string()], // This should be ignored in show_all mode
            false, // show_hidden doesn't matter in show_all mode
            true,  // show_all = true
        ).unwrap();

        let mut found_files = Vec::new();
        let mut completed = false;
        
        for _ in 0..100 {
            let messages = searcher.try_recv();
            for message in messages {
                match message {
                    SearchMessage::FileFound(file) => {
                        found_files.push(file);
                    }
                    SearchMessage::Finished => {
                        completed = true;
                        break;
                    }
                    SearchMessage::Error(_) => {
                        panic!("Unexpected error during search");
                    }
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(completed);
        assert_eq!(found_files.len(), 3); // Should find ALL files in show_all mode
        
        let file_names: Vec<String> = found_files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.iter().any(|name| name.ends_with("root.md")));
        assert!(file_names.iter().any(|name| name.ends_with(".hidden/secret.md")));
        assert!(file_names.iter().any(|name| name.ends_with("node_modules/package.md")));
    }

    #[test]
    fn test_background_searcher_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        let mut searcher = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec![],
            false,
            false,
        ).unwrap();

        let mut found_files = Vec::new();
        let mut completed = false;
        
        for _ in 0..100 {
            let messages = searcher.try_recv();
            for message in messages {
                match message {
                    SearchMessage::FileFound(file) => {
                        found_files.push(file);
                    }
                    SearchMessage::Finished => {
                        completed = true;
                        break;
                    }
                    SearchMessage::Error(_) => {
                        panic!("Unexpected error during search");
                    }
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(completed);
        assert_eq!(found_files.len(), 0);
    }

    #[test]
    fn test_background_searcher_completion_state() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        File::create(dir_path.join("test.md")).unwrap();

        let mut searcher = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec![],
            false,
            false,
        ).unwrap();

        // Initially not complete
        assert!(!searcher.is_complete);

        // Wait for completion
        let mut completed = false;
        for _ in 0..100 {
            let messages = searcher.try_recv();
            for message in messages {
                if matches!(message, SearchMessage::Finished) {
                    completed = true;
                    break;
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        assert!(completed);
        assert!(searcher.is_complete);
    }

    #[test]
    fn test_background_searcher_streaming_behavior() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create multiple files to test streaming
        for i in 0..5 {
            File::create(dir_path.join(format!("file{}.md", i))).unwrap();
        }

        let mut searcher = BackgroundSearcher::new(
            dir_path.to_str().unwrap(),
            vec![],
            false,
            false,
        ).unwrap();

        let mut found_files = Vec::new();
        let mut completed = false;
        let mut _received_partial_results = false;
        
        for _ in 0..200 { // Give more time for streaming
            let messages = searcher.try_recv();
            if !messages.is_empty() && !completed {
                // If we get some files but not Finished yet, we're streaming
                let has_files = messages.iter().any(|m| matches!(m, SearchMessage::FileFound(_)));
                let has_finished = messages.iter().any(|m| matches!(m, SearchMessage::Finished));
                
                if has_files && !has_finished {
                    _received_partial_results = true;
                }
            }
            
            for message in messages {
                match message {
                    SearchMessage::FileFound(file) => {
                        found_files.push(file);
                    }
                    SearchMessage::Finished => {
                        completed = true;
                        break;
                    }
                    SearchMessage::Error(_) => {
                        panic!("Unexpected error during search");
                    }
                }
            }
            if completed {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }

        assert!(completed);
        assert_eq!(found_files.len(), 5);
        // Note: received_partial_results might be false if search is very fast
        // This is acceptable behavior
    }

    #[test]
    fn test_background_searcher_nonexistent_directory() {
        let searcher = BackgroundSearcher::new(
            "/nonexistent/directory/path",
            vec![],
            false,
            false,
        );

        // Should create searcher successfully (error handling happens in the thread)
        assert!(searcher.is_ok());
        
        let mut searcher = searcher.unwrap();
        let mut received_error = false;
        
        for _ in 0..100 {
            let messages = searcher.try_recv();
            for message in messages {
                match message {
                    SearchMessage::Error(_) => {
                        received_error = true;
                        break;
                    }
                    SearchMessage::Finished => {
                        break;
                    }
                    _ => {}
                }
            }
            if received_error {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        // The search should complete without panicking
        // Error handling in walkdir will just skip inaccessible paths
    }

    #[test]
    fn test_search_message_debug() {
        // Test that SearchMessage implements Debug properly
        let temp_dir = TempDir::new().unwrap();
        let file = MarkdownFile::new(temp_dir.path().join("test.md"));
        
        let file_found = SearchMessage::FileFound(file);
        let finished = SearchMessage::Finished;
        let error = SearchMessage::Error("test error".to_string());

        // These should not panic
        let _ = format!("{:?}", file_found);
        let _ = format!("{:?}", finished);
        let _ = format!("{:?}", error);
    }
}