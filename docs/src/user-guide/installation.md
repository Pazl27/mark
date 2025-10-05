# Installation

This guide will walk you through installing Mark on your system.

## Prerequisites

Before installing Mark, ensure you have the following:

- **Rust** (1.70 or later) - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository
- A modern terminal with UTF-8 support

## Installation Methods

### From Source (Recommended)

The easiest way to install Mark is directly from the source code:

```bash
# Clone the repository
git clone https://github.com/Pazl27/mark.git
cd mark

# Build and install
cargo install --path .
```

### Using Cargo Install (Future)

Once published to crates.io, you'll be able to install with:

```bash
cargo install mark
```

### Pre-built Binaries (Future)

Pre-built binaries will be available for download from the GitHub releases page for:
- Linux (x86_64, ARM64)

## Verify Installation

After installation, verify that Mark is working correctly:

```bash
# Check version
mark --version

# View help
mark --help

# Test with a sample file
echo "# Hello World\n\nThis is a test file." > test.md
mark test.md
```

## System Requirements

- **Memory**: Minimum 256MB RAM
- **Terminal**: Any modern terminal emulator
- **Colors**: 256-color support recommended for best experience

## Troubleshooting

### Common Issues

**Rust not found**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Compilation errors**
```bash
# Update Rust to latest version
rustup update
```

**Permission denied (Unix systems)**
```bash
# Install to user directory instead
cargo install --path . --root ~/.local
export PATH="$HOME/.local/bin:$PATH"
```

### Getting Help

If you encounter issues:
1. Search existing [GitHub issues](https://github.com/Pazl27/mark/issues)
2. Create a new issue with your system details

## Next Steps

Once installed, head over to [Getting Started](getting-started.md) to learn how to use Mark!
