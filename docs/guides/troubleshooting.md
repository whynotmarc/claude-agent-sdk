# Claude Agent SDK Rust - Troubleshooting Guide

**Version**: 1.0.0
**Last Updated**: 2026-01-13
**SDK Version**: 0.7.0+

---

## Table of Contents

1. [Quick Diagnosis](#quick-diagnosis)
2. [Common Errors](#common-errors)
3. [V1 API Issues](#v1-api-issues)
4. [V2 API Issues](#v2-api-issues)
5. [Subagent Issues](#subagent-issues)
6. [Skills Issues](#skills-issues)
7. [Performance Issues](#performance-issues)
8. [Network Issues](#network-issues)
9. [Build/Compile Issues](#buildcompile-issues)
10. [Testing Issues](#testing-issues)

---

## Quick Diagnosis

### Diagnostic Checklist

Before diving into specific issues, run through this checklist:

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions};

#[tokio::main]
async fn diagnostic() -> anyhow::Result<()> {
    println!("=== Claude Agent SDK Diagnostic ===\n");

    // 1. Check API key
    match std::env::var("ANTHROPIC_API_KEY") {
        Ok(_) => println!("✅ ANTHROPIC_API_KEY is set"),
        Err(_) => println!("❌ ANTHROPIC_API_KEY is NOT set"),
    }

    // 2. Test simple query
    println!("\nTesting simple query...");
    match prompt("Say 'test'", SessionOptions::default()).await {
        Ok(result) => println!("✅ Query successful: {}", result.message.content),
        Err(e) => println!("❌ Query failed: {}", e),
    }

    // 3. Check budget
    println!("\nTesting budget limit...");
    let options = SessionOptions::builder()
        .max_budget_usd(Some(0.01))
        .build();
    match prompt("Write a book", options).await {
        Ok(_) => println!("⚠️ Budget limit not enforced"),
        Err(claude_agent_sdk::ClaudeError::BudgetExceeded) => {
            println!("✅ Budget limit working")
        }
        Err(e) => println!("❌ Unexpected error: {}", e),
    }

    Ok(())
}
```

### Log Levels

Enable debug logging:

```rust
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Enable debug logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Your code here...
    Ok(())
}
```

---

## Common Errors

### Error 1: "ANTHROPIC_API_KEY not set"

**Symptom**:
```
Error: Environment variable ANTHROPIC_API_KEY not set
```

**Causes**:
1. Environment variable not set
2. Variable name misspelled
3. Not exported in shell

**Solutions**:

#### Solution 1: Set environment variable

```bash
# Linux/macOS
export ANTHROPIC_API_KEY="sk-ant-..."

# Windows (PowerShell)
$env:ANTHROPIC_API_KEY="sk-ant-..."

# Windows (Command Prompt)
set ANTHROPIC_API_KEY=sk-ant-...
```

#### Solution 2: Use .env file

```bash
# Install dotenv
cargo install dotenv

# Create .env file
echo 'ANTHROPIC_API_KEY=sk-ant-...' > .env

# Load in code
dotenv::dotenv().ok();
```

#### Solution 3: Set programmatically

```rust
std::env::set_var("ANTHROPIC_API_KEY", "sk-ant-...");
```

**Verification**:
```rust
fn check_api_key() {
    match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) => println!("API key set: {}...", &key[..10]),
        Err(_) => eprintln!("API key NOT set"),
    }
}
```

---

### Error 2: "Budget exceeded"

**Symptom**:
```
Error: BudgetExceeded
```

**Causes**:
1. Query cost exceeded `max_budget_usd`
2. Cumulative cost of multi-turn conversation

**Solutions**:

#### Solution 1: Increase budget

```rust
let options = SessionOptions::builder()
    .max_budget_usd(Some(10.0)) // Increase from 5.0 to 10.0
    .build();
```

#### Solution 2: Reduce query complexity

```rust
// Instead of:
prompt("Write a comprehensive guide covering all aspects of Rust", options).await?;

// Use:
prompt("Write a brief overview of Rust", options).await?;
```

#### Solution 3: Use cheaper model

```rust
let options = SessionOptions::builder()
    .model(Some("claude-haiku-4")) // Cheaper than sonnet/opus
    .max_budget_usd(Some(5.0))
    .build();
```

**Monitoring**:
```rust
async fn monitored_query(query: &str) -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    let result = prompt(query, SessionOptions::default()).await?;

    let elapsed = start.elapsed();
    println!("Query completed in {:?}", elapsed);

    // Check if approaching budget
    // (This is hypothetical - actual budget tracking depends on implementation)

    Ok(())
}
```

---

### Error 3: "Network timeout"

**Symptom**:
```
Error: NetworkError: Timeout after 30s
```

**Causes**:
1. Slow network connection
2. Long-running query
3. Firewall blocking requests

**Solutions**:

#### Solution 1: Increase timeout (if configurable)

```rust
// Note: Actual timeout configuration depends on implementation
// This is hypothetical

let options = SessionOptions::builder()
    .timeout(Some(Duration::from_secs(120))) // Increase to 2 minutes
    .build();
```

#### Solution 2: Break into smaller queries

```rust
// Instead of one large query:
prompt("Explain everything about Rust ownership, borrowing, lifetimes, and memory safety", options).await?;

// Use multiple smaller queries:
let topics = vec![
    "Explain Rust ownership",
    "Explain Rust borrowing",
    "Explain Rust lifetimes",
    "Explain Rust memory safety",
];

for topic in topics {
    let result = prompt(topic, SessionOptions::default()).await?;
    println!("{}\n", result.message.content);
}
```

#### Solution 3: Implement retry with exponential backoff

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn query_with_retry(
    message: &str,
    max_retries: u32,
) -> anyhow::Result<String> {
    let mut retry_count = 0;

    loop {
        match prompt(message, SessionOptions::default()).await {
            Ok(result) => return Ok(result.message.content),
            Err(claude_agent_sdk::ClaudeError::NetworkError(_))
                if retry_count < max_retries =>
            {
                retry_count += 1;
                let backoff = Duration::from_secs(2u64.pow(retry_count));
                eprintln!("Network timeout, retry {} in {:?}", retry_count, backoff);
                sleep(backoff).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
```

---

### Error 4: "Permission denied"

**Symptom**:
```
Error: Permission denied
```

**Causes**:
1. Insufficient permission mode
2. Operation requires elevated permissions
3. File system permissions issue

**Solutions**:

#### Solution 1: Use appropriate permission mode

```rust
use claude_agent_sdk::v2::{SessionOptions, PermissionMode};

// For automation
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::BypassPermissions))
    .build();

// For planning
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::Plan))
    .build();

// For interactive use
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::Default))
    .build();
```

#### Solution 2: Check file permissions

```bash
# Check file permissions
ls -la path/to/file

# Fix permissions if needed
chmod 644 path/to/file
```

---

## V1 API Issues

### Issue 1: Streaming stops unexpectedly

**Symptom**:
```rust
let mut stream = query_stream("Explain Rust", options).await?;
while let Some(result) = stream.next().await {
    // Stream stops after a few messages
}
```

**Causes**:
1. Budget exhausted
2. Max turns reached
3. Error occurred

**Solution**:
```rust
let mut stream = query_stream("Explain Rust", options).await?;

loop {
    match stream.next().await {
        Some(Ok(Message::Assistant(msg))) => {
            println!("{}", msg.message.content);
        }
        Some(Ok(Message::Result(_))) => {
            println!("\n[End of conversation]");
            break;
        }
        Some(Err(e)) => {
            eprintln!("Error: {}", e);
            break;
        }
        None => {
            println!("\n[Stream ended]");
            break;
        }
        _ => {}
    }
}
```

### Issue 2: ClaudeClient::connect() fails

**Symptom**:
```rust
let mut client = ClaudeClient::new(options);
client.connect().await?; // Fails
```

**Causes**:
1. Network connectivity issue
2. Invalid credentials
3. Server unavailable

**Solution**:
```rust
use claude_agent_sdk::{ClaudeClient, ClaudeAgentOptions, ClaudeError};

async fn connect_with_retry(
    max_retries: u32,
) -> anyhow::Result<ClaudeClient> {
    let mut retry_count = 0;
    let options = ClaudeAgentOptions::default();

    loop {
        match ClaudeClient::connect_with_options(&options).await {
            Ok(client) => return Ok(client),
            Err(ClaudeError::NetworkError(_)) if retry_count < max_retries => {
                retry_count += 1;
                eprintln!("Connection failed, retry {}...", retry_count);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
```

---

## V2 API Issues

### Issue 1: "Session not connected"

**Symptom**:
```rust
let mut session = create_session(SessionOptions::default()).await?;
let messages = session.receive().await?; // Error: Session not connected
```

**Cause**: Calling `receive()` before `send()`

**Solution**:
```rust
let mut session = create_session(SessionOptions::default()).await?;

// ALWAYS send first
session.send("Hello").await?;

// THEN receive
let messages = session.receive().await?;
```

### Issue 2: Message type confusion

**Symptom**:
```rust
let messages = session.receive().await?;
for msg in messages {
    println!("{}", msg.message.content); // Prints user messages too
}
```

**Solution**:
```rust
let messages = session.receive().await?;

for msg in messages {
    match msg.type_.as_str() {
        "assistant" => println!("Claude: {}", msg.message.content),
        "user" => println!("User: {}", msg.message.content),
        "result" => {
            println!("[End of conversation]");
            break;
        }
        _ => continue,
    }
}
```

### Issue 3: Session not resuming

**Symptom**:
```rust
let session = resume_session("session-id", SessionOptions::default()).await?;
// Session starts fresh instead of resuming
```

**Cause**: Invalid session ID or session expired

**Solution**:
```rust
// Save session ID when created
let mut session = create_session(SessionOptions::default()).await?;
let session_id = session.get_id(); // Hypothetical method
std::fs::write("session_id.txt", &session_id)?;

// Later, resume with valid ID
let session_id = std::fs::read_to_string("session_id.txt")?;
let mut session = resume_session(&session_id, SessionOptions::default()).await?;
```

---

## Subagent Issues

### Issue 1: "Subagent not found"

**Symptom**:
```rust
executor.execute("my_agent", "task").await?;
// Error: NotFound("my_agent")
```

**Cause**: Subagent not registered or name misspelled

**Solution**:
```rust
let agent = Subagent::builder()
    .name("my_agent") // Exact name
    .description("My agent")
    .instructions("Do tasks")
    .allowed_tools(vec![])
    .build();

executor.register(agent)?;

// Verify registration
if executor.has_subagent("my_agent") {
    executor.execute("my_agent", "task").await?;
} else {
    eprintln!("Agent not registered");
}
```

### Issue 2: "Subagent execution failed"

**Symptom**:
```rust
executor.execute("agent", "task").await?;
// Error: ExecutionFailed("...")
```

**Causes**:
1. Invalid input format
2. Unclear instructions
3. Missing tools

**Solution**:
```rust
// Ensure clear instructions
let agent = Subagent::builder()
    .name("summarizer")
    .description("Text summarizer")
    .instructions(
        "Summarize the given text in 1-2 sentences. \
         Focus on main points."
    )
    .allowed_tools(vec![])
    .build();

// Provide clear input
let result = executor.execute(
    "summarizer",
    "Summarize this: Rust is a systems programming language \
     that runs blazingly fast, prevents segfaults, and \
     guarantees thread safety."
).await?;
```

---

## Skills Issues

### Issue 1: SKILL.md parsing fails

**Symptom**:
```rust
let skill = SkillMdFile::parse("path/to/SKILL.md")?;
// Error: ParseError("...")
```

**Cause**: Invalid SKILL.md format

**Solution**:
```markdown
<!-- Correct SKILL.md format -->
# Skill Name

**Description**: Brief description

## Instructions

Your instructions here.

## Resources

- resource1.txt
- resource2.json
```

**Verification**:
```rust
fn validate_skill_md(path: &Path) -> Result<(), SkillMdError> {
    let content = std::fs::read_to_string(path)?;

    // Check required sections
    if !content.contains("# ") {
        return Err(SkillMdError::MissingTitle);
    }

    if !content.contains("## Instructions") {
        return Err(SkillMdError::MissingInstructions);
    }

    Ok(())
}
```

### Issue 2: Resource not found

**Symptom**:
```rust
let resource = skill.get_resource("config.txt");
// Returns None
```

**Cause**: Resource not listed in SKILL.md or file doesn't exist

**Solution**:
```rust
// Check if resource exists
if let Some(resource_path) = skill.get_resource("config.txt") {
    if resource_path.exists() {
        let content = std::fs::read_to_string(resource_path)?;
        println!("{}", content);
    } else {
        eprintln!("Resource file doesn't exist: {:?}", resource_path);
    }
} else {
    eprintln!("Resource not found in cache");

    // List available resources
    println!("Available resources:");
    for name in skill.get_resource_names() {
        println!("- {}", name);
    }
}
```

---

## Performance Issues

### Issue 1: Slow response times

**Symptom**: Queries take >10 seconds

**Diagnosis**:
```rust
use std::time::Instant;

async fn timed_query(query: &str) -> anyhow::Result<String> {
    let start = Instant::now();

    let result = prompt(query, SessionOptions::default()).await?;

    let elapsed = start.elapsed();
    println!("Query took {:?}", elapsed);

    if elapsed.as_secs() > 10 {
        eprintln!("WARNING: Query took longer than 10 seconds");
    }

    Ok(result.message.content)
}
```

**Solutions**:

#### Solution 1: Use faster model
```rust
let options = SessionOptions::builder()
    .model(Some("claude-haiku-4")) // Fastest
    .build();
```

#### Solution 2: Reduce input size
```rust
// Instead of sending entire file
let content = std::fs::read_to_string("large_file.txt")?;
prompt(&content, options).await?;

// Send excerpt only
let content = std::fs::read_to_string("large_file.txt")?;
let excerpt = &content[..1000]; // First 1000 chars
prompt(excerpt, options).await?;
```

#### Solution 3: Cache responses
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
        let read = cache.read().await;
        if let Some(cached) = read.get(query) {
            return Ok(cached.clone());
        }
    }

    // Query
    let result = prompt(query, SessionOptions::default()).await?;

    // Cache result
    let response = result.message.content;
    {
        let mut write = cache.write().await;
        write.insert(query.to_string(), response.clone());
    }

    Ok(response)
}
```

### Issue 2: High memory usage

**Symptom**: Memory usage grows over time

**Cause**: Not releasing resources

**Solution**:
```rust
// ✅ GOOD: Explicit cleanup
async fn with_cleanup() -> anyhow::Result<()> {
    let mut session = create_session(SessionOptions::default()).await?;

    session.send("Hello").await?;
    let messages = session.receive().await?;

    // Process messages
    for msg in messages {
        println!("{}", msg.message.content);
    }

    // Explicit cleanup
    session.close().await?;

    Ok(())
}

// ❌ BAD: No cleanup
async fn without_cleanup() -> anyhow::Result<()> {
    let session = create_session(SessionOptions::default()).await?;
    // Session never closed, resources leaked
    Ok(())
}
```

---

## Network Issues

### Issue 1: Connection refused

**Symptom**:
```
Error: Connection refused (os error 111)
```

**Causes**:
1. Firewall blocking connection
2. Proxy configuration needed
3. DNS resolution failure

**Solutions**:

#### Solution 1: Check firewall
```bash
# Test connectivity
curl https://api.anthropic.com

# Allow connections if blocked
# (commands depend on your firewall)
```

#### Solution 2: Configure proxy
```rust
std::env::set_var("HTTP_PROXY", "http://proxy.example.com:8080");
std::env::set_var("HTTPS_PROXY", "http://proxy.example.com:8080");
```

### Issue 2: DNS resolution fails

**Symptom**:
```
Error: dns error: failed to lookup address information
```

**Solution**:
```bash
# Test DNS
nslookup api.anthropic.com

# Try different DNS server
echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf
```

---

## Build/Compile Issues

### Issue 1: "Linking with cc failed"

**Symptom**:
```
error: linking with `cc` failed: exit code: 1
```

**Cause**: Missing system dependencies

**Solution**:
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora
sudo dnf install gcc
```

### Issue 2: "Cannot find -lssl"

**Symptom**:
```
error: cannot find -lssl
```

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# Fedora
sudo dnf install openssl-devel

# macOS
brew install openssl
export OPENSSL_DIR=/usr/local/opt/openssl
```

---

## Testing Issues

### Issue 1: Tests timeout

**Symptom**:
```
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

**Solution**:
```rust
#[tokio::test]
async fn test_with_timeout() -> anyhow::Result<()> {
    // Set timeout
    tokio::time::timeout(
        Duration::from_secs(30),
        async {
            let result = prompt("Test", SessionOptions::default()).await?;
            Ok(())
        }
    ).await??;

    Ok(())
}
```

### Issue 2: Flaky tests

**Symptom**: Tests pass/fail intermittently

**Causes**:
1. Network issues
2. Race conditions
3. Resource contention

**Solution**:
```rust
#[tokio::test]
async fn test_with_retry() -> anyhow::Result<()> {
    let mut retry_count = 0;
    let max_retries = 3;

    loop {
        match prompt("Test", SessionOptions::default()).await {
            Ok(_) => return Ok(()),
            Err(_) if retry_count < max_retries => {
                retry_count += 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
```

---

## Getting Help

If you can't resolve your issue:

1. **Check Documentation**:
   - [V2 API Guide](v2-api-guide.md)
   - [Subagent Guide](subagent-guide.md)
   - [Best Practices](best-practices.md)

2. **Search Issues**:
   - [GitHub Issues](https://github.com/louloulin/cc-agent-sdk/issues)

3. **Create Minimal Reproduction**:
   ```rust
   // Minimal reproducible example
   #[tokio::main]
   async fn main() -> anyhow::Result<()> {
       // Your minimal code here
       Ok(())
   }
   ```

4. **Include Information**:
   - SDK version
   - Rust version (`rustc --version`)
   - Error message
   - Minimal code example
   - Steps to reproduce

---

**Document Version**: 1.0.0
**Last Updated**: 2026-01-13
**Maintainer**: Loulou Lin
