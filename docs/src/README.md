# Mark - Markdown Viewer

Welcome to **Mark**, a fast and elegant terminal-based markdown viewer and renderer written in Rust.

## What is Mark?

Mark is a command-line utility that brings beautiful markdown rendering directly to your terminal. Whether you're reading documentation, reviewing code comments, or browsing through markdown files, Mark provides a smooth and feature-rich experience without leaving your terminal environment.

## Key Features

- **Fast Performance** - Built with Rust for lightning-fast rendering
- **Beautiful TUI** - Powered by Ratatui for a smooth terminal interface
- **Syntax Highlighting** - Full code block syntax highlighting
- **Multiple Themes** - Choose from various color schemes
- **File Navigation** - Easy browsing and searching of markdown files
- **Configurable** - Extensive customization via TOML configuration
- **Smart Search** - Find content across multiple files
- **Cross-platform** - Works on Linux, macOS, and Windows

## Quick Start

```bash
# View a single markdown file
mark README.md

# Browse markdown files in a directory
mark ./docs/

# Use with custom configuration
mark --config ~/.config/mark/config.toml README.md
```

## Architecture Overview

Mark is built with a modular architecture:

- **Lexer** - Tokenizes markdown syntax with comprehensive error handling
- **Parser** - Converts tokens into a structured document tree
- **Renderer** - Transforms the document into beautiful terminal output
- **TUI** - Provides interactive navigation and user interface
- **Configuration** - Handles themes, settings, and user preferences

## Getting Help

- Read the [User Guide](user-guide/installation.md) for detailed instructions
- Check out [Usage Examples](user-guide/usage.md) for common workflows
- See the [Developer Guide](developer/contributing.md) to contribute
- Report issues on [GitHub](https://github.com/Pazl27/mark)

Ready to get started? Head over to the [Installation](user-guide/installation.md) guide!
