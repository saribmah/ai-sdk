# Contributing to AI SDK Rust

Thank you for considering contributing to AI SDK Rust! This document provides guidelines and instructions for contributing.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ai-sdk.git
   cd ai-sdk
   ```
3. **Install Just** (if not already installed):
   ```bash
   cargo install just
   ```

4. **Install pre-commit hooks**:
   ```bash
   just install-hooks
   ```

## Development Workflow

### Making Changes

1. **Create a new branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards below

3. **Test your changes**:
   ```bash
   just pre-commit  # Runs fmt, clippy, and tests
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: your descriptive commit message"
   ```
   
   The pre-commit hook will automatically:
   - Format your code with `rustfmt`
   - Run `clippy` to catch issues
   - Verify code compiles

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub

## Coding Standards

### Code Formatting

- Use `rustfmt` for all code formatting
- Run `just fmt` or `cargo fmt --all` before committing
- The pre-commit hook will auto-format code for you

### Linting

- Fix all `clippy` warnings before submitting
- Run `just clippy` to check for issues
- Clippy runs with `-D warnings` (treats warnings as errors)
- Use `just clippy-fix` to auto-fix some issues

### Testing

- Write tests for new functionality
- Ensure all existing tests pass: `just test`
- Add integration tests in the examples directory when appropriate

### Documentation

- Document all public APIs with doc comments
- Include examples in doc comments when helpful
- Update relevant documentation files (README.md, DEVELOPMENT.md, etc.)
- Run `just doc` to ensure documentation builds without warnings

## Commit Message Format

We follow conventional commit format:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:
```
feat: add support for Azure OpenAI streaming
fix: handle empty tool call responses correctly
docs: update README with streaming examples
```

## Pre-Commit Hooks

Pre-commit hooks are automatically installed via `just install-hooks`. They will:

1. **Auto-format code** with rustfmt
2. **Run clippy** and block commits with warnings
3. **Verify compilation** with cargo check

To bypass hooks (not recommended):
```bash
git commit --no-verify
```

Note: CI will still run all checks, so issues will be caught before merge.

## Pull Request Process

1. **Update documentation** if you've changed APIs
2. **Add tests** for new functionality
3. **Ensure CI passes** - all tests and checks must pass
4. **Keep PRs focused** - one feature or fix per PR
5. **Respond to feedback** - address review comments promptly

## CI/CD

Our CI runs on every push and PR:

- ✅ Code formatting check
- ✅ Clippy linting
- ✅ Full test suite on Linux, macOS, and Windows
- ✅ Documentation build
- ✅ Release build verification

## Project Structure

- `ai-sdk-core/` - Core functionality (generate_text, prompts, messages)
- `ai-sdk-provider/` - Provider traits and interfaces
- `ai-sdk-openai-compatible/` - OpenAI-compatible provider implementation
- `ai-sdk-provider-utils/` - Shared utilities for AI SDK providers
- `examples/` - Example applications and integration tests

## Need Help?

- Check the [DEVELOPMENT.md](../../DEVELOPMENT.md) for detailed development setup
- Review existing code for patterns and conventions
- Open an issue for questions or discussions

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the project and community

Thank you for contributing! 🎉
