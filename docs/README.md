# Claude Agent SDK Rust - Documentation

Welcome to the official documentation for Claude Agent SDK Rust.

## 📚 Documentation Structure

### Getting Started
- [Installation Guide](../README.md#installation)
- [Quick Start](../README.md#quick-start)
- [Examples](../examples/README.md)

### Core Concepts
- [Architecture Overview](architecture/overview.md)
- [Agent System](architecture/agents.md)
- [Orchestration System](architecture/orchestration.md)
- [Skills System](architecture/skills.md)

### User Guides
- [V2 API Guide](guides/v2-api-guide.md) - Complete V2 API documentation with examples
- [Subagent Guide](guides/subagent-guide.md) - Comprehensive subagent system tutorial
- [Best Practices](guides/best-practices.md) - Production-ready coding patterns
- [Troubleshooting](guides/troubleshooting.md) - Common issues and solutions
- [Investment Analysis Guide](guides/investment-analysis.md)
- [Trading Integration Guide](guides/trading-integration.md)
- [Backtesting Guide](guides/backtesting.md)
- [Data Sources Guide](guides/data-sources.md)

### API Reference
- [Public API Documentation](api/public-api.md)
- [MCP Integration](api/mcp-integration.md)
- [Trading API](api/trading-api.md)

### Development
- [Development Setup](development/setup.md)
- [Testing Guide](development/testing.md)
- [Contributing Guidelines](../CONTRIBUTING.md)
- [Code Style Guide](development/code-style.md)

### Appendices
- [Changelog](../CHANGELOG.md)
- [Migration Guide (V1 → V2)](../MIGRATION_GUIDE.md) - Complete API migration guide
- [Migration Guide](development/migration-guide.md)
- [Troubleshooting](development/troubleshooting.md)
- [FAQ](guides/faq.md)

## 🎯 Quick Links

- **Installation**: See [README](../README.md)
- **Examples**: [examples/](../examples/)
- **API Docs**: Run `cargo doc --open`
- **Contributing**: [CONTRIBUTING.md](../CONTRIBUTING.md)

## 📖 Reading Order

### For Users
1. [README](../README.md) - Installation and quick start
2. [Examples](../examples/README.md) - Learn by doing
3. [Guides](guides/) - Domain-specific usage

### For Contributors
1. [Development Setup](development/setup.md)
2. [Architecture Overview](architecture/overview.md)
3. [Testing Guide](development/testing.md)
4. [Contributing Guidelines](../CONTRIBUTING.md)

## 🔍 Searching Documentation

Use the search function in your editor or IDE to find:
- **Concepts**: Search for specific terms like "Agent", "Skill", "MCP"
- **API**: Search for type names like `InvestmentAssistant`, `BacktestEngine`
- **Examples**: Search for file names in `examples/`

## 📝 Documentation Convention

This project follows the [Rust Documentation Guidelines](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html):

- **`//!`**: Crate-level and module-level documentation
- **`///`**: Item-level documentation (functions, structs, etc.)
- **`/// # Examples`**: Code examples in documentation
- **`cargo doc`**: Generate HTML documentation from doc comments

## 🚀 Building Documentation

```bash
# Generate and open documentation
cargo doc --open

# Build documentation for all packages
cargo doc --workspace --open

# Build with public dependencies only
cargo doc --no-deps --open
```

## 🤝 Contributing to Documentation

Documentation improvements are welcome! Please:

1. Check the [Contributing Guidelines](../CONTRIBUTING.md)
2. Update the relevant documentation file
3. Ensure examples are tested and working
4. Run `cargo doc` to verify doc comments compile
5. Submit a Pull Request

## 📧 Support

For questions or issues:
- Check [FAQ](guides/faq.md)
- Review [Troubleshooting](development/troubleshooting.md)
- Open an [Issue](https://github.com/louloulin/claude-agent-sdk/issues)

---

**Last Updated**: 2026-01-12
