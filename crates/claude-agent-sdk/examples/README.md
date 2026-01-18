# Claude Agent SDK - Examples

Welcome to the Claude Agent SDK examples directory! This collection contains 56 examples demonstrating all features of the SDK.

## 📋 Prerequisites

Before running any examples, make sure you have:

1. **Rust**: Version 1.90 or higher
2. **Claude Code CLI**: Version 2.0.0 or higher
3. **Authentication**: Either OAuth/Subscription OR API key

   **Option A: OAuth/Subscription (Recommended)**
   ```bash
   claude login  # No API key needed
   ```

   **Option B: API Key**
   ```bash
   export ANTHROPIC_API_KEY=your_api_key_here
   ```

## 🚀 Quick Start

### Run Your First Example
```bash
cargo run --example 01_hello_world
```

### Run Any Example
```bash
cargo run --example <example_name>
```

## 📚 Examples by Category

### 1. Basic Core Features (01-23)
Beginner-friendly examples covering SDK fundamentals

- 01_hello_world - Simple query example
- 02_limit_tool_use - Restrict tool usage
- 06_bidirectional_client - Bidirectional streaming
- 09_agents - Agent orchestration
- ...and 19 more

### 2. Agent Skills System (30-41)
Advanced Skills features and patterns

- 30_agent_skills - Skills overview
- 32_agent_skills_discovery - Discovery mechanism
- 38_agent_skills_hot_reload - Hot reloading
- ...and 10 more

### 3. Advanced Patterns (42-49)
Production-ready patterns and real-world usage

- 42_mcp_async_tasks - Async MCP tasks
- 44_concurrent_queries - Concurrency patterns
- 48_performance_benchmarking - Performance testing
- ...and 8 more

### 4. Production & Integration (50-55)
Enterprise features and deployment

- 50_production_deployment - Deployment guide
- 51_orchestration - Orchestration patterns
- 55_real_skill_md_verification - Verification

## 📖 Learning Path

1. **Start Here** (Beginner)
   - 01_hello_world
   - 02_limit_tool_use
   - 13_system_prompt

2. **Core Features** (Intermediate)
   - 06_bidirectional_client
   - 14_streaming_mode
   - 05_hooks_pretooluse

3. **Advanced Topics** (Advanced)
   - 09_agents
   - 30_agent_skills
   - 42_mcp_async_tasks

## 🔧 Common Issues

### "ANTHROPIC_API_KEY not set"

If using OAuth/Subscription, this error can be ignored - ensure you're logged in:
```bash
claude login
```

If using API key:
```bash
export ANTHROPIC_API_KEY=your_key_here
```

### Example compiles but doesn't run
This is expected - examples need valid authentication and network connection.

## 📞 Getting Help

- **Main README**: [../../README.md](../../README.md)
- **Analysis Report**: [../../EXAMPLES_ANALYSIS_REPORT.md](../../EXAMPLES_ANALYSIS_REPORT.md)
- **GitHub**: [louloulin/claude-agent-sdk](https://github.com/louloulin/claude-agent-sdk)

---

**Happy Coding! 🚀**
