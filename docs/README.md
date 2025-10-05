# Mark Documentation

This directory contains the documentation for Mark, built using [mdBook](https://rust-lang.github.io/mdBook/).

## Quick Start

### Prerequisites

- [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html) - Install with `cargo install mdbook`

### Building the Documentation

```bash
# Build the documentation
./scripts/docs.sh build

# Serve with live reload (recommended for development)
./scripts/docs.sh serve

# Clean build artifacts
./scripts/docs.sh clean
```

### Viewing the Documentation

After building, the documentation will be available at:
- **Local development**: http://localhost:3000 (when using `serve`)
- **Static files**: `docs/book/index.html` (after `build`)

## Documentation Structure

```
docs/
├── book.toml           # mdBook configuration
├── src/                # Source markdown files
│   ├── SUMMARY.md      # Table of contents
│   ├── README.md       # Introduction
│   ├── user-guide/     # User documentation
│   ├── developer/      # Developer documentation
└── book/               # Generated output (ignored by git)
```

## Writing Documentation

### Adding New Pages

1. Create a new `.md` file in the appropriate directory under `src/`
2. Add the page to `src/SUMMARY.md` to include it in the navigation
3. Use relative links to reference other pages: `[link text](relative/path.md)`

### Markdown Features

The documentation supports:
- Standard CommonMark syntax
- GitHub Flavored Markdown
- Syntax highlighting for code blocks
- Math expressions (with MathJax)
- Cross-references between pages

### Code Examples

Use fenced code blocks with language specification:

```rust
// Rust code example
fn main() {
    println!("Hello, world!");
}
```

```bash
# Shell commands
cargo build --release
```

### Links and References

- **Internal links**: `[Getting Started](user-guide/getting-started.md)`
- **External links**: `[Rust Book](https://doc.rust-lang.org/book/)`
- **Repository links**: Use the GitHub repository URL in `book.toml`

## Configuration

The documentation configuration is in `book.toml`:

- **Title and metadata**: Project information
- **Build settings**: Output directory and options
- **HTML output**: Theme, search, and UI customization
- **Preprocessors**: Additional markdown processing

## Development Workflow

### Writing Process

1. **Edit**: Modify markdown files in `src/`
2. **Preview**: Run `./scripts/docs.sh serve` for live reload
3. **Review**: Check formatting and links
4. **Build**: Run `./scripts/docs.sh build` for final build
5. **Commit**: Add changes to git

### Best Practices

- **Clear headings**: Use descriptive section titles
- **Code examples**: Include practical examples
- **Links**: Verify all internal and external links work
- **Images**: Store images in `src/images/` and use relative paths
- **Consistency**: Follow the established style and structure

## Troubleshooting

### Common Issues

**mdBook not found**
```bash
cargo install mdbook
```

**Build fails**
- Check `book.toml` syntax
- Verify all linked files exist
- Check for broken internal links

**Serving issues**
- Ensure port 3000 is available
- Check firewall settings
- Try a different port: `mdbook serve --port 3001`

### Getting Help

- Check the [mdBook documentation](https://rust-lang.github.io/mdBook/)
- Review existing documentation structure for examples
- Open an issue on the [GitHub repository](https://github.com/Pazl27/mark)

## Contributing

When contributing to the documentation:

1. **Follow the existing structure** and style
2. **Test your changes** by building locally
3. **Check all links** work correctly
4. **Include examples** where helpful
5. **Update the changelog** for significant changes

## Scripts

The `scripts/docs.sh` script provides convenient commands:

```bash
./scripts/docs.sh build     # Build documentation
./scripts/docs.sh serve     # Serve with live reload
./scripts/docs.sh clean     # Clean build artifacts
./scripts/docs.sh watch     # Watch for changes
./scripts/docs.sh validate  # Validate links and code
./scripts/docs.sh help      # Show all commands
```

## Deployment

The documentation can be deployed to:
- **GitHub Pages**: Automatic deployment via GitHub Actions
- **Static hosting**: Upload `book/` directory contents
- **Local server**: Serve `book/index.html` with any web server

---

For more information about the Mark project, see the main [README](../README.md).
