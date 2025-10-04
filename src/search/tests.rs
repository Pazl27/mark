use super::*;
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs::{self, File};
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_file_new() {
        let path = PathBuf::from("/path/to/test.md");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        assert_eq!(md_file.name, "test.md");
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_markdown_file_new_with_extension() {
        let path = PathBuf::from("/path/to/readme.markdown");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        assert_eq!(md_file.name, "readme.markdown");
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_markdown_file_new_no_extension() {
        let path = PathBuf::from("/path/to/README");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        assert_eq!(md_file.name, "README");
        assert_eq!(md_file.content, None);
    }

    #[test]
    fn test_markdown_file_new_invalid_unicode() {
        let path = PathBuf::from("/");
        let md_file = MarkdownFile::new(path.clone());

        assert_eq!(md_file.path, path);
        assert_eq!(md_file.name, "unknown");
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
        assert_eq!(files[2].name, "doc3.md");
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

        // Create test files
        File::create(dir_path.join("file1.md")).unwrap();
        File::create(dir_path.join("file2.md")).unwrap();
        File::create(dir_path.join("not_markdown.txt")).unwrap();
        File::create(dir_path.join("README")).unwrap();

        let result = find_markdown_files(dir_path.to_str().unwrap());
        assert!(result.is_ok());
        let files = result.unwrap();

        assert_eq!(files.len(), 2);
        let file_names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"file1.md".to_string()));
        assert!(file_names.contains(&"file2.md".to_string()));
    }

    #[test]
    fn test_find_markdown_files_with_subdirectories() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create subdirectory
        let sub_dir = dir_path.join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        // Create test files
        File::create(dir_path.join("root.md")).unwrap();
        File::create(sub_dir.join("nested.md")).unwrap();
        File::create(sub_dir.join("another.txt")).unwrap();

        let result = find_markdown_files(dir_path.to_str().unwrap());
        assert!(result.is_ok());
        let files = result.unwrap();

        assert_eq!(files.len(), 2);
        let file_names: Vec<String> = files.iter().map(|f| f.name.clone()).collect();
        assert!(file_names.contains(&"root.md".to_string()));
        assert!(file_names.contains(&"nested.md".to_string()));
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
        assert_eq!(files[0].name, "doc.md");
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

        for (i, file) in files.iter().enumerate() {
            assert_eq!(file.path, paths[i]);
            assert_eq!(file.name, paths[i].file_name().unwrap().to_str().unwrap());
            assert_eq!(file.content, None);
        }
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
            PathBuf::from(".md"),           // Just extension
            PathBuf::from("file.md.backup"), // Multiple dots
            PathBuf::from("a.very.long.filename.with.many.dots.md"),
        ];

        let files = convert_to_files(paths);

        assert_eq!(files[0].name, ".md");
        assert_eq!(files[1].name, "file.md.backup");
        assert_eq!(files[2].name, "a.very.long.filename.with.many.dots.md");
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
        readme.write_all(b"# Project\n\nThis is a test project.").unwrap();

        let mut doc1 = File::create(sub_dir1.join("guide.md")).unwrap();
        doc1.write_all(b"# User Guide\n\n## Installation\n\nInstall the software.").unwrap();

        let mut doc2 = File::create(sub_dir3.join("advanced.md")).unwrap();
        doc2.write_all(b"# Advanced Usage\n\nFor power users.").unwrap();

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

        // Verify file names
        assert_eq!(files[0].name, "README.md");
        assert_eq!(files[1].name, "advanced.md");
        assert_eq!(files[2].name, "guide.md");

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
        let readme_file = files.iter().find(|f| f.name == "README.md").unwrap();
        assert!(readme_file.content.as_ref().unwrap().contains("test project"));

        let guide_file = files.iter().find(|f| f.name == "guide.md").unwrap();
        assert!(guide_file.content.as_ref().unwrap().contains("Installation"));

        let advanced_file = files.iter().find(|f| f.name == "advanced.md").unwrap();
        assert!(advanced_file.content.as_ref().unwrap().contains("power users"));
    }
}
