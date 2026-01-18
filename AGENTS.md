# AGENTS.md

Guidance for agentic coding agents working in this repository.

## Scope
- Applies to the entire workspace rooted at this file.
- Follow instructions in more specific AGENTS.md files (if any) under subdirectories.
- Refer to `DEVELOPMENT.md` and `CONTRIBUTING.md` for deeper context.

## Core Tooling
- Command runner: `just` (see `justfile`).
- Rust toolchain: Rust 1.75+, Cargo.
- Formatting: `rustfmt` (no custom `rustfmt.toml`).
- Linting: `clippy` with `-D warnings`.

## Build / Lint / Test Commands
- List all `just` recipes: `just --list`
- Format (auto-fix): `just fmt` or `cargo fmt --all`
- Format check only: `just fmt-check` or `cargo fmt --all -- --check`
- Lint: `just clippy` or `cargo clippy --all-targets --all-features --workspace -- -D warnings`
- Lint auto-fix: `just clippy-fix`
- Compile check: `just check` or `cargo check --all-features --workspace`
- Test all: `just test` or `cargo test --all-features --workspace`
- Build all crates: `just build`
- Build release: `just build-release`
- Docs: `just doc` (strict `RUSTDOCFLAGS="-D warnings"`)
- CI-like run: `just ci` (fmt-check + clippy + test + doc)
- Pre-commit checks: `just pre-commit` (fmt + clippy + test)

## Running a Single Test
- By name in a crate: `cargo test -p llm-kit-core my_test_name`
- Fully qualified path: `cargo test -p llm-kit-core module::tests::my_test`
- Single integration test file: `cargo test -p llm-kit-core --test integration_name`
- Single doc test: `cargo test -p llm-kit-core --doc item_path`
- Single example (as test): `cargo test -p llm-kit-core --example example_name`

## Examples / Local Runs
- List examples: `just list-examples`
- Run an example: `just run-example basic_chat`
- Watch tests: `just watch` (requires `cargo-watch`)

## Targeted Workspace Commands
- Build a single crate: `cargo build -p llm-kit-openai`
- Check a single crate: `cargo check -p llm-kit-openai`
- Test a single crate: `cargo test -p llm-kit-openai`
- Lint a single crate: `cargo clippy -p llm-kit-openai --all-targets --all-features -- -D warnings`
- Run tests with output: `cargo test -p llm-kit-core my_test -- --nocapture`
- Open docs locally: `just doc-open`

## Workspace Practices
- Keep changes scoped; avoid unrelated refactors.
- Update examples when adding new capabilities.
- Prefer reusing shared provider utils and core abstractions.
- Keep provider-specific constants in the provider crate.
- Run `just pre-commit` before committing changes.
- Run `just ci` before large PRs or releases.
- Use `just clean` if build artifacts are stale.
- Use `just update` for dependency bumps.

## Code Style Guidelines
- Follow rustfmt output; do not hand-align or override formatting.
- Keep public APIs documented; crates use `#![warn(missing_docs)]`.
- Use builder patterns consistently (`with_*`, `set_*`, fluent chaining) matching existing APIs.
- Prefer explicit types for public structs/enums; avoid type aliases that obscure meaning.
- Prefer `Arc<dyn Trait>` for shared trait objects in async contexts.
- Use `serde`/`serde_json` for JSON structures; prefer `json!` for literals.
- Avoid magic strings; centralize provider names and model IDs when possible.

## Imports and Modules
- Order imports in groups: standard library, external crates, then local crate modules.
- Separate import groups with a blank line.
- Avoid glob imports except for explicit preludes or test modules.
- Re-export public types in `lib.rs` alongside module docs (see existing crates).
- Keep `mod` declarations near the top of files with doc comments for public modules.

## Naming Conventions
- Types and traits: `CamelCase`.
- Functions and variables: `snake_case`.
- Modules/files: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Features: `kebab-case` in `Cargo.toml`, `snake_case` in `cfg(feature = "...")`.

## API Design Patterns
- Provide `new()` constructors and builder-style setters for configuration types.
- Name settings structs `*Settings` and options structs `*Options` for consistency.
- Prefer `Default` implementations for configuration structs.
- Use enums for known model IDs (`*ModelId`) and fall back to strings where needed.
- Keep provider-specific options scoped under `provider_options` fields.

## Error Handling
- Prefer `Result<T, E>`; avoid `unwrap`/`expect` in production code.
- Use crate error types (`AISDKError`, `ProviderError`, etc.) or boxed errors where required.
- Map provider-specific errors into shared error enums when crossing crate boundaries.
- Use `?` to propagate errors; add context only when it improves debugging.
- Use `log::warn!` for non-fatal issues (see `generate_text`/`stream_text`).

## Async / Concurrency
- Use `async`/`await` with Tokio; avoid blocking calls in async paths.
- Prefer `CancellationToken` for abortable operations when available.
- Keep async trait impls behind `#[async_trait]` as in provider traits.
- Use `Arc` for shared state across async tasks; avoid `Rc` in async code.

## Serialization & HTTP
- Follow existing serde attributes and field naming conventions in each crate.
- Keep provider request/response types close to their API modules.
- Preserve wire formats; avoid lossy conversions when mapping to SDK types.

## Testing
- Use `#[tokio::test]` for async tests; keep tests deterministic and isolated.
- Add unit tests alongside modules; add integration tests under `examples/` when appropriate.
- Avoid network calls in tests unless explicitly required; use mocks or fixtures.
- Name tests after behavior, not implementation details.

## Documentation
- Public items require doc comments and examples; use `no_run` where needed.
- Update relevant docs when changing APIs: `README.md`, `DEVELOPMENT.md`, crate docs.
- Keep module-level docs at top of `lib.rs` files.
- Document provider-specific behavior in crate-level docs.

## Logging and Diagnostics
- Use the `log` crate for warnings and non-fatal errors.
- Include enough context in log messages to debug provider issues.
- Avoid logging secrets or API keys.

## Safety, Performance, and Clarity
- Avoid unnecessary clones; pass references when ownership is not required.
- Use `Vec::with_capacity` when size is known and performance matters.
- Keep hot-path allocations minimal, but prefer clarity over micro-optimizations.
- Do not introduce unsafe code without strong justification.

## Security and Secrets
- Never commit API keys or credentials.
- Use environment variables (e.g., `OPENAI_API_KEY`) for secrets.
- Avoid writing secrets to logs or test snapshots.

## Project Structure Notes
- Workspace is multi-crate; core crates include `llm-kit-core`, `llm-kit-provider`, and provider crates.
- Follow existing module layout and keep changes scoped to the target crate.
- Use feature flags for optional storage or provider-specific functionality.

## Hooks and CI
- Pre-commit hooks run `rustfmt`, `clippy`, and `cargo check`; tests are optional in hook.
- CI runs formatting checks, clippy, full test suite, docs, and release builds.

## Cursor / Copilot Rules
- No `.cursor/rules`, `.cursorrules`, or `.github/copilot-instructions.md` found in repo.
