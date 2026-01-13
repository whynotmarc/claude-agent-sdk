# Claude Agent SDK Rust - Best Practices

**Version**: 1.0.0
**Last Updated**: 2026-01-13
**SDK Version**: 0.7.0+

---

## Table of Contents

1. [Core Principles](#core-principles)
2. [API Usage](#api-usage)
3. [Error Handling](#error-handling)
4. [Performance Optimization](#performance-optimization)
5. [Security](#security)
6. [Testing](#testing)
7. [Code Organization](#code-organization)
8. [Resource Management](#resource-management)
9. [Documentation](#documentation)
10. [Deployment](#deployment)

---

## Core Principles

### 1. Choose the Right API

**V1 API** (Default):
```rust
use claude_agent_sdk::{query, ClaudeClient};

// Use when you need:
// - Async generator-style streaming
// - Session forking
// - Fine-grained lifecycle control
```

**V2 API** (Simplified):
```rust
use claude_agent_sdk::v2::{prompt, create_session};

// Use when you need:
// - Simpler send/receive pattern
// - Cleaner multi-turn conversations
// - TypeScript-like experience
```

**Decision Tree**:
```
Need session forking?
├─ Yes → V1 API
└─ No
    └─ Prefer explicit send/receive?
        ├─ Yes → V2 API
        └─ No → V1 API
```

### 2. Always Set Budget Limits

```rust
use claude_agent_sdk::v2::{SessionOptions, PermissionMode};

let options = SessionOptions::builder()
    .max_budget_usd(Some(5.0)) // Prevent overspending
    .max_turns(Some(10))       // Prevent infinite loops
    .build();
```

**Why**:
- Prevent unexpected costs
- Catch runaway conversations
- Enforce resource constraints

### 3. Use Appropriate Permission Modes

```rust
// For automation/scripts
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::BypassPermissions))
    .build();

// For planning operations
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::Plan))
    .build();

// For interactive use
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::Default))
    .build();
```

---

## API Usage

### Session Management

#### ✅ DO: Always close sessions

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

async fn good_session_management() -> anyhow::Result<()> {
    let mut session = create_session(SessionOptions::default()).await?;

    // ... use session ...

    session.close().await?; // Explicit cleanup
    Ok(())
}
```

#### ❌ DON'T: Rely on implicit cleanup

```rust
async fn bad_session_management() -> anyhow::Result<()> {
    let session = create_session(SessionOptions::default()).await?;
    // Session drops without explicit close
    // Resources may not be released immediately
    Ok(())
}
```

### Message Processing

#### ✅ DO: Process messages by type

```rust
let messages = session.receive().await?;

for msg in messages {
    match msg.type_.as_str() {
        "assistant" => println!("Claude: {}", msg.message.content),
        "user" => println!("User: {}", msg.message.content),
        "result" => break, // End of conversation
        _ => continue,     // Ignore unknown types
    }
}
```

#### ❌ DON'T: Assume message order or types

```rust
let messages = session.receive().await?;

// Wrong: Assumes first message is from assistant
if let Some(first) = messages.first() {
    println!("{}", first.message.content); // May be user message or result
}
```

### Error Handling

#### ✅ DO: Handle specific errors

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions, ClaudeError};

match prompt("Hello", SessionOptions::default()).await {
    Ok(result) => println!("{}", result.message.content),
    Err(ClaudeError::QueryFailed(msg)) => {
        eprintln!("Query failed: {}", msg);
        // Retry or fallback logic
    }
    Err(ClaudeError::NetworkError(err)) => {
        eprintln!("Network error: {}", err);
        // Retry with backoff
    }
    Err(ClaudeError::BudgetExceeded) => {
        eprintln!("Budget exceeded");
        // Stop processing
    }
    Err(err) => {
        eprintln!("Unexpected error: {}", err);
        // Log and handle
    }
}
```

#### ❌ DON'T: Use generic error handling

```rust
match prompt("Hello", SessionOptions::default()).await {
    Ok(result) => println!("{}", result.message.content),
    Err(e) => eprintln!("Error: {}", e), // Too generic
}
```

---

## Error Handling

### Comprehensive Error Strategy

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions, ClaudeError};
use std::time::Duration;
use tokio::time::sleep;

async fn robust_query_with_retries(
    message: &str,
    max_retries: u32,
) -> anyhow::Result<String> {
    let mut retry_count = 0;

    loop {
        match prompt(message, SessionOptions::default()).await {
            Ok(result) => return Ok(result.message.content),
            Err(ClaudeError::NetworkError(_)) if retry_count < max_retries => {
                retry_count += 1;
                let backoff = Duration::from_secs(2u64.pow(retry_count));
                eprintln!("Network error, retry {} in {:?}", retry_count, backoff);
                sleep(backoff).await;
            }
            Err(ClaudeError::BudgetExceeded) => {
                return Err(anyhow::anyhow!("Budget exceeded, cannot retry"));
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
```

### Contextual Error Messages

```rust
#[derive(Debug)]
pub enum AppError {
    QueryFailed { context: String, source: ClaudeError },
    SessionClosed { reason: String },
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::QueryFailed { context, source } => {
                write!(f, "Query failed in '{}': {}", context, source)
            }
            AppError::SessionClosed { reason } => {
                write!(f, "Session closed: {}", reason)
            }
        }
    }
}

impl std::error::Error for AppError {}

// Usage
async fn contextual_query() -> Result<String, AppError> {
    prompt("Hello", SessionOptions::default())
        .await
        .map(|r| r.message.content)
        .map_err(|e| AppError::QueryFailed {
            context: "greeting".to_string(),
            source: e,
        })
}
```

---

## Performance Optimization

### Progressive Disclosure

Load resources lazily when needed:

```rust
use claude_agent_sdk::skills::SkillMdFile;

// ✅ GOOD: Use get_resource() for O(1) lookup
let skill = SkillMdFile::parse("path/to/SKILL.md")?;

if let Some(resource_path) = skill.get_resource("config.json") {
    // Load only when accessed
    let content = std::fs::read_to_string(resource_path)?;
}
```

### Parallel Operations

Execute independent tasks concurrently:

```rust
use futures::future::join_all;

async fn parallel_analysis(code: &str) -> anyhow::Result<Vec<String>> {
    let tasks = vec![
        check_syntax(code),
        check_style(code),
        check_security(code),
    ];

    let results = join_all(tasks).await;

    results.into_iter().collect()
}
```

### Caching

Cache expensive operations:

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type Cache = Arc<RwLock<HashMap<String, String>>>;

async fn cached_query(
    cache: Cache,
    query: &str,
) -> anyhow::Result<String> {
    // Check cache
    {
        let read_cache = cache.read().await;
        if let Some(cached) = read_cache.get(query) {
            return Ok(cached.clone());
        }
    }

    // Not in cache, perform query
    let result = prompt(query, SessionOptions::default()).await?;
    let response = result.message.content;

    // Update cache
    {
        let mut write_cache = cache.write().await;
        write_cache.insert(query.to_string(), response.clone());
    }

    Ok(response)
}
```

### Streaming for Large Responses

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};
use futures::StreamExt;

async fn streaming_response() -> anyhow::Result<()> {
    let mut session = create_session(SessionOptions::default()).await?;

    session.send("Explain Rust in detail").await?;

    let messages = session.receive().await?;
    let mut stream = messages.into_stream();

    while let Some(result) = stream.next().await {
        let msg = result?;
        if msg.type_ == "assistant" {
            print!("{}", msg.message.content);
            std::io::stdout().flush()?;
        }
    }

    Ok(())
}
```

---

## Security

### Tool Whitelisting

```rust
use claude_agent_sdk::subagents::Subagent;

// ✅ GOOD: Explicit tool whitelist
let subagent = Subagent::builder()
    .name("file_reader")
    .allowed_tools(vec![
        "read_file".to_string(),
        "list_directory".to_string(),
        // Only specific tools
    ])
    .build();

// ❌ BAD: All tools allowed
let subagent = Subagent::builder()
    .name("dangerous_agent")
    .allowed_tools(vec!["*".to_string()]) // Security risk!
    .build();
```

### Input Validation

```rust
fn validate_input(user_input: &str) -> Result<String, String> {
    // Check length
    if user_input.len() > 10_000 {
        return Err("Input too long".to_string());
    }

    // Check for suspicious patterns
    let suspicious = vec!["<script>", "javascript:", "data:"];
    for pattern in suspicious {
        if user_input.to_lowercase().contains(pattern) {
            return Err(format!("Suspicious pattern detected: {}", pattern));
        }
    }

    Ok(user_input.to_string())
}

async fn safe_query(user_input: &str) -> anyhow::Result<String> {
    let validated = validate_input(user_input)?;

    let result = prompt(&validated, SessionOptions::default()).await?;
    Ok(result.message.content)
}
```

### Environment Variables for Secrets

```rust
use std::env;

// ✅ GOOD: Use environment variables
fn get_api_key() -> anyhow::Result<String> {
    env::var("ANTHROPIC_API_KEY")
        .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))
}

// ❌ BAD: Hardcode secrets
// fn get_api_key() -> String {
//     "sk-ant-1234567890".to_string() // NEVER DO THIS!
// }
```

### Least Privilege Principle

```rust
// ✅ GOOD: Minimal required permissions
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::Default)) // Least privilege
    .build();

// ❌ BAD: Maximum permissions when not needed
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::BypassPermissions))
    .build();
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_success() {
        let result = prompt("Test", SessionOptions::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_budget_limit() {
        let options = SessionOptions::builder()
            .max_budget_usd(Some(0.0001)) // Very low budget
            .build();

        let result = prompt("Write a book", options).await;
        assert!(matches!(result, Err(ClaudeError::BudgetExceeded)));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_multi_turn_conversation() {
    let mut session = create_session(SessionOptions::default()).await.unwrap();

    session.send("My name is Alice").await.unwrap();
    let _ = session.receive().await.unwrap();

    session.send("What's my name?").await.unwrap();
    let messages = session.receive().await.unwrap();

    let response = messages.iter()
        .find(|m| m.type_ == "assistant")
        .unwrap();

    assert!(response.message.content.contains("Alice"));
}
```

### Mocking

```rust
#[cfg(test)]
mockall::mock! {
    pub ClaudeClient {}

    impl ClaudeClient {
        pub async fn query(&mut self, message: &str) -> Result<String, ClaudeError>;
    }
}

#[tokio::test]
async fn test_with_mock() {
    let mut mock = ClaudeClient::new();
    mock.expect_query()
        .returning(|_| Ok("Mock response".to_string()));

    let result = mock.query("Test").await.unwrap();
    assert_eq!(result, "Mock response");
}
```

---

## Code Organization

### Module Structure

```
src/
├── lib.rs              # Public API
├── agents/             # Agent-related code
│   ├── mod.rs
│   └── executor.rs
├── skills/             # Skills system
│   ├── mod.rs
│   └── skill_md.rs
├── subagents/          # Subagent system
│   ├── mod.rs
│   └── executor.rs
└── v2/                 # V2 API
    ├── mod.rs
    └── session.rs
```

### Re-exports

```rust
// lib.rs
pub mod agents;
pub mod skills;
pub mod subagents;
pub mod v2;

// Re-export commonly used types
pub use agents::{ClaudeClient, ClaudeAgentOptions};
pub use skills::{SkillMdFile, SkillsDirScanner};
pub use subagents::{Subagent, SubagentExecutor};
pub use v2::{prompt, create_session, SessionOptions};

// Re-export error type
pub use errors::ClaudeError;
```

### Feature Flags

```toml
[features]
default = ["v1", "v2"]
v1 = []
v2 = []
subagents = ["v1"]
skills = ["v1"]
full = ["v1", "v2", "subagents", "skills"]
```

---

## Resource Management

### Connection Pooling

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct ConnectionPool {
    semaphore: Arc<Semaphore>,
    max_connections: usize,
}

impl ConnectionPool {
    pub fn new(max_connections: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_connections)),
            max_connections,
        }
    }

    pub async fn acquire(&self) -> Arc<Semaphore> {
        self.semaphore.clone().acquire_owned().await.unwrap()
    }
}
```

### Rate Limiting

```rust
use std::time::{Duration, Instant};
use std::collections::VecDeque;

pub struct RateLimiter {
    requests: VecDeque<Instant>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: VecDeque::with_capacity(max_requests),
            max_requests,
            window,
        }
    }

    pub async fn acquire(&mut self) {
        let now = Instant::now();

        // Remove old requests
        while let Some(&front) = self.requests.front() {
            if now.duration_since(front) > self.window {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        // Wait if at limit
        if self.requests.len() >= self.max_requests {
            let oldest = self.requests.front().unwrap();
            let wait_time = self.window.saturating_sub(now.duration_since(*oldest));
            tokio::time::sleep(wait_time).await;
        }

        self.requests.push_back(now);
    }
}
```

---

## Documentation

### Code Documentation

```rust
/// Queries Claude with the given message.
///
/// # Arguments
///
/// * `message` - The user message to send
/// * `options` - Session configuration options
///
/// # Returns
///
/// Returns a `PromptResult` containing Claude's response.
///
/// # Errors
///
/// Returns `ClaudeError::QueryFailed` if the query fails.
/// Returns `ClaudeError::NetworkError` if there's a network issue.
///
/// # Examples
///
/// ```rust
/// use claude_agent_sdk::v2::{prompt, SessionOptions};
///
/// # tokio_test::block_on(async {
/// let result = prompt("Hello!", SessionOptions::default()).await.unwrap();
/// println!("{}", result.message.content);
/// # })
/// ```
pub async fn prompt(
    message: &str,
    options: SessionOptions,
) -> Result<PromptResult, ClaudeError> {
    // Implementation
}
```

### README Sections

```markdown
# Project Name

## Quick Start

\`\`\`rust
use claude_agent_sdk::v2::{prompt, SessionOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = prompt("Hello!", SessionOptions::default()).await?;
    println!("{}", result.message.content);
    Ok(())
}
\`\`\`

## Features

- Feature 1
- Feature 2

## Documentation

- [V2 API Guide](docs/guides/v2-api-guide.md)
- [Subagent Guide](docs/guides/subagent-guide.md)
- [Best Practices](docs/guides/best-practices.md)

## Examples

See [examples/](examples/) for complete examples.
```

---

## Deployment

### Configuration

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub anthropic_api_key: String,
    pub max_budget_usd: Option<f64>,
    pub max_turns: Option<u32>,
    pub permission_mode: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY")?,
            max_budget_usd: std::env::var("MAX_BUDGET_USD")
                .ok()
                .and_then(|s| s.parse().ok()),
            max_turns: std::env::var("MAX_TURNS")
                .ok()
                .and_then(|s| s.parse().ok()),
            permission_mode: std::env::var("PERMISSION_MODE")
                .unwrap_or_else(|_| "default".to_string()),
        })
    }
}
```

### Logging

```rust
use tracing::{info, warn, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting application");

    match prompt("Hello", SessionOptions::default()).await {
        Ok(result) => {
            info!("Query successful");
            println!("{}", result.message.content);
        }
        Err(e) => {
            error!("Query failed: {}", e);
        }
    }

    Ok(())
}
```

### Health Checks

```rust
pub async fn health_check() -> Result<String, String> {
    match prompt("ping", SessionOptions::default()).await {
        Ok(_) => Ok("healthy".to_string()),
        Err(e) => Err(format!("unhealthy: {}", e)),
    }
}
```

---

## Summary

### Key Takeaways

1. **Choose the right API** (V1 vs V2) for your use case
2. **Always set budget limits** to prevent overspending
3. **Handle errors specifically** for better debugging
4. **Use streaming** for large responses
5. **Implement caching** for repeated queries
6. **Whitelist tools** for security
7. **Test thoroughly** with unit and integration tests
8. **Organize code** into logical modules
9. **Document everything** with examples
10. **Monitor resources** in production

### Resources

- [V2 API Guide](v2-api-guide.md)
- [Subagent Guide](subagent-guide.md)
- [Migration Guide](../../../MIGRATION_GUIDE.md)
- [Examples](../../../examples/)

---

**Document Version**: 1.0.0
**Last Updated**: 2026-01-13
**Maintainer**: Loulou Lin
