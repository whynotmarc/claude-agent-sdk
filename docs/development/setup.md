# Development Setup Guide

This guide covers setting up a development environment for contributing to Claude Agent SDK Rust.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Repository Setup](#repository-setup)
- [Building](#building)
- [Testing](#testing)
- [Development Workflow](#development-workflow)
- [Debugging](#debugging)
- [Code Style](#code-style)
- [Documentation](#documentation)

---

## Prerequisites

### Required Software

1. **Rust Toolchain**: 1.90 or higher
   ```bash
   rustup --version
   rustc --version
   cargo --version
   ```

2. **Claude Code CLI**: 2.0.0 or higher
   ```bash
   claude --version
   ```

3. **Git**: Version control
   ```bash
   git --version
   ```

### Recommended Tools

1. **IDE/Editor**:
   - VS Code with rust-analyzer
   - IntelliJ IDEA with Rust plugin
   - Neovim with rust-tools.nvim

2. **CLI Tools**:
   - `cargo-watch` - Watch for changes and rebuild
   - `cargo-nextest` - Faster test execution
   - `cargo-expand` - Macro expansion inspection

3. **Linter/Formatter**:
   - `rustfmt` - Code formatting
   - `clippy` - Linting

---

## Repository Setup

### 1. Clone Repository

```bash
git clone https://github.com/louloulin/claude-agent-sdk.git
cd claude-agent-sdk
```

### 2. Install Dependencies

```bash
# Install all dependencies
cargo build --workspace
```

### 3. Verify Installation

```bash
# Run basic tests
cargo test --workspace --lib

# Check compilation
cargo check --workspace
```

### 4. Configure Environment

```bash
# Set Anthropic API key
export ANTHROPIC_API_KEY="your_api_key_here"

# Optional: Set data source API keys
export ALPHA_VANTAGE_API_KEY="your_alpha_vantage_key"
```

---

## Building

### Development Build

```bash
# Build all crates (debug)
cargo build --workspace

# Build specific crate
cargo build -p claude-agent-sdk
cargo build -p investintel-agent
```

### Release Build

```bash
# Optimized build
cargo build --workspace --release

# Build specific binary
cargo build --bin invest-cli --release
```

### Examples

```bash
# Build all examples
cargo build --examples

# Build specific example
cargo build --example 01_hello_world
```

### Build Features

```bash
# Build with default features
cargo build --workspace

# Build without default features
cargo build --workspace --no-default-features

# Build with specific features
cargo build --workspace --features "ml,experimental"
```

---

## Testing

### Run All Tests

```bash
# Run all tests in workspace
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run tests in parallel
cargo test --workspace -- --test-threads=4
```

### Run Specific Tests

```bash
# Run library tests only
cargo test --package claude-agent-sdk --lib
cargo test --package investintel-agent --lib

# Run specific test
cargo test test_name
cargo test test_module::test_name

# Run tests matching pattern
cargo test investment
```

### Test Categories

```bash
# Unit tests (fast, no external dependencies)
cargo test --workspace --lib

# Integration tests (slower, may require APIs)
cargo test --workspace --test '*'

# Doc tests (examples in documentation)
cargo test --doc
```

### Test Output Options

```bash
# Show test output
cargo test -- --nocapture

# Pretty print test output
cargo test -- --format pretty

# Log test execution
cargo test -- --show-output
```

### Common Test Issues

#### API Keys Required

Some tests require external APIs. These tests are skipped if API keys are not set:

```bash
# Set API keys to enable these tests
export ALPHA_VANTAGE_API_KEY="your_key"
export YAHOO_FINANCE_API_KEY="your_key"
```

#### Network Tests

Tests requiring network access may fail offline. Skip with:

```bash
cargo test --workspace --skip network
```

---

## Development Workflow

### 1. Create Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

```bash
# Edit files
vim src/file.rs

# Watch for changes and rebuild
cargo watch -x build
```

### 3. Run Tests

```bash
# Watch mode: rebuild and test on changes
cargo watch -x test
```

### 4. Format Code

```bash
# Format all files
cargo fmt

# Check formatting without making changes
cargo fmt -- --check

# Format specific package
cargo fmt -p claude-agent-sdk
```

### 5. Lint Code

```bash
# Run clippy
cargo clippy --all-targets --all-features

# Auto-fix lint issues
cargo clippy --fix

# Check without building
cargo clippy --all-targets --all-features -- -D warnings
```

### 6. Commit Changes

```bash
git add .
git commit -m "feat: add your feature"
```

### 7. Push and Create PR

```bash
git push origin feature/your-feature-name
# Create PR on GitHub
```

---

## Debugging

### Enable Debug Output

```rust
let options = ClaudeAgentOptions {
    stderr_callback: Some(Arc::new(|msg| {
        eprintln!("DEBUG: {}", msg);
    })),
    extra_args: Some({
        let mut args = HashMap::new();
        args.insert("debug-to-stderr".to_string(), None);
        args
    }),
    ..Default::default()
};
```

### Use Logging

```rust
use tracing::{info, debug, error, instrument};

#[tracing::instrument]
async fn my_function(arg: &str) -> Result<()> {
    info!("Processing: {}", arg);
    debug!("Detailed info");
    // ...
    Ok(())
}
```

### Debug Tests

```bash
# Capture test output
cargo test -- --nocapture --test-threads=1

# Print test output
cargo test -- --show-output

# Run single test with output
cargo test test_name -- --exact --nocapture
```

### Memory Profiling

```bash
# Use Valgrind (Linux)
cargo build --release
valgrind --leak-check=full ./target/release/invest-cli

# Use Instruments (macOS)
cargo build --release
instruments -t "Leaks" target/release/invest-cli
```

### Performance Profiling

```bash
# Use flamegraph
cargo install flamegraph
cargo flamegraph --bin invest-cli

# Use criterion for benchmarks
cargo bench
```

---

## Code Style

### Rust Conventions

Follow standard Rust conventions:
- **Naming**: `snake_case` for variables/functions, `PascalCase` for types
- **Line width**: 100 characters (configured in `.rustfmt.toml`)
- **Indentation**: 4 spaces
- **Imports**: Group imports, use `cargo fmt` to sort

### Example Style

```rust
//! Module doc comment

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Struct doc comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyStruct {
    /// Field doc comment
    pub field: String,
}

impl MyStruct {
    /// Creates a new instance
    pub fn new(field: String) -> Self {
        Self { field }
    }

    /// Does something
    ///
    /// # Errors
    ///
    /// Returns an error if...
    ///
    /// # Examples
    ///
    /// ```
    /// let s = MyStruct::new("test".to_string());
    /// ```
    pub fn do_something(&mut self) -> Result<()> {
        // ...
        Ok(())
    }
}
```

### Documentation Comments

- **Module level**: `//!`
- **Item level**: `///`
- **Examples**: Include runnable examples
- **Errors**: Document error conditions
- **Panics**: Document when functions panic

---

## Documentation

### Build Documentation

```bash
# Generate documentation
cargo doc --no-deps

# Build and open in browser
cargo doc --open

# Workspace documentation
cargo doc --workspace --open
```

### Documentation Standards

All public APIs must have:
1. **Doc comments**: `///` or `//!`
2. **Examples**: Runnable code examples
3. **Error documentation**: What errors can occur
4. **Panics**: When it can panic
5. **Safety**: Safety considerations if `unsafe`

### Example Documentation

```rust
/// Queries Claude with a simple prompt.
///
/// This is the simplest way to interact with Claude. It sends a single prompt
/// and returns all messages in the conversation.
///
/// # Arguments
///
/// * `prompt` - The prompt to send to Claude
/// * `options` - Optional configuration for this query
///
/// # Returns
///
/// A vector of `Message` instances representing the conversation.
///
/// # Errors
///
/// Returns an error if:
/// - Claude Code CLI is not installed
/// - API key is not configured
/// - Network communication fails
///
/// # Examples
///
/// ```
/// use claude_agent_sdk::query;
///
/// # async fn example() -> anyhow::Result<()> {
/// let messages = query("What is 2 + 2?", None).await?;
/// # Ok(())
/// # }
/// ```
pub async fn query(
    prompt: &str,
    options: Option<ClaudeAgentOptions>,
) -> Result<Vec<Message>> {
    // ...
}
```

---

## Continuous Integration

### CI Checks

The project uses GitHub Actions for CI:
- **Build**: Verify code compiles
- **Tests**: Run test suite
- **Lint**: Check formatting and clippy
- **Docs**: Verify documentation builds

### Pre-commit Hooks

Install pre-commit hooks:

```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install
```

Or use git hooks with `cargo-husky`:

```bash
cargo install cargo-husky
cargo husky install
```

---

## Troubleshooting

### Build Errors

**Error**: "Failed to compile"

**Solution**:
```bash
# Clean build artifacts
cargo clean

# Rebuild
cargo build --workspace
```

### Test Failures

**Error**: Tests fail locally but pass on CI

**Solution**:
```bash
# Update dependencies
cargo update

# Clean test cache
cargo test --workspace --clean
```

### Claude Code CLI Not Found

**Error**: "Failed to locate claude executable"

**Solution**:
```bash
# Ensure Claude Code CLI is installed
claude --version

# Add to PATH if needed
export PATH="$PATH:/path/to/claude"
```

### API Key Issues

**Error**: "API key not configured"

**Solution**:
```bash
# Set environment variable
export ANTHROPIC_API_KEY="your_key"

# Or configure in Claude Code settings
claude settings
```

---

## Performance Optimization

### Build Time

```bash
# Use cargo check for faster builds
cargo check --workspace

# Incremental builds
cargo build --workspace

# Parallel compilation
cargo build --workspace -j 8
```

### Binary Size

```bash
# Strip symbols
cargo build --release
strip target/release/invest-cli

# Use LTO (Link Time Optimization)
[profile.release]
lto = true
codegen-units = 1
```

### Runtime Performance

```bash
# Profile-guided optimization
cargo build --release --profile pgo

# Benchmarking
cargo bench
```

---

## Related Resources

- [Testing Guide](testing.md)
- [Code Style Guide](code-style.md)
- [Contributing Guidelines](../../CONTRIBUTING.md)
- [Architecture Overview](../architecture/overview.md)

---

**Last Updated**: 2026-01-12
