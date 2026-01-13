# V2 API Complete Guide

**Version**: 1.0.0
**Last Updated**: 2026-01-13
**SDK Version**: 0.7.0+

---

## Table of Contents

1. [Introduction](#introduction)
2. [Quick Start](#quick-start)
3. [Core Concepts](#core-concepts)
4. [API Reference](#api-reference)
5. [Usage Patterns](#usage-patterns)
6. [Advanced Topics](#advanced-topics)
7. [Examples](#examples)
8. [Migrating from V1](#migrating-from-v1)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Introduction

The V2 API provides a simplified, TypeScript-inspired interface for interacting with Claude. It emphasizes explicit send/receive patterns and cleaner multi-turn conversations while maintaining full compatibility with the V1 API.

### Key Benefits

- **Simpler API**: Explicit `send()` and `receive()` methods
- **Cleaner Multi-turn**: More intuitive conversation management
- **Type Safety**: Full Rust type safety with minimal overhead
- **Zero Dependencies**: No breaking changes from V1
- **Performance**: Identical performance to V1 API

### When to Use V2

✅ **Use V2 when**:
- You prefer explicit send/receive patterns
- You want cleaner multi-turn conversation syntax
- You're migrating from TypeScript SDK V2
- You don't need session forking feature

❌ **Use V1 when**:
- You need async generator-style streaming
- You require session forking
- You prefer manual control over query lifecycle
- You're working with existing V1 codebase

---

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
claude-agent-sdk = "0.7.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

#### One-Shot Query

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = prompt(
        "What is 2 + 2?",
        SessionOptions::default()
    ).await?;

    println!("{}", result.message.content);
    // Output: "2 + 2 equals 4."
    Ok(())
}
```

#### Multi-Turn Conversation

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut session = create_session(SessionOptions::default()).await?;

    // Turn 1
    session.send("What is Rust?").await?;
    let messages = session.receive().await?;
    for msg in messages {
        if msg.type_ == "assistant" {
            println!("{}", msg.message.content);
        }
    }

    // Turn 2
    session.send("What are its key features?").await?;
    let messages = session.receive().await?;
    for msg in messages {
        if msg.type_ == "assistant" {
            println!("{}", msg.message.content);
        }
    }

    Ok(())
}
```

---

## Core Concepts

### Session

A `Session` represents a conversation with Claude. It maintains conversation state and handles multi-turn interactions.

**Key Properties**:
- **Stateful**: Maintains conversation history
- **Bidirectional**: Supports both sending and receiving messages
- **Async**: All operations are async for non-blocking behavior

**Lifecycle**:
```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

async fn session_lifecycle() -> anyhow::Result<()> {
    // 1. Create session
    let mut session = create_session(SessionOptions::default()).await?;

    // 2. Send messages
    session.send("Hello!").await?;

    // 3. Receive responses
    let messages = session.receive().await?;

    // 4. Repeat send/receive as needed
    // ...

    // 5. Close session (automatic with Drop)
    session.close().await?;
    Ok(())
}
```

### SessionOptions

Configuration options for creating or resuming sessions.

**Available Options**:

```rust
use claude_agent_sdk::v2::{SessionOptions, PermissionMode};

let options = SessionOptions::builder()
    .model("claude-opus-4")                    // Model to use
    .permission_mode(Some(PermissionMode::BypassPermissions))
    .max_turns(Some(10))                       // Maximum conversation turns
    .max_budget_usd(Some(5.0))                 // Cost limit
    .build();
```

**Option Details**:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `model` | `Option<String>` | `None` | Model to use (e.g., "claude-opus-4", "claude-sonnet-4") |
| `permission_mode` | `Option<PermissionMode>` | `None` | Permission mode for operations |
| `max_turns` | `Option<u32>` | `None` | Maximum number of conversation turns |
| `max_budget_usd` | `Option<f64>` | `None` | Maximum budget in USD |

### Message Types

V2 API uses simplified message structures:

**Response Message**:
```rust
pub struct Message {
    pub type_: String,        // "assistant", "user", "result"
    pub message: MessageContent,
}

pub struct MessageContent {
    pub content: String,      // Text content
    // ... other fields
}
```

**Processing Messages**:
```rust
let messages = session.receive().await?;

for msg in messages {
    match msg.type_.as_str() {
        "assistant" => {
            println!("Claude: {}", msg.message.content);
        }
        "user" => {
            println!("User: {}", msg.message.content);
        }
        "result" => {
            // End of conversation
            break;
        }
        _ => {}
    }
}
```

---

## API Reference

### Functions

#### `prompt()`

One-shot query without maintaining session state.

**Signature**:
```rust
pub async fn prompt(
    message: &str,
    options: SessionOptions
) -> Result<PromptResult, ClaudeError>
```

**Parameters**:
- `message: &str` - The user message to send
- `options: SessionOptions` - Session configuration

**Returns**:
- `Result<PromptResult, ClaudeError>` - Query result or error

**Example**:
```rust
use claude_agent_sdk::v2::{prompt, SessionOptions};

let result = prompt("Explain async/await in Rust", SessionOptions::default()).await?;
println!("Response: {}", result.message.content);
```

#### `create_session()`

Create a new session for multi-turn conversations.

**Signature**:
```rust
pub async fn create_session(
    options: SessionOptions
) -> Result<Session, ClaudeError>
```

**Parameters**:
- `options: SessionOptions` - Session configuration

**Returns**:
- `Result<Session, ClaudeError>` - New session or error

**Example**:
```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

let mut session = create_session(SessionOptions::default()).await?;
session.send("Hello!").await?;
```

#### `resume_session()`

Resume an existing session by ID.

**Signature**:
```rust
pub async fn resume_session(
    session_id: &str,
    options: SessionOptions
) -> Result<Session, ClaudeError>
```

**Parameters**:
- `session_id: &str` - ID of session to resume
- `options: SessionOptions` - Session configuration

**Returns**:
- `Result<Session, ClaudeError>` - Resumed session or error

**Example**:
```rust
use claude_agent_sdk::v2::{resume_session, SessionOptions};

let mut session = resume_session("session-abc123", SessionOptions::default()).await?;
session.send("Continue our discussion").await?;
```

### Methods

#### `Session::send()`

Send a message to Claude.

**Signature**:
```rust
pub async fn send(&mut self, message: &str) -> Result<(), ClaudeError>
```

**Parameters**:
- `message: &str` - Message to send

**Returns**:
- `Result<(), ClaudeError>` - Success or error

**Example**:
```rust
session.send("What is ownership in Rust?").await?;
```

#### `Session::receive()`

Receive messages from Claude.

**Signature**:
```rust
pub async fn receive(&mut self) -> Result<Vec<Message>, ClaudeError>
```

**Returns**:
- `Result<Vec<Message>, ClaudeError>` - Vector of messages or error

**Example**:
```rust
let messages = session.receive().await?;
for msg in messages {
    if msg.type_ == "assistant" {
        println!("{}", msg.message.content);
    }
}
```

#### `Session::close()`

Close the session and release resources.

**Signature**:
```rust
pub async fn close(&mut self) -> Result<(), ClaudeError>
```

**Returns**:
- `Result<(), ClaudeError>` - Success or error

**Note**: Sessions are automatically closed when dropped, but explicit close is recommended for cleanup.

---

## Usage Patterns

### Pattern 1: Simple Query

**Use Case**: One-off questions without conversation context

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions};

async fn simple_query() -> anyhow::Result<()> {
    let result = prompt(
        "What is the capital of France?",
        SessionOptions::default()
    ).await?;

    println!("Answer: {}", result.message.content);
    Ok(())
}
```

### Pattern 2: Multi-Turn Conversation

**Use Case**: Extended conversations with context

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

async fn multi_turn() -> anyhow::Result<()> {
    let mut session = create_session(SessionOptions::default()).await?;

    let questions = vec![
        "What is Rust?",
        "What are its key features?",
        "How does it compare to C++?",
    ];

    for question in questions {
        session.send(question).await?;

        let messages = session.receive().await?;
        for msg in messages {
            if msg.type_ == "assistant" {
                println!("{}\n", msg.message.content);
            }
        }
    }

    Ok(())
}
```

### Pattern 3: Streaming Response

**Use Case**: Processing responses as they arrive

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};
use futures::StreamExt;

async fn streaming_response() -> anyhow::Result<()> {
    let mut session = create_session(SessionOptions::default()).await?;

    session.send("Explain Rust's ownership model").await?;

    let message_stream = session.receive().await?;
    let mut stream = message_stream.into_stream();

    while let Some(result) = stream.next().await {
        let msg = result?;
        if msg.type_ == "assistant" {
            print!("{}", msg.message.content);
            // Flush for immediate display
            use std::io::Write;
            std::io::stdout().flush()?;
        }
    }

    Ok(())
}
```

### Pattern 4: Session Resume

**Use Case**: Resuming previous conversations

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

async fn session_resume() -> anyhow::Result<()> {
    // Create initial session
    let mut session = create_session(SessionOptions::default()).await?;
    session.send("My name is Alice").await?;
    let _ = session.receive().await?;

    let session_id = session.get_id(); // Hypothetical method

    // Later... resume session
    let mut session = create_session(
        SessionOptions::builder()
            .resume(session_id)
            .build()
    ).await?;

    session.send("What's my name?").await?;
    let messages = session.receive().await?;
    // Claude will remember "Alice"

    Ok(())
}
```

### Pattern 5: Error Handling

**Use Case**: Robust error handling

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions, ClaudeError};

async fn with_error_handling() -> Result<String, Box<dyn std::error::Error>> {
    match prompt("Hello", SessionOptions::default()).await {
        Ok(result) => Ok(result.message.content),
        Err(ClaudeError::QueryFailed(msg)) => {
            eprintln!("Query failed: {}", msg);
            Err(msg.into())
        }
        Err(ClaudeError::NetworkError(err)) => {
            eprintln!("Network error: {}", err);
            Err(err.into())
        }
        Err(err) => {
            eprintln!("Unexpected error: {}", err);
            Err(err.into())
        }
    }
}
```

---

## Advanced Topics

### Custom System Prompts

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

let options = SessionOptions::builder()
    .system_prompt(Some(
        "You are a Rust expert. Provide concise, technical answers."
    ))
    .build();

let mut session = create_session(options).await?;
```

### Permission Modes

```rust
use claude_agent_sdk::v2::{SessionOptions, PermissionMode};

// Bypass permissions (for trusted environments)
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::BypassPermissions))
    .build();

// Plan mode (for planning operations)
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::Plan))
    .build();
```

### Budget Control

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

// Limit maximum spend to $1.00
let options = SessionOptions::builder()
    .max_budget_usd(Some(1.0))
    .build();

let mut session = create_session(options).await?;
```

### Turn Limiting

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

// Limit to 5 conversation turns
let options = SessionOptions::builder()
    .max_turns(Some(5))
    .build();

let mut session = create_session(options).await?;
```

---

## Examples

### Example 1: Code Assistant

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut session = create_session(
        SessionOptions::builder()
            .system_prompt(Some(
                "You are a Rust programming expert. \
                 Provide code examples and explanations."
            ))
            .build()
    ).await?;

    let code_problem = r#"
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1;
        println!("{}", s1); // Error: value borrowed
    }
    "#;

    session.send(&format!("Explain this error:\n{}", code_problem)).await?;

    let messages = session.receive().await?;
    for msg in messages {
        if msg.type_ == "assistant" {
            println!("{}", msg.message.content);
        }
    }

    Ok(())
}
```

### Example 2: Translation Service

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions};

async fn translate(text: &str, target_lang: &str) -> anyhow::Result<String> {
    let prompt_text = format!(
        "Translate the following text to {}. Only return the translation:\n\n{}",
        target_lang, text
    );

    let result = prompt(&prompt_text, SessionOptions::default()).await?;
    Ok(result.message.content)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let translation = translate("Hello, world!", "Spanish").await?;
    println!("Translation: {}", translation);
    Ok(())
}
```

### Example 3: Interactive Chatbot

```rust
use claude_agent_sdk::v2::{create_session, SessionOptions};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut session = create_session(SessionOptions::default()).await?;

    println!("Chat with Claude! (Type 'quit' to exit)");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "quit" {
            break;
        }

        session.send(input).await?;

        let messages = session.receive().await?;
        for msg in messages {
            if msg.type_ == "assistant" {
                println!("Claude: {}", msg.message.content);
            }
        }
    }

    Ok(())
}
```

---

## Migrating from V1

### Key Differences

| Aspect | V1 API | V2 API |
|--------|--------|--------|
| **One-shot** | `query(message, options)` | `prompt(message, options)` |
| **Multi-turn** | `ClaudeClient::query()` + `receive_response()` | `Session::send()` + `receive()` |
| **Session** | `ClaudeClient` | `Session` |
| **Options** | `ClaudeAgentOptions` | `SessionOptions` |
| **Streaming** | `query_stream()` | `Session::receive().into_stream()` |

### Migration Example

**V1 Code**:
```rust
use claude_agent_sdk::{ClaudeClient, ClaudeAgentOptions, PermissionMode};

let mut client = ClaudeClient::new(
    ClaudeAgentOptions::builder()
        .permission_mode(PermissionMode::BypassPermissions)
        .build()
);

client.connect().await?;
client.query("Hello!").await?;

let mut stream = client.receive_response();
while let Some(result) = stream.next().await {
    let message = result?;
    // Process message...
}

client.disconnect().await?;
```

**V2 Equivalent**:
```rust
use claude_agent_sdk::v2::{create_session, SessionOptions, PermissionMode};

let mut session = create_session(
    SessionOptions::builder()
        .permission_mode(Some(PermissionMode::BypassPermissions))
        .build()
).await?;

session.send("Hello!").await?;

let messages = session.receive().await?;
for msg in messages {
    // Process message...
}
```

See [MIGRATION_GUIDE.md](../../../MIGRATION_GUIDE.md) for complete migration details.

---

## Best Practices

### 1. Always Handle Errors

```rust
use claude_agent_sdk::v2::{prompt, SessionOptions, ClaudeError};

match prompt("Hello", SessionOptions::default()).await {
    Ok(result) => println!("{}", result.message.content),
    Err(ClaudeError::QueryFailed(e)) => {
        eprintln!("Query failed: {}", e);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

### 2. Set Budget Limits

```rust
let options = SessionOptions::builder()
    .max_budget_usd(Some(5.0)) // Limit to $5
    .build();
```

### 3. Use Appropriate Permission Modes

```rust
// For automation: BypassPermissions
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::BypassPermissions))
    .build();

// For planning: Plan
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::Plan))
    .build();
```

### 4. Close Sessions Explicitly

```rust
let mut session = create_session(SessionOptions::default()).await?;

// ... use session ...

session.close().await?; // Explicit cleanup
```

### 5. Process Messages by Type

```rust
let messages = session.receive().await?;

for msg in messages {
    match msg.type_.as_str() {
        "assistant" => println!("Claude: {}", msg.message.content),
        "user" => println!("User: {}", msg.message.content),
        "result" => break, // End of conversation
        _ => continue,
    }
}
```

---

## Troubleshooting

### Common Issues

#### Issue 1: "Session not connected"

**Cause**: Trying to receive without sending first

**Solution**:
```rust
// WRONG
let mut session = create_session(SessionOptions::default()).await?;
let messages = session.receive().await?; // Error!

// CORRECT
let mut session = create_session(SessionOptions::default()).await?;
session.send("Hello").await?; // Send first
let messages = session.receive().await?;
```

#### Issue 2: "Timeout waiting for response"

**Cause**: Network issues or long-running queries

**Solution**: Increase timeout or check network connectivity

#### Issue 3: "Permission denied"

**Cause**: Insufficient permissions for operation

**Solution**: Use appropriate permission mode:
```rust
let options = SessionOptions::builder()
    .permission_mode(Some(PermissionMode::BypassPermissions))
    .build();
```

### Getting Help

- Check [MIGRATION_GUIDE.md](../../../MIGRATION_GUIDE.md)
- Review [examples/](../../../examples/)
- Open an [issue](https://github.com/louloulin/cc-agent-sdk/issues)

---

**Document Version**: 1.0.0
**Last Updated**: 2026-01-13
**Maintainer**: Loulou Lin
