#!/bin/bash

# Documentation build and serve script for Mark project
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if mdbook is installed
check_mdbook() {
    if ! command -v mdbook &> /dev/null; then
        print_error "mdbook is not installed"
        print_info "Installing mdbook..."
        cargo install mdbook
        print_success "mdbook installed successfully"
    else
        print_info "mdbook is already installed ($(mdbook --version))"
    fi
}

# Function to build documentation
build_docs() {
    print_info "Building documentation..."

    # Check if docs directory exists
    if [ ! -d "docs" ]; then
        print_error "docs directory not found"
        exit 1
    fi

    cd docs

    # Build the book
    if mdbook build; then
        print_success "Documentation built successfully"
        print_info "Documentation available at: docs/book/index.html"
    else
        print_error "Failed to build documentation"
        exit 1
    fi

    cd ..
}

# Function to serve documentation
serve_docs() {
    print_info "Starting documentation server..."

    # Check if docs directory exists
    if [ ! -d "docs" ]; then
        print_error "docs directory not found"
        exit 1
    fi

    cd docs

    print_info "Documentation will be available at: http://localhost:3000"
    print_info "Press Ctrl+C to stop the server"

    # Serve with live reload
    mdbook serve --hostname 0.0.0.0 --port 3000
}

# Function to clean documentation builds
clean_docs() {
    print_info "Cleaning documentation builds..."

    if [ -d "docs/book" ]; then
        rm -rf docs/book
        print_success "Removed docs/book/"
    fi

    if [ -d "target/doc-book" ]; then
        rm -rf target/doc-book
        print_success "Removed target/doc-book/"
    fi

    print_success "Documentation builds cleaned"
}

# Function to initialize mdbook (for new projects)
init_docs() {
    print_info "Initializing mdbook documentation..."

    if [ -d "docs" ]; then
        print_warning "docs directory already exists"
        read -p "Do you want to reinitialize? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Initialization cancelled"
            exit 0
        fi
        rm -rf docs
    fi

    mdbook init docs --title "Mark - Markdown Viewer"
    print_success "Documentation initialized in docs/"
    print_info "Edit docs/src/SUMMARY.md to customize the book structure"
}

# Function to watch for changes and rebuild
watch_docs() {
    print_info "Watching documentation for changes..."

    # Check if docs directory exists
    if [ ! -d "docs" ]; then
        print_error "docs directory not found"
        exit 1
    fi

    cd docs

    print_info "Watching for changes... Press Ctrl+C to stop"
    mdbook watch
}

# Function to validate documentation
validate_docs() {
    print_info "Validating documentation..."

    # Check if docs directory exists
    if [ ! -d "docs" ]; then
        print_error "docs directory not found"
        exit 1
    fi

    cd docs

    # Test build
    if mdbook test; then
        print_success "Documentation validation passed"
    else
        print_error "Documentation validation failed"
        exit 1
    fi

    cd ..
}

# Function to show help
show_help() {
    echo "Documentation script for Mark project"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  build     Build the documentation"
    echo "  serve     Serve documentation with live reload (default)"
    echo "  clean     Clean documentation build artifacts"
    echo "  init      Initialize mdbook documentation"
    echo "  watch     Watch for changes and rebuild"
    echo "  validate  Validate documentation links and code"
    echo "  help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0              # Serve documentation (default)"
    echo "  $0 build        # Build documentation"
    echo "  $0 serve        # Serve with live reload"
    echo "  $0 clean        # Clean build artifacts"
    echo ""
}

# Main script logic
main() {
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ]; then
        print_error "This script must be run from the project root directory"
        exit 1
    fi

    # Check for mdbook installation
    check_mdbook

    # Parse command line arguments
    case "${1:-serve}" in
        "build")
            build_docs
            ;;
        "serve")
            serve_docs
            ;;
        "clean")
            clean_docs
            ;;
        "init")
            init_docs
            ;;
        "watch")
            watch_docs
            ;;
        "validate")
            validate_docs
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            print_error "Unknown command: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
