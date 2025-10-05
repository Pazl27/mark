# Getting Started

Welcome to Mark! This guide will help you get up and running with the markdown viewer in just a few minutes.

## First Steps

After [installing Mark](installation.md), let's start with the basics.

### Viewing a Single File

The simplest way to use Mark is to view a single markdown file:

```bash
mark README.md
```

This will open the file in Mark's terminal interface where you can:
- Scroll through the content with arrow keys
- Navigate sections with Page Up/Page Down
- Press `q` to quit

### Browsing a Directory

Mark can also browse through multiple markdown files in a directory:

```bash
mark ./docs/
```

This opens a file browser where you can:
- Use arrow keys to navigate files
- Press Enter to open a file
- Press Escape to go back to the file list

## Basic Navigation

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑k/↓j` | Scroll up/down |
| `gg/G` | Go to beginning/end |
| `q` or `Ctrl+C` | Quit |
| `?` | Show help |
| `/` | Search |

## Configuration

Mark uses a TOML configuration file for customization. Create a config file at:

- Linux/macOS: `~/.config/mark/config.toml`
- Windows: `%APPDATA%\mark\config.toml`

### Basic Configuration

```toml
// TODO
```
### Reading Documentation

```bash
# Browse project documentation
mark ./project-docs/

# Read a specific guide
mark ./project-docs/user-guide.md
```

### 2. Configuration Profiles

You can use different configurations for different contexts:

```bash
# Use work configuration
mark --config ~/.config/mark/config1.toml document.md

# Use presentation theme
mark --config ~/.config/mark/config2.toml slides.md
```

<!--### 3. Piping Content

Mark can read from stdin:

```bash
echo "# Hello World" | mark
curl -s https://raw.githubusercontent.com/user/repo/main/README.md | mark
```-->

## Common Use Cases

### Documentation Reading
Perfect for browsing project documentation, API references, and technical guides without leaving the terminal.

### Code Review
Quickly review README files, changelogs, and documentation changes during code reviews.

### Writing and Editing
Use Mark alongside your favorite editor to preview markdown as you write.

### Presentations
Use Mark's clean rendering for terminal-based presentations and demos.

## What's Next?

- Learn about [Configuration](configuration.md) options
- Explore [Usage Examples](usage.md) for advanced workflows
- Check out the [Developer Guide](../developer/contributing.md) to contribute

## Getting Help

If you need help:
- Press `?` in Mark to see keyboard shortcuts
- Visit the [GitHub repository](https://github.com/Pazl27/mark) for issues and discussions
