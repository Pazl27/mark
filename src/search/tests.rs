use super::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_file_new() {
        let path = PathBuf::from("/path/to/test.md");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        assert_eq!(md_file.name, "/path/to/test.md");
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_markdown_file_new_with_extension() {
        let path = PathBuf::from("/path/to/readme.markdown");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        assert_eq!(md_file.name, "/path/to/readme.markdown");
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_markdown_file_new_no_extension() {
        let path = PathBuf::from("/path/to/document");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        assert_eq!(md_file.name, "/path/to/document");
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_markdown_file_new_invalid_unicode() {
        // Create a path that should result in "unknown" name
        // The current implementation will show the full path if it can convert to string
        let path = PathBuf::from("/path/with/invalid/unicode/\u{FFFF}.md");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        // The path contains valid Unicode, so it will show the full path
        assert_eq!(md_file.name, "/path/with/invalid/unicode/\u{ffff}.md");
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_convert_to_files_empty() {
        let paths = vec![];
        let files = convert_to_files(paths);
        assert!(files.is_empty());
    }

    #[test]
    fn test_convert_to_files_single() {
        let paths = vec![PathBuf::from("test.md")];
        let files = convert_to_files(paths);

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "test.md");
        assert_eq!(files[0].path, PathBuf::from("test.md"));
    }

    #[test]
    fn test_convert_to_files_multiple() {
        let paths = vec![
            PathBuf::from("doc1.md"),
            PathBuf::from("doc2.md"),
            PathBuf::from("subdir/doc3.md"),
        ];
        let files = convert_to_files(paths);

        assert_eq!(files.len(), 3);
        assert_eq!(files[0].name, "doc1.md");
        assert_eq!(files[1].name, "doc2.md");
        assert_eq!(files[2].name, "subdir/doc3.md");
    }

    // Tests that use temporary filesystem but clean up after themselves
    #[test]
    fn test_markdown_file_load_content_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        let test_content = "# Test Markdown\n\nThis is a test file.";

        // Create test file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();

        // Test loading content
        let mut md_file = MarkdownFile::new(file_path);
        let result = md_file.load_content();

        assert!(result.is_ok());
        assert_eq!(md_file.content, Some(test_content.to_string()));
    }

    #[test]
    fn test_markdown_file_load_content_file_not_found() {
        let non_existent_path = PathBuf::from("/non/existent/file.md");
        let mut md_file = MarkdownFile::new(non_existent_path);

        let result = md_file.load_content();
        assert!(result.is_err());
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_markdown_file_load_content_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.md");

        // Create empty test file
        File::create(&file_path).unwrap();

        // Test loading content
        let mut md_file = MarkdownFile::new(file_path);
        let result = md_file.load_content();

        assert!(result.is_ok());
        assert_eq!(md_file.content, Some(String::new()));
    }

    #[test]
    fn test_find_markdown_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_str().unwrap();

        let result = find_markdown_files(dir_path);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_find_markdown_files_with_md_files() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create some test files
        File::create(dir_path.join("file1.md")).unwrap();
        File::create(dir_path.join("file2.md")).unwrap();
        File::create(dir_path.join("not_markdown.txt")).unwrap();

        let result = find_markdown_files(dir_path.to_str().unwrap());
        assert!(result.is_ok());
        let files = result.unwrap();

        assert_eq!(files.len(), 2);
        // Names will be full paths since we're not in the temp directory
        let names: Vec<&String> = files.iter().map(|f| &f.name).collect();
        assert!(names.iter().any(|name| name.ends_with("file1.md")));
        assert!(names.iter().any(|name| name.ends_with("file2.md")));
    }

    #[test]
    fn test_find_markdown_files_with_subdirectories() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create directory structure
        let sub_dir = dir_path.join("subdir");
        std::fs::create_dir(&sub_dir).unwrap();

        // Create files
        File::create(dir_path.join("root.md")).unwrap();
        File::create(sub_dir.join("nested.md")).unwrap();
        File::create(dir_path.join("other.txt")).unwrap();

        let result = find_markdown_files(dir_path.to_str().unwrap());
        assert!(result.is_ok());
        let files = result.unwrap();

        assert_eq!(files.len(), 2);
        // Names will be full paths since we're not in the temp directory
        let names: Vec<&String> = files.iter().map(|f| &f.name).collect();
        assert!(names.iter().any(|name| name.ends_with("root.md")));
        assert!(names.iter().any(|name| name.ends_with("subdir/nested.md")));
    }

    #[test]
    fn test_find_markdown_files_mixed_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create files with different extensions
        File::create(dir_path.join("doc.md")).unwrap();
        File::create(dir_path.join("readme.markdown")).unwrap();
        File::create(dir_path.join("text.txt")).unwrap();
        File::create(dir_path.join("data.json")).unwrap();
        File::create(dir_path.join("no_extension")).unwrap();

        let result = find_markdown_files(dir_path.to_str().unwrap());
        assert!(result.is_ok());
        let files = result.unwrap();

        // Only .md files should be found (not .markdown in current implementation)
        assert_eq!(files.len(), 1);
        // The name will be the full path since we're not in the temp directory
        assert!(files[0].name.ends_with("doc.md"));
    }

    #[test]
    fn test_find_markdown_files_nonexistent_directory() {
        let result = find_markdown_files("/non/existent/directory");
        // This should not panic but return an empty result since WalkDir filters errors
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_markdown_file_path_operations() {
        let paths = vec![
            PathBuf::from("simple.md"),
            PathBuf::from("/absolute/path/file.md"),
            PathBuf::from("relative/path/doc.md"),
            PathBuf::from("../parent/up.md"),
            PathBuf::from("./current/here.md"),
        ];

        let files = convert_to_files(paths.clone());

        // Test that path is preserved
        for (i, file) in files.iter().enumerate() {
            assert_eq!(file.path, paths[i]);
            assert_eq!(file.content, None);
        }

        // Test specific name behaviors (names are relative to current dir or full path)
        assert_eq!(files[0].name, "simple.md");
        assert_eq!(files[1].name, "/absolute/path/file.md");
        assert_eq!(files[2].name, "relative/path/doc.md");
        assert_eq!(files[3].name, "../parent/up.md");
        assert_eq!(files[4].name, "current/here.md"); // ./current/here.md becomes current/here.md
    }

    #[test]
    fn test_markdown_file_unicode_paths() {
        let paths = vec![
            PathBuf::from("文档.md"),
            PathBuf::from("документ.md"),
            PathBuf::from("ドキュメント.md"),
        ];

        let files = convert_to_files(paths.clone());

        assert_eq!(files.len(), 3);
        assert_eq!(files[0].name, "文档.md");
        assert_eq!(files[1].name, "документ.md");
        assert_eq!(files[2].name, "ドキュメント.md");
    }

    #[test]
    fn test_markdown_file_edge_case_names() {
        let paths = vec![
            PathBuf::from(".md"),            // Just extension
            PathBuf::from("file.md.backup"), // Multiple dots
            PathBuf::from("a.very.long.filename.with.many.dots.md"),
        ];

        let files = convert_to_files(paths);

        assert_eq!(files[0].name, ".md");
        assert_eq!(files[1].name, "file.md.backup");
        assert_eq!(files[2].name, "a.very.long.filename.with.many.dots.md");
    }

    #[test]
    fn test_expand_tilde_home_only() {
        // Save original HOME value
        let original_home = std::env::var("HOME").ok();
        
        // Mock HOME environment variable for testing
        std::env::set_var("HOME", "/home/testuser");

        let result = super::expand_tilde("~");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/home/testuser"));

        // Restore original HOME value
        if let Some(home) = original_home {
            std::env::set_var("HOME", home);
        }
    }

    #[test]
    fn test_expand_tilde_with_subpath() {
        // Mock HOME environment variable for testing
        std::env::set_var("HOME", "/home/testuser");

        let result = super::expand_tilde("~/Documents/markdown");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            PathBuf::from("/home/testuser/Documents/markdown")
        );

        // Clean up
        std::env::remove_var("HOME");
    }

    #[test]
    fn test_expand_tilde_no_expansion_needed() {
        let result = super::expand_tilde("/absolute/path");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("/absolute/path"));

        let result = super::expand_tilde("relative/path");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PathBuf::from("relative/path"));
    }

    #[test]
    fn test_expand_tilde_missing_home_env() {
        // Save original HOME value
        let original_home = std::env::var("HOME").ok();

        // Remove HOME environment variable
        std::env::remove_var("HOME");

        let result = super::expand_tilde("~");
        assert!(result.is_err());

        let result = super::expand_tilde("~/Documents");
        assert!(result.is_err());

        // Restore original HOME value
        if let Some(home) = original_home {
            std::env::set_var("HOME", home);
        }
    }

    #[test]
    fn test_integration_full_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create a complex directory structure with various files
        let sub_dir1 = dir_path.join("docs");
        let sub_dir2 = dir_path.join("src");
        let sub_dir3 = sub_dir1.join("guides");

        fs::create_dir_all(&sub_dir1).unwrap();
        fs::create_dir_all(&sub_dir2).unwrap();
        fs::create_dir_all(&sub_dir3).unwrap();

        // Create markdown files with content
        let mut readme = File::create(dir_path.join("README.md")).unwrap();
        readme
            .write_all(b"# Project\n\nThis is a test project.")
            .unwrap();

        let mut doc1 = File::create(sub_dir1.join("guide.md")).unwrap();
        doc1.write_all(b"# User Guide\n\n## Installation\n\nInstall the software.")
            .unwrap();

        let mut doc2 = File::create(sub_dir3.join("advanced.md")).unwrap();
        doc2.write_all(b"# Advanced Usage\n\nFor power users.")
            .unwrap();

        // Create non-markdown files (should be ignored)
        File::create(dir_path.join("config.toml")).unwrap();
        File::create(sub_dir2.join("main.rs")).unwrap();
        File::create(sub_dir1.join("image.png")).unwrap();

        // Test the full workflow
        let result = find_markdown_files(dir_path.to_str().unwrap());
        assert!(result.is_ok());
        let mut files = result.unwrap();

        // Should find exactly 3 markdown files
        assert_eq!(files.len(), 3);

        // Sort by name for predictable testing
        files.sort_by(|a, b| a.name.cmp(&b.name));

        // Verify file names (they will be full paths, so check endings)
        let names: Vec<&String> = files.iter().map(|f| &f.name).collect();
        assert!(names.iter().any(|name| name.ends_with("README.md")));
        assert!(names.iter().any(|name| name.ends_with("docs/guides/advanced.md")));
        assert!(names.iter().any(|name| name.ends_with("docs/guide.md")));

        // Test loading content for each file
        for file in &mut files {
            assert!(file.content.is_none());
            let load_result = file.load_content();
            assert!(load_result.is_ok());
            assert!(file.content.is_some());

            let content = file.content.as_ref().unwrap();
            assert!(!content.is_empty());
            assert!(content.starts_with('#')); // All our test files start with headers
        }

        // Verify specific content
        let readme_file = files.iter().find(|f| f.name.ends_with("README.md")).unwrap();
        assert!(readme_file
            .content
            .as_ref()
            .unwrap()
            .contains("test project"));

        let guide_file = files.iter().find(|f| f.name.ends_with("docs/guide.md")).unwrap();
        assert!(guide_file
            .content
            .as_ref()
            .unwrap()
            .contains("Installation"));

        let advanced_file = files.iter().find(|f| f.name.ends_with("docs/guides/advanced.md")).unwrap();
        assert!(advanced_file
            .content
            .as_ref()
            .unwrap()
            .contains("power users"));
    }

    #[test]
    fn test_find_all_markdown_files_unfiltered() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create directory structure including hidden dirs and ignored dirs
        let node_modules = dir_path.join("node_modules");
        let go_dir = dir_path.join("go");
        let hidden_dir = dir_path.join(".hidden");
        let normal_dir = dir_path.join("docs");

        fs::create_dir_all(&node_modules).unwrap();
        fs::create_dir_all(&go_dir).unwrap();
        fs::create_dir_all(&hidden_dir).unwrap();
        fs::create_dir_all(&normal_dir).unwrap();

        // Create markdown files in different directories
        File::create(dir_path.join("root.md")).unwrap();
        File::create(node_modules.join("package.md")).unwrap(); // Should be ignored
        File::create(go_dir.join("main.md")).unwrap(); // Should be ignored
        File::create(hidden_dir.join("secret.md")).unwrap(); // Should be included
        File::create(normal_dir.join("readme.md")).unwrap(); // Should be included

        let result = super::super::find_all_markdown_files_unfiltered(dir_path.to_str().unwrap());

        assert!(result.is_ok());
        let files = result.unwrap();

        // Should find ALL 5 files: root.md, secret.md (in hidden dir), readme.md, package.md (in node_modules), main.md (in go)
        assert_eq!(files.len(), 5);

        // Check that files are found using path endings since names will be full paths
        let names: Vec<&String> = files.iter().map(|f| &f.name).collect();
        assert!(names.iter().any(|name| name.ends_with("root.md")));
        assert!(names.iter().any(|name| name.ends_with(".hidden/secret.md")));
        assert!(names.iter().any(|name| name.ends_with("docs/readme.md")));
        assert!(names.iter().any(|name| name.ends_with("node_modules/package.md")));
        assert!(names.iter().any(|name| name.ends_with("go/main.md")));
    }

    #[test]
    fn test_find_all_vs_without_hidden_difference() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create both hidden and normal directories, plus an ignored directory
        let hidden_dir = dir_path.join(".hidden");
        let normal_dir = dir_path.join("docs");
        let node_modules = dir_path.join("node_modules");

        fs::create_dir_all(&hidden_dir).unwrap();
        fs::create_dir_all(&normal_dir).unwrap();
        fs::create_dir_all(&node_modules).unwrap();

        // Create markdown files
        File::create(dir_path.join("root.md")).unwrap();
        File::create(hidden_dir.join("secret.md")).unwrap();
        File::create(normal_dir.join("public.md")).unwrap();
        File::create(node_modules.join("package.md")).unwrap();

        let ignored_dirs: Vec<String> = vec!["node_modules".to_string()];

        // Test find_all_unfiltered - should include everything (hidden + ignored)
        let all_result =
            super::super::find_all_markdown_files_unfiltered(dir_path.to_str().unwrap());
        assert!(all_result.is_ok());
        let all_files = all_result.unwrap();
        assert_eq!(all_files.len(), 4); // Should find all 4 files including node_modules

        // Test without_hidden - should exclude hidden directories but still respect ignored_dirs
        let without_hidden_result = super::super::find_markdown_files_without_hidden_with_ignored(
            dir_path.to_str().unwrap(),
            &ignored_dirs,
        );
        assert!(without_hidden_result.is_ok());
        let without_hidden_files = without_hidden_result.unwrap();

        assert_eq!(without_hidden_files.len(), 2); // Should find only 2 files (not the ones in .hidden or node_modules)

        // Check that files are found using path endings since names will be full paths
        let names: Vec<&String> = without_hidden_files.iter().map(|f| &f.name).collect();
        assert!(!names.iter().any(|name| name.ends_with(".hidden/secret.md")));
        assert!(!names.iter().any(|name| name.ends_with("node_modules/package.md")));
        assert!(names.iter().any(|name| name.ends_with("root.md")));
        assert!(names.iter().any(|name| name.ends_with("docs/public.md")));
    }
}
