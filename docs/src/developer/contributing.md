# Contributing to Mark

Thank you for your interest in contributing to Mark! This guide will help you get started with contributing to the project, whether you're fixing bugs, adding features, or improving documentation.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.70 or later) - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For version control
- **A good text editor** - VS Code, Vim, Emacs, etc.

### Setting Up the Development Environment

1. **Fork and Clone**
   ```bash
   # Fork the repository on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/mark.git
   cd mark
   ```

2. **Set up the upstream remote**
   ```bash
   git remote add upstream https://github.com/Pazl27/mark.git
   ```

3. **Install dependencies and build**
   ```bash
   cargo build
   cargo test
   ```

4. **Verify everything works**
   ```bash
   cargo run -- README.md
   ```

## Development Workflow

### Branch Management

We use a GitHub Flow approach:

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

2. **Keep your branch up to date**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

3. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   # Then create a Pull Request on GitHub
   ```

### Code Style and Standards

#### Rust Code Style

We follow standard Rust conventions:

- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common issues
- Follow Rust naming conventions (snake_case, etc.)
- Write descriptive comments for complex logic
- Include unit tests for new functionality

#### Code Organization

The project is organized as follows:

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── cli/                 # Command-line interface
├── config/              # Configuration handling
├── markdown_parser/     # Markdown parsing logic
│   ├── lexer/          # Tokenization
│   ├── parser/         # AST generation
│   └── renderer/       # Output generation
├── tui/                # Terminal UI components
├── theme/              # Theme system
├── search/             # Search functionality
└── error.rs            # Error types and handling
```

#### Testing

- Write unit tests for all new functions
- Add integration tests for new features
- Include edge case tests
- Use descriptive test names

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_handles_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_parser_processes_headers() {
        // Test implementation
    }
}
```

## Types of Contributions

### Bug Reports

When reporting bugs, please include:

- **Clear description** of the issue
- **Steps to reproduce** the problem
- **Expected vs actual behavior**
- **System information** (OS, terminal, Rust version)
- **Sample files** that trigger the issue (if applicable)

Use this template:

```markdown
## Bug Description
Brief description of what's wrong.

## Steps to Reproduce
1. Run `mark file.md`
2. Press key X
3. See error Y

## Expected Behavior
What should happen.

## Actual Behavior
What actually happens.

## Environment
- OS: Linux/macOS/Windows
- Terminal: Terminal app name and version
- Mark version: `mark --version`
- Rust version: `rustc --version`

## Additional Context
Any other relevant information.
```

### Feature Requests

For new features, please:

- **Search existing issues** to avoid duplicates
- **Describe the use case** clearly
- **Explain why** this feature would be valuable
- **Consider alternatives** and their trade-offs
- **Be open to discussion** about implementation

### Code Contributions

#### Small Changes

For small changes (typos, minor bug fixes):

1. Create a branch
2. Make your changes
3. Test locally
4. Submit a pull request

#### Large Changes

For significant changes:

1. **Open an issue first** to discuss the approach
2. **Get feedback** from maintainers
3. **Break down** the work into smaller PRs if possible
4. **Follow the development workflow**

#### Pull Request Guidelines

- **Write clear commit messages**
  ```
  feat: add syntax highlighting for code blocks
  
  - Implement highlighting using syntect crate
  - Add configuration options for themes
  - Include tests for various languages
  
  Closes #123
  ```

- **Keep PRs focused** - one feature/fix per PR
- **Include tests** for new functionality
- **Update documentation** as needed
- **Ensure CI passes** before requesting review

## Development Tasks

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test lexer

# Run tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

### Code Quality Checks

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Check for unused dependencies
cargo machete
```

### Building Documentation

```bash
# Build API documentation
cargo doc --open

# Build user guide
cd docs
mdbook serve
```

### Performance Testing

```bash
# Build optimized version
cargo build --release

# Profile with large files
cargo run --release -- large-file.md

# Benchmark lexer performance
cargo bench lexer
```

## Architecture Overview

### Core Components

1. **Lexer** (`src/markdown_parser/lexer/`)
   - Tokenizes markdown text
   - Handles all markdown syntax elements
   - Provides comprehensive error reporting

2. **Parser** (`src/markdown_parser/parser/`)
   - Converts tokens to Abstract Syntax Tree (AST)
   - Handles markdown semantics
   - Resolves references and links

3. **Renderer** (`src/markdown_parser/renderer/`)
   - Converts AST to terminal output
   - Handles styling and theming
   - Manages terminal-specific formatting

4. **TUI** (`src/tui/`)
   - Interactive terminal interface
   - Navigation and input handling
   - File browser and search

5. **Configuration** (`src/config/`)
   - TOML configuration parsing
   - Theme management
   - User preferences

### Design Principles

- **Performance**: Fast startup and rendering
- **Reliability**: Handle edge cases gracefully
- **Usability**: Intuitive interface and helpful errors
- **Extensibility**: Plugin system for custom features
- **Portability**: Cross-platform compatibility

## Common Development Tasks

### Adding a New Token Type

1. Add token variant to `src/markdown_parser/lexer/tokens.rs`
2. Update lexer logic in `src/markdown_parser/lexer/lexer.rs`
3. Add parser support in `src/markdown_parser/parser/`
4. Update renderer in `src/markdown_parser/renderer/`
5. Add comprehensive tests

### Adding a Configuration Option

1. Update config struct in `src/config/mod.rs`
2. Add default value and validation
3. Update TOML parsing logic
4. Add CLI flag if needed
5. Update documentation

### Adding a New Theme

1. Create theme file in `src/theme/`
2. Register theme in theme system
3. Add color definitions
4. Test with various content types
5. Update documentation

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite
4. Build documentation
5. Create git tag
6. Publish to crates.io (maintainers only)

## Getting Help

### Community Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Discord** (if available): Real-time chat

### Documentation

- **User Guide**: How to use Mark
- **API Documentation**: Code-level documentation
- **Architecture Guide**: High-level design decisions

### Mentorship

New contributors are always welcome! If you're new to:

- **Rust**: Check out the [Rust Book](https://doc.rust-lang.org/book/)
- **Terminal apps**: Look at similar projects like `bat`, `fd`, `ripgrep`
- **Open source**: Read the [Open Source Guide](https://opensource.guide/)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- **Be respectful** in all interactions
- **Be constructive** in feedback and criticism
- **Be patient** with new contributors
- **Follow** our [Code of Conduct](../CODE_OF_CONDUCT.md)

## Recognition

Contributors are recognized in several ways:

- **Changelog**: Major contributions mentioned in releases
- **README**: Contributors section
- **All-Contributors**: Recognizes all types of contributions

## Thank You!

Every contribution, no matter how small, helps make Mark better. Whether you're:

- Reporting bugs
- Fixing typos
- Adding features
- Improving documentation
- Helping other users

Your efforts are appreciated and help build a better tool for everyone.
