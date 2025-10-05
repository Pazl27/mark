# Architecture Overview

This document provides a comprehensive overview of Mark's architecture, design decisions, and implementation details. Understanding this architecture will help you contribute effectively to the project.

## High-Level Architecture

Mark follows a modular, pipeline-based architecture that processes markdown content through several distinct stages:

```
Input → Lexer → Parser → AST → Renderer → TUI → Output
```

### Core Pipeline

1. **Input Processing**: Raw markdown text or file paths
2. **Lexical Analysis**: Tokenization of markdown syntax
3. **Parsing**: Construction of Abstract Syntax Tree (AST)
4. **Rendering**: Conversion to terminal-friendly format
5. **Display**: Interactive terminal user interface
6. **Output**: Rendered content with navigation

## Module Structure

### Project Layout

```
src/
├── main.rs                 # CLI entry point and argument parsing
├── lib.rs                  # Public API and module exports
├── cli/                    # Command-line interface logic
│   ├── mod.rs             # CLI module definition
│   ├── args.rs            # Argument parsing and validation
│   └── commands.rs        # Command implementations
├── config/                 # Configuration system
│   ├── mod.rs             # Configuration management
│   ├── loader.rs          # Config file loading and parsing
│   ├── theme.rs           # Theme configuration
│   └── validation.rs      # Config validation logic
├── markdown_parser/        # Core markdown processing
│   ├── mod.rs             # Parser module exports
│   ├── lexer/             # Lexical analysis
│   │   ├── mod.rs         # Lexer module definition
│   │   ├── lexer.rs       # Main lexer implementation
│   │   └── tokens.rs      # Token definitions
│   ├── parser/            # Syntax analysis (future)
│   │   ├── mod.rs         # Parser implementation
│   │   ├── ast.rs         # AST node definitions
│   │   └── builder.rs     # AST construction logic
│   └── renderer/          # Output generation (future)
│       ├── mod.rs         # Renderer implementations
│       ├── terminal.rs    # Terminal-specific rendering
│       └── html.rs        # HTML export functionality
├── tui/                   # Terminal user interface (future)
│   ├── mod.rs             # TUI module definition
│   ├── app.rs             # Main application state
│   ├── events.rs          # Event handling
│   └── widgets/           # Custom UI widgets
├── theme/                 # Theme system (future)
│   ├── mod.rs             # Theme management
│   ├── colors.rs          # Color definitions
│   └── builtin.rs         # Built-in themes
├── search/                # Search functionality (future)
│   ├── mod.rs             # Search implementation
│   └── index.rs           # Search indexing
└── error.rs               # Error types and handling
```
