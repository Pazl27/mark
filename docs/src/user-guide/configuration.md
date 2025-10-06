# Configuration

Mark is highly configurable through TOML configuration files. This guide covers all available configuration options and how to customize Mark to your preferences.

## Configuration File Location

Mark looks for its configuration file in the following locations:

- **Linux/macOS**: `~/.config/mark/config.toml`
- **Windows**: `%APPDATA%\mark\config.toml`

If no configuration file is found, Mark will offer to download the default configuration from the repository.

You can also find the complete default configuration file in this book: [config.toml](config.toml)

## Configuration Structure

The configuration file is written in TOML format and consists of two main sections:

1. **`[settings]`** - General application settings
2. **`[color]`** - Color themes for dark and light modes

## Settings Section

The `[settings]` section contains general application preferences:

```toml
[settings]
theme = "dark"
width = 80
syntax_highlighting = true
hidden_files = false
ignored_dirs = ["node_modules", "go", ".git", "target", "build", "dist", ".vscode", ".idea", ".next", "vendor", "__pycache__", ".pytest_cache", "coverage"]
```

### Available Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `theme` | String | `"dark"` | Color theme to use (`"dark"` or `"light"`) |
| `width` | Integer | `80` | Maximum display width in characters (20-200) |
| `syntax_highlighting` | Boolean | `true` | Enable syntax highlighting for code blocks |
| `hidden_files` | Boolean | `false` | Show hidden files and directories |
| `ignored_dirs` | Array | See default | Directories to ignore when browsing |

#### Theme

Controls the color scheme used by Mark. Must be either `"dark"` or `"light"`.

```toml
theme = "dark"    # Use dark theme
theme = "light"   # Use light theme
```

#### Width

Sets the maximum display width in characters. This affects text wrapping and layout.

- **Range**: 20-200 characters
- **Default**: 80 characters

```toml
width = 80    # Standard terminal width
width = 120   # Wide display
width = 60    # Narrow display
```

#### Syntax Highlighting

Enables or disables syntax highlighting for code blocks in markdown files.

```toml
syntax_highlighting = true   # Enable highlighting
syntax_highlighting = false  # Disable highlighting
```

#### Hidden Files

Controls whether hidden files (starting with `.`) are shown when browsing directories.

```toml
hidden_files = false  # Hide dotfiles
hidden_files = true   # Show dotfiles
```

#### Ignored Directories

An array of directory names to ignore when browsing. These directories won't appear in the file browser.

```toml
ignored_dirs = [
    "node_modules",     # npm packages
    "go",              # Go cache
    ".git",            # Git repository data
    "target",          # Rust build output
    "build",           # Build directories
    "dist",            # Distribution directories
    ".vscode",         # VS Code settings
    ".idea",           # IntelliJ settings
    ".next",           # Next.js cache
    "vendor",          # Dependency directories
    "__pycache__",     # Python cache
    ".pytest_cache",   # Pytest cache
    "coverage"         # Coverage reports
]
```

## Color Section

The `[color]` section defines color schemes for both dark and light themes. Colors must be specified in hexadecimal format (`#rrggbb`).

### Dark Theme Colors

```toml
[color.dark]
background = "#000000"
text = "#ffffff"
code_block = "#333333"
h1 = "#ff0000"
h2 = "#ff4444"
h3 = "#ff8888"
h4 = "#ffaaaa"
h5 = "#ffcccc"
h6 = "#ffeeee"
link = "#0000ff"
passive = "#888888"
```

### Light Theme Colors

```toml
[color.light]
background = "#ffffff"
text = "#000000"
code_block = "#f0f0f0"
h1 = "#cc0000"
h2 = "#aa0000"
h3 = "#880000"
h4 = "#660000"
h5 = "#440000"
h6 = "#220000"
link = "#0000cc"
passive = "#666666"
```

### Color Fields

Each theme (dark and light) supports the following color fields:

| Field | Description |
|-------|-------------|
| `background` | Main background color |
| `text` | Primary text color |
| `code_block` | Background color for code blocks |
| `h1` | Color for level 1 headings |
| `h2` | Color for level 2 headings |
| `h3` | Color for level 3 headings |
| `h4` | Color for level 4 headings |
| `h5` | Color for level 5 headings |
| `h6` | Color for level 6 headings |
| `link` | Color for hyperlinks |
| `passive` | Color for secondary/dimmed text |

All colors must be in hexadecimal format: `#rrggbb` where each component (red, green, blue) is a two-digit hexadecimal number (00-ff).

## Complete Default Configuration

Here's the complete default configuration file (also available as [config.toml](config.toml) in this book):

```toml
[settings]
theme = "dark"
width = 80
syntax_highlighting = true
hidden_files = false
ignored_dirs = ["node_modules", "go", ".git", "target", "build", "dist", ".vscode", ".idea", ".next", "vendor", "__pycache__", ".pytest_cache", "coverage"]

[color.dark]
background = "#000000"
text = "#ffffff"
code_block = "#333333"
h1 = "#ff0000"
h2 = "#ff4444"
h3 = "#ff8888"
h4 = "#ffaaaa"
h5 = "#ffcccc"
h6 = "#ffeeee"
link = "#0000ff"
passive = "#888888"

[color.light]
background = "#ffffff"
text = "#000000"
code_block = "#f0f0f0"
h1 = "#cc0000"
h2 = "#aa0000"
h3 = "#880000"
h4 = "#660000"
h5 = "#440000"
h6 = "#220000"
link = "#0000cc"
passive = "#666666"
```
## Validation and Error Handling

Mark validates the configuration file when loading and provides helpful error messages for common issues:

### Common Validation Errors

1. **Invalid theme**: Theme must be `"dark"` or `"light"`
3. **Invalid color format**: Colors must be in `#rrggbb` format
4. **Missing sections**: Both `[settings]` and `[color]` sections are required
5. **Missing fields**: All required fields must be present

### Error Messages

When Mark encounters a configuration error, it will display:

- The specific error and location
- The path to your configuration file
- A link to this documentation

## Creating Your Configuration

### Option 1: Download Default Configuration

When you first run Mark without a configuration file, it will offer to download the default configuration:

```
Configuration file not found: /home/user/.config/mark/config.toml

Would you like to download the default configuration? [Y/n]
```

Type `Y` or press Enter to download and use the default configuration.

### Option 2: Create Custom Configuration

1. Create the configuration directory:
   ```bash
   mkdir -p ~/.config/mark
   ```

2. Create and edit the configuration file:
   ```bash
   nano ~/.config/mark/config.toml
   ```

3. Add your preferred settings using the examples above as a starting point, or copy from the [default config file](config.toml) in this book.

### Option 3: Copy and Modify Default

1. Copy the [default configuration file](config.toml) from this book, or download it from the repository
2. Save it to your configuration directory (`~/.config/mark/config.toml`)
3. Modify the settings to your preferences

## Command Line Override

You can specify a custom configuration file path using the `--config` flag:

```bash
mark --config /path/to/custom/config.toml document.md
```

This is useful for:
- Testing different configurations
- Project-specific settings
- Temporary configuration changes

## Troubleshooting

### Configuration Not Loading

If your configuration isn't being applied:

1. Check the file path is correct
2. Verify the TOML syntax is valid
3. Ensure all required sections and fields are present
4. Check file permissions

### Invalid Colors

Color validation errors are common. Remember:

- Colors must start with `#`
- Must be exactly 7 characters (`#rrggbb`)
- Use only hexadecimal digits (0-9, a-f, A-F)
