# AGENTS.md - Caddy-Dev Development Guide

This document provides guidelines for agentic coding agents operating in this repository.

## Project Overview

- **Language**: Rust 2024 Edition
- **Toolchain**: rustc 1.90+, cargo 1.90+
- **Type**: CLI application for generating Caddyfile.dev from templates
- **Dependencies**: clap (CLI parsing), dialoguer (interactive prompts), glob (file patterns), dirs (config directories)

## Build Commands

### Core Operations

```bash
# Build release binary
cargo build --release

# Build debug binary
cargo build

# Run in development mode
cargo run -- [arguments]

# Clean build artifacts
cargo clean
```

### Testing

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Linting & Formatting

```bash
# Format code
cargo fmt

# Check formatting (without modifying)
cargo fmt --check

# Run clippy lints
cargo clippy

# Run clippy with warnings allowed
cargo clippy -- -A warnings

# Check for errors without building
cargo check

# Audit dependencies for vulnerabilities
cargo audit

# Update dependencies
cargo update
```

## Code Style Guidelines

### Imports & Organization

- Group imports by category with blank lines between groups: std → external → local
- Use explicit import paths (no glob imports except for `use std::collections::*`)
- Alphabetize within groups

```rust
// Correct order
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input};
use dirs::config_dir;
```

### Naming Conventions

- **Structs/Enums**: PascalCase (e.g., `Args`, `Command`)
- **Functions/Variables**: snake_case (e.g., `generate_caddyfile_dev`, `output_dir`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Field names**: snake_case (e.g., `output_dir: Option<PathBuf>`)

### Error Handling

- Use `Result` for fallible operations
- Use `eprintln!` with exit code 1 for CLI errors
- Provide descriptive error messages with context
- Use `unwrap_or_else` for fallback logic

```rust
if let Err(e) = fs::create_dir_all(&config_dir) {
    eprintln!("Error creating config directory '{}': {}", config_dir.display(), e);
    std::process::exit(1);
}
```

### CLI Design

- Use clap derive macros for CLI argument parsing
- Define enums for subcommands with doc comments
- Use descriptive argument names with long flags

```rust
#[derive(Parser, Debug)]
#[command(author, version, about, subcommand_required = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate Caddyfile.dev from a template
    Generate {
        #[arg(short = 'o', long = "output-dir", value_name = "DIR")]
        output_dir: Option<PathBuf>,
    },
    /// Initialize caddy-dev by setting up folders
    Init,
    /// Reload Caddy with the generated config
    Reload,
}
```

### Code Patterns

- Use `PathBuf` for file paths, avoid `String`
- Use `Option<T>` for optional values
- Use early returns for error cases
- Prefer `if let` over `match` for single patterns

### File Organization

- Keep main entry point minimal
- Extract functions for distinct operations
- Private functions below their public callers
- Max ~300 lines per file

### Testing

- Add `#[test]` functions for unit tests
- Test error cases as well as success cases
- Use `tempfile` or `assert_fs` for filesystem tests

### Common Patterns

- Use `std::process::exit(1)` for CLI errors after printing
- Use `expect()` only for truly unrecoverable errors
- Handle both absolute and home-directory-relative paths (`~`)

## Pre-Commit Checklist

```bash
cargo fmt && cargo clippy && cargo check
```

## File Extensions

- Rust source: `.rs`
- Cargo manifest: `Cargo.toml`

