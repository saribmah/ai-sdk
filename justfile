# LLM Kit - Justfile
# Just is a command runner - see https://github.com/casey/just

# Default recipe - show help
default:
    @just --list

# Install git pre-commit hooks
install-hooks:
    @echo "Installing git pre-commit hooks..."
    @mkdir -p .git/hooks
    @cp scripts/pre-commit.sh .git/hooks/pre-commit
    @chmod +x .git/hooks/pre-commit
    @echo "✓ Git hooks installed successfully!"
    @echo "  To skip hooks on commit, use: git commit --no-verify"

# Format code with rustfmt (auto-fix)
fmt:
    @echo "Formatting code..."
    @cargo fmt --all
    @echo "✓ Code formatted"

# Run quick compile check
check:
    @echo "Running cargo check..."
    @cargo check --all-features --workspace
    @echo "✓ Check passed"

# Run clippy lints
clippy:
    @echo "Running clippy..."
    @cargo clippy --all-targets --all-features --workspace -- -D warnings
    @echo "✓ Clippy passed"

# Run all tests
test:
    @echo "Running tests..."
    @cargo test --all-features --workspace
    @echo "✓ Tests passed"

# Build all crates
build:
    @echo "Building workspace..."
    @cargo build --all-features --workspace
    @echo "✓ Build complete"

# Build in release mode
build-release:
    @echo "Building workspace (release)..."
    @cargo build --all-features --workspace --release
    @echo "✓ Release build complete"

# Build documentation
doc:
    @echo "Building documentation..."
    @RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --workspace
    @echo "✓ Documentation built"
    @echo "  Open: target/doc/llm_kit_core/index.html"

# Open documentation in browser
doc-open:
    @echo "Building and opening documentation..."
    @RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --workspace --open

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    @cargo clean
    @echo "✓ Clean complete"

# Run all pre-commit checks (fmt, clippy, test)
pre-commit: fmt clippy test
    @echo "✓ All pre-commit checks passed!"

# Run CI checks locally (what GitHub Actions will run)
ci: fmt-check clippy test doc
    @echo "✓ All CI checks passed!"

# Check code formatting (without fixing)
fmt-check:
    @echo "Checking code formatting..."
    @cargo fmt --all -- --check
    @echo "✓ Code is properly formatted"

# Run clippy with auto-fix where possible
clippy-fix:
    @echo "Running clippy with auto-fix..."
    @cargo clippy --fix --all-targets --all-features --workspace --allow-dirty --allow-staged
    @echo "✓ Clippy auto-fix complete"

# Run a specific example
run-example EXAMPLE:
    @echo "Running example: {{EXAMPLE}}"
    @cargo run --example {{EXAMPLE}}

# List all available examples
list-examples:
    @echo "Available examples:"
    @ls examples/*.rs | xargs -n1 basename | sed 's/.rs$//' | sed 's/^/  - /'

# Watch and run tests on file changes (requires cargo-watch)
watch:
    @echo "Watching for changes..."
    @cargo watch -x test

# Update dependencies
update:
    @echo "Updating dependencies..."
    @cargo update
    @echo "✓ Dependencies updated"

# Check for outdated dependencies (requires cargo-outdated)
outdated:
    @cargo outdated

# Run security audit (requires cargo-audit)
audit:
    @echo "Running security audit..."
    @cargo audit

# Generate code coverage report (requires cargo-tarpaulin)
coverage:
    @echo "Generating coverage report..."
    @cargo tarpaulin --out Html --output-dir coverage
    @echo "✓ Coverage report generated in coverage/"

# Benchmark (if benchmarks exist)
bench:
    @echo "Running benchmarks..."
    @cargo bench

# Full check - everything CI does plus extras
full-check: ci audit
    @echo "✓ Full check complete!"
