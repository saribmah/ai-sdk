# Development Guide

This guide covers setting up your development environment and using the development tools for the AI SDK Rust project.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setting Up Pre-Commit Hooks](#setting-up-pre-commit-hooks)
- [Development Workflow](#development-workflow)
- [Available Commands](#available-commands)
- [Pre-Commit Checks](#pre-commit-checks)

## Prerequisites

- Rust 1.75 or later
- Cargo
- Git
- Just (required) - See installation instructions below

### Installing Just

[Just](https://github.com/casey/just) is a command runner similar to make but better suited for modern development.

**Via Cargo:**
```bash
cargo install just
```

**Via Package Manager:**
```bash
# macOS
brew install just

# Linux (Ubuntu/Debian)
sudo apt install just

# Arch Linux
sudo pacman -S just

# Windows (Scoop)
scoop install just

# Windows (Chocolatey)
choco install just
```

For other installation methods, see the [official documentation](https://github.com/casey/just#installation).

## Setting Up Pre-Commit Hooks

We provide git pre-commit hooks to ensure code quality and catch issues before they're committed.

### Quick Setup (Recommended)

Run the following command to install the git hooks:

```bash
just install-hooks
```

This will install a pre-commit hook that automatically:
- ✅ Formats your code with `rustfmt` (auto-fixes)
- ✅ Runs `clippy` to catch common mistakes (blocks if issues found)
- ✅ Runs `cargo check` to ensure code compiles (blocks if fails)

### Manual Setup

If you don't have Just installed, you can manually install the hooks:

```bash
mkdir -p .git/hooks
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## Development Workflow

### Standard Workflow

1. Make your changes
2. Run checks locally:
   ```bash
   just pre-commit
   ```
3. Commit your changes:
   ```bash
   git add .
   git commit -m "Your commit message"
   ```
   The pre-commit hook will automatically run and:
   - Auto-format your code
   - Check for linting issues
   - Verify code compiles

### Bypassing Hooks (Not Recommended)

If you need to bypass the pre-commit hooks (e.g., for work-in-progress commits):

```bash
git commit --no-verify -m "WIP: your message"
```

**Note:** All commits will still be checked by CI, so issues will be caught before merging.

## Available Commands

We provide a Justfile with common development commands. Run `just` or `just --list` to see all available commands.

### Code Quality

```bash
# Format code (auto-fix)
just fmt

# Check formatting without fixing
just fmt-check

# Run clippy linter
just clippy

# Run clippy with auto-fix
just clippy-fix

# Run all pre-commit checks (fmt + clippy + test)
just pre-commit

# Run all CI checks locally
just ci
```

### Building and Testing

```bash
# Quick compile check
just check

# Run tests
just test

# Build all crates
just build

# Build in release mode
just build-release

# Build documentation
just doc

# Build and open documentation
just doc-open
```

### Examples

```bash
# List all available examples
just list-examples

# Run a specific example
just run-example basic_chat
```

### Cleanup and Maintenance

```bash
# Clean build artifacts
just clean

# Update dependencies
just update

# Check for outdated dependencies
just outdated

# Run security audit
just audit
```

### Development Helpers

```bash
# Watch for changes and run tests
just watch

# Generate code coverage
just coverage

# Run benchmarks
just bench
```

### Setup

```bash
# Install git hooks
just install-hooks

# Show all available commands
just --list
```

## Pre-Commit Checks

The pre-commit hook runs the following checks in order:

### 1. Code Formatting (Auto-Fix)

```bash
cargo fmt --all
```

- **Action:** Automatically formats code according to Rust style guidelines
- **Auto-fix:** Yes - formatted files are automatically staged
- **Blocks commit:** No

### 2. Clippy Linting

```bash
cargo clippy --all-targets --all-features --workspace -- -D warnings
```

- **Action:** Checks for common mistakes and style issues
- **Auto-fix:** Some issues can be fixed with `cargo clippy --fix`
- **Blocks commit:** Yes - if any warnings are found

Common clippy fixes:
```bash
# Try auto-fixing clippy issues
cargo clippy --fix --all-targets --all-features --workspace
```

### 3. Cargo Check

```bash
cargo check --all-features --workspace
```

- **Action:** Verifies that code compiles
- **Auto-fix:** No
- **Blocks commit:** Yes - if compilation fails

### 4. Tests (Optional)

Tests are commented out by default in the pre-commit hook for faster commits, but you can enable them by uncommenting the test section in `scripts/pre-commit.sh`.

To run tests manually:
```bash
make test
# or
cargo test --all-features --workspace
```

## CI/CD

All commits pushed to GitHub will be validated by our CI pipeline, which runs:

- Formatting check (`cargo fmt --check`)
- Clippy linting
- Full test suite on multiple platforms (Ubuntu, macOS, Windows)
- Documentation build
- Release builds

See `.github/workflows/ci.yml` for the complete CI configuration.

## Tips

1. **Run checks before committing:**
   ```bash
   just pre-commit
   ```
   This runs the same checks as the git hook without actually committing.

2. **Fix formatting issues quickly:**
   ```bash
   just fmt
   ```

3. **Check what would be committed:**
   ```bash
   git diff --cached
   ```

4. **Stash changes temporarily:**
   ```bash
   git stash
   # ... do something else ...
   git stash pop
   ```

## Troubleshooting

### Pre-commit hook not running

Make sure the hook is executable:
```bash
chmod +x .git/hooks/pre-commit
```

### Hook runs but doesn't block commits

Check that the hook script has `set -e` at the top, which causes it to exit on any error.

### Clippy errors are unclear

Run clippy with more verbose output:
```bash
cargo clippy --all-targets --all-features --workspace -- -D warnings --verbose
```

### Need to commit without running hooks

Use `--no-verify`, but remember that CI will still catch issues:
```bash
git commit --no-verify -m "your message"
```
