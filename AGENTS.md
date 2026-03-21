# AGENTS.md — Development Guide for `jj-commit`

This file provides guidance for agentic coding assistants operating in this repository.

## Project Overview

`jj-commit` (binary: `jjc`) is a Rust CLI tool that wraps the [Jujutsu](https://github.com/martinvonz/jj) VCS `jj commit` command with an interactive, convention-aware commit-message builder. It supports two conventions: **Conventional Commits** and **Gitmoji**. The repository itself is versioned with Jujutsu (`.jj/`) co-located on a git backing store.

---

## Build, Lint, and Test Commands

All commands go through Cargo. There is no Makefile or custom scripts.

### Build

```sh
cargo build           # debug build
cargo build --release # release build
cargo run -- [args]   # run the binary directly
cargo check           # type-check without producing a binary (fast feedback)
```

### Testing

```sh
cargo test                                  # run all tests
cargo test <test_name>                      # run a single test by name (substring match)
cargo test convention::tests                # run all tests in the convention module
cargo test commit::conventional::tests      # run all tests in a specific submodule
```

Single-test example:

```sh
cargo test test_parse_conventional_full
```

### Lint and Format

```sh
cargo fmt                  # auto-format all source files
cargo fmt -- --check       # check formatting without modifying files (use in CI)
cargo clippy               # run linter; treat warnings as guidance
cargo clippy -- -D warnings # fail on any Clippy warning (stricter mode)
```

No `rustfmt.toml` or `clippy.toml` files exist; defaults are used. Always run `cargo fmt` before committing.

---

## Project Structure

```
src/
├── main.rs              # CLI entry point (clap argument parsing, top-level dispatch)
├── jj.rs                # Shell-out wrapper for `jj` CLI commands
├── commit/
│   ├── mod.rs           # CommitMessage struct + shared prompt helpers
│   ├── conventional.rs  # Parse/build Conventional Commits messages
│   └── gitmoji.rs       # Parse/build Gitmoji messages
├── convention/
│   ├── mod.rs           # Convention enum + auto-detection logic
│   ├── conventional.rs  # is_conventional() predicate
│   └── gitmoji.rs       # is_gitmoji() predicate
└── types/
    ├── mod.rs           # Re-exports
    ├── conventional.rs  # ConventionalType enum (feat, fix, chore, …)
    └── gitmoji.rs       # GitmojiType struct + GITMOJIS static table
```

Each domain area has its own subdirectory with a `mod.rs` public interface and `conventional.rs` / `gitmoji.rs` siblings implementing convention-specific logic. Follow this pattern when adding new features.

---

## Code Style Guidelines

### Language

- **Rust, edition 2024**, targeting Rust 1.94.0+.

### Types and Data Modeling

- Use **enums** for finite sets of variants: `Convention`, `ConventionalType`.
- Use **structs** for composite data: `CommitMessage`, `GitmojiType`.
- Use `Option<T>` for optional fields; never use sentinel string values (e.g., empty string) as a substitute.
- Use `Vec<String>` for variable-length collections.
- Use `&'static str` for string fields in static data tables (e.g., `GITMOJIS`).
- Derive `clap::ValueEnum` on enums that are exposed as CLI arguments.

### Naming Conventions

| Construct                     | Convention             | Example                               |
| ----------------------------- | ---------------------- | ------------------------------------- |
| Types (structs, enums)        | `PascalCase`           | `CommitMessage`, `ConventionalType`   |
| Functions, methods, variables | `snake_case`           | `parse_conventional`, `build_gitmoji` |
| Modules                       | `snake_case`           | `commit`, `convention`, `types`       |
| Static/const data             | `SCREAMING_SNAKE_CASE` | `GITMOJIS`                            |

### Error Handling

- Use `color_eyre::eyre::Result<T>` as the return type for fallible functions.
- Use `bail!()` (from `color_eyre`) for early-exit error conditions.
- Use `eyre!()` for constructing ad-hoc errors.
- Use `.wrap_err("context message")` to attach context when propagating errors with `?`.
- **Never use `.unwrap()`** in production code paths. Prefer `.unwrap_or("")`, `.unwrap_or_default()`, or proper `?` propagation.
- Check subprocess success via `output.status.success()` before using output.

```rust
// Good
let output = Command::new("jj").args(&args).output().wrap_err("failed to run jj")?;
if !output.status.success() {
    bail!("jj exited with non-zero status");
}

// Bad
let output = Command::new("jj").args(&args).output().unwrap();
```

---

## Testing Guidelines

### Test Location

Tests are **unit tests co-located** with the code they test, in an inline module at the bottom of each file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_<function>_<scenario>() {
        // Arrange
        // Act
        // Assert
    }
}
```

There is no `tests/` directory and no integration tests. Do not create a separate test file unless there is a compelling reason.

### What to Test

- Test **pure functions**: parsing, detection predicates, string formatting.
- Do **not** attempt to test interactive prompt functions (`build_conventional`, `build_gitmoji`, `prompt_*`) — they require a TTY.
- Do **not** attempt to test `jj.rs` subprocess calls — they require a real `jj` installation.

### Test Naming

Follow the pattern `test_<function_name>_<scenario>`:

- `test_parse_conventional_full`
- `test_parse_conventional_no_scope`
- `test_is_gitmoji_shortcode_form`
- `test_detect_convention_no_match`

---

## Workflow Reminders

- Run `cargo fmt` before committing any code change.
- Run `cargo clippy` and address warnings before committing.
- Run `cargo test` to verify no tests are broken.
- The project uses Jujutsu for version control. Use `jj` commands (not `git`) when interacting with the repository history, unless specifically using the git backing store.
