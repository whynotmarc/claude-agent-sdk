# Subagent System Complete Guide

**Version**: 1.0.0
**Last Updated**: 2026-01-13
**SDK Version**: 0.7.0+

---

## Table of Contents

1. [Introduction](#introduction)
2. [Core Concepts](#core-concepts)
3. [Quick Start](#quick-start)
4. [API Reference](#api-reference)
5. [Usage Patterns](#usage-patterns)
6. [Advanced Topics](#advanced-topics)
7. [Best Practices](#best-practices)
8. [Examples](#examples)
9. [Troubleshooting](#troubleshooting)

---

## Introduction

The Subagent system enables task delegation and specialization within the Claude Agent SDK. It allows you to create specialized agents with specific skills, instructions, and tool access, then delegate tasks to them automatically or manually.

### What are Subagents?

**Subagents** are specialized Claude instances with:
- **Specific Instructions**: Custom prompts for specialized tasks
- **Restricted Tools**: Limited tool access for security
- **Independent Context**: Separate conversation state
- **Delegation Control**: Automatic or manual task routing

### Key Benefits

- **Specialization**: Create experts for specific domains
- **Security**: Restrict tool access per subagent
- **Modularity**: Break complex tasks into smaller pieces
- **Testability**: Test subagents independently
- **Reusability**: Share subagents across sessions

### When to Use Subagents

✅ **Use Subagents when**:
- You have distinct task domains (e.g., code review, documentation, testing)
- Different security requirements per task
- Complex workflows requiring specialization
- You want to parallelize independent tasks
- You need clear separation of concerns

❌ **Don't use Subagents when**:
- Simple one-shot queries suffice
- Tasks are highly interdependent
- Overhead isn't justified
- Single agent can handle all tasks

---

## Core Concepts

### Subagent Structure

```rust
pub struct Subagent {
    pub name: String,              // Unique identifier
    pub description: String,       // Human-readable description
    pub instructions: String,      // System instructions
    pub allowed_tools: Vec<String>, // Whitelisted tools
    pub max_turns: Option<u32>,    // Turn limit
    pub model: Option<String>,     // Model override
}
```

### Delegation Strategies

```rust
pub enum DelegationStrategy {
    Auto,     // Claude automatically decides when to delegate
    Manual,   // Requires explicit SubagentTool calls
    ToolCall, // Delegate through tool calls
}
```

**Strategy Comparison**:

| Strategy | Control | Flexibility | Use Case |
|----------|---------|-------------|----------|
| `Auto` | Low | High | General purpose, trust Claude's judgment |
| `Manual` | High | Low | Precise control, debugging |
| `ToolCall` | Medium | Medium | Tool-based delegation |

### Subagent Execution Flow

```
Main Agent
    ↓
[Delegation Decision]
    ↓
SubagentExecutor.execute(name, input)
    ↓
[Create Subsession]
    ↓
[Execute with Instructions]
    ↓
[Return Output]
    ↓
Main Agent integrates result
```

---

## Quick Start

### Basic Example

```rust
use claude_agent_sdk::subagents::{Subagent, SubagentExecutor, SubagentError};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create subagent
    let code_reviewer = Subagent::builder()
        .name("code_reviewer")
        .description("Expert code reviewer")
        .instructions("Review code for bugs, style issues, and improvements. Be concise and constructive.")
        .allowed_tools(vec!["read_file".to_string(), "search_code".to_string()])
        .build();

    // Create executor
    let mut executor = SubagentExecutor::new();

    // Register subagent
    executor.register(code_reviewer)?;

    // Execute task
    let result = executor.execute(
        "code_reviewer",
        "Review this function: fn add(a: i32, b: i32) -> i32 { a + b }"
    ).await?;

    println!("Review: {}", result.messages.join("\n"));
    Ok(())
}
```

---

## API Reference

### Subagent

#### `new()`

Create a new subagent.

```rust
let subagent = Subagent::new(
    "reviewer".to_string(),
    "Code review expert".to_string(),
    "Review code carefully".to_string(),
    vec!["read_file".to_string()],
    None,  // max_turns
    None,  // model
);
```

#### `builder()`

Create subagent using builder pattern (recommended).

```rust
let subagent = Subagent::builder()
    .name("reviewer")
    .description("Code review expert")
    .instructions("Review code carefully")
    .allowed_tools(vec!["read_file".to_string()])
    .max_turns(Some(5))
    .model(Some("claude-opus-4"))
    .build();
```

**Builder Methods**:

| Method | Type | Required | Default |
|--------|------|----------|---------|
| `name()` | `String` | Yes | - |
| `description()` | `String` | Yes | - |
| `instructions()` | `String` | Yes | - |
| `allowed_tools()` | `Vec<String>` | Yes | - |
| `max_turns()` | `Option<u32>` | No | `None` |
| `model()` | `Option<String>` | No | `None` |

### SubagentExecutor

#### `new()`

Create a new executor.

```rust
let executor = SubagentExecutor::new();
```

#### `register()`

Register a subagent.

```rust
executor.register(subagent)?;
```

**Errors**: `SubagentError::AlreadyExists` if name is duplicate

#### `execute()`

Execute a task with a subagent.

```rust
let result = executor.execute(
    "subagent_name",
    "task input"
).await?;
```

**Parameters**:
- `name: &str` - Subagent name
- `input: &str` - Task description

**Returns**: `Result<SubagentOutput, SubagentError>`

**Errors**:
- `NotFound`: Subagent doesn't exist
- `ExecutionFailed`: Task execution failed
- `InvalidInput`: Invalid input format

#### `execute_with_config()`

Execute with custom configuration.

```rust
let result = executor.execute_with_config(
    "subagent_name",
    "task input",
    Some("claude-opus-4"),
    Some(10)
).await?;
```

#### `list_subagents()`

Get all registered subagent names.

```rust
let names = executor.list_subagents();
// => ["code_reviewer", "doc_writer", "test_generator"]
```

#### `has_subagent()`

Check if subagent exists.

```rust
if executor.has_subagent("code_reviewer") {
    // ...
}
```

### SubagentConfig

Manages multiple subagents and delegation strategy.

```rust
use claude_agent_sdk::subagents::{SubagentConfig, DelegationStrategy};

let mut config = SubagentConfig::new(DelegationStrategy::Auto);

config.add_subagent(code_reviewer)?;
config.add_subagent(doc_writer)?;

let subagent = config.get_subagent("code_reviewer")?;
```

---

## Usage Patterns

### Pattern 1: Manual Delegation

**Use Case**: Precise control over which subagent handles which task

```rust
use claude_agent_sdk::subagents::{Subagent, SubagentExecutor};

async fn manual_delegation() -> anyhow::Result<()> {
    let reviewer = Subagent::builder()
        .name("reviewer")
        .description("Code reviewer")
        .instructions("Review code for bugs")
        .allowed_tools(vec![])
        .build();

    let doc_writer = Subagent::builder()
        .name("doc_writer")
        .description("Documentation writer")
        .instructions("Write clear documentation")
        .allowed_tools(vec!["read_file".to_string()])
        .build();

    let mut executor = SubagentExecutor::new();
    executor.register(reviewer)?;
    executor.register(doc_writer)?;

    // Manually choose subagent based on task type
    let task = "Review this code: ...";
    let subagent_name = if task.contains("review") {
        "reviewer"
    } else if task.contains("document") {
        "doc_writer"
    } else {
        return Err(anyhow::anyhow!("Unknown task type"));
    };

    let result = executor.execute(subagent_name, task).await?;
    println!("{}", result.messages.join("\n"));

    Ok(())
}
```

### Pattern 2: Parallel Execution

**Use Case**: Run independent tasks concurrently

```rust
use futures::future::join_all;

async fn parallel_execution() -> anyhow::Result<()> {
    let mut executor = SubagentExecutor::new();

    // Register subagents
    executor.register(test_agent)?;
    executor.register(doc_agent)?;
    executor.register(review_agent)?;

    // Define tasks
    let tasks = vec![
        ("test_agent", "Write tests for auth module"),
        ("doc_agent", "Document API endpoints"),
        ("review_agent", "Review PR #123"),
    ];

    // Execute in parallel
    let futures = tasks.into_iter().map(|(name, input)| {
        executor.execute(name, input)
    }).collect::<Vec<_>>();

    let results = join_all(futures).await;

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(output) => println!("Task {}: {}", i, output.messages.join("\n")),
            Err(e) => eprintln!("Task {} failed: {}", i, e),
        }
    }

    Ok(())
}
```

### Pattern 3: Chained Execution

**Use Case**: Pass output of one subagent to another

```rust
async fn chained_execution() -> anyhow::Result<()> {
    let mut executor = SubagentExecutor::new();

    executor.register(code_generator)?;
    executor.register(code_reviewer)?;

    // Step 1: Generate code
    let gen_result = executor.execute(
        "code_generator",
        "Write a function to validate email addresses"
    ).await?;

    let generated_code = gen_result.messages.join("\n");

    // Step 2: Review generated code
    let review_result = executor.execute(
        "code_reviewer",
        &format!("Review this code:\n{}", generated_code)
    ).await?;

    println!("Code:\n{}", generated_code);
    println!("\nReview:\n{}", review_result.messages.join("\n"));

    Ok(())
}
```

### Pattern 4: Specialized Subagents

**Use Case**: Domain-specific experts

```rust
async fn specialized_subagents() -> anyhow::Result<()> {
    let mut executor = SubagentExecutor::new();

    // Security expert
    let security_expert = Subagent::builder()
        .name("security_expert")
        .description("Security code reviewer")
        .instructions(
            "Review code for security vulnerabilities including: \
             SQL injection, XSS, CSRF, authentication issues, \
             authorization flaws, and sensitive data exposure. \
             Provide specific fixes."
        )
        .allowed_tools(vec!["read_file".to_string(), "search_code".to_string()])
        .build();

    // Performance expert
    let performance_expert = Subagent::builder()
        .name("performance_expert")
        .description("Performance optimization expert")
        .instructions(
            "Analyze code for performance issues including: \
             algorithmic complexity, memory usage, I/O operations, \
             database queries, and caching opportunities. \
             Suggest specific optimizations."
        )
        .allowed_tools(vec!["read_file".to_string()])
        .build();

    executor.register(security_expert)?;
    executor.register(performance_expert)?;

    // Run both analyses
    let code = r#"
    fn process_data(data: Vec<String>) -> Vec<String> {
        let mut results = Vec::new();
        for item in data {
            results.push(item.to_uppercase());
        }
        results
    }
    "#;

    let security_review = executor.execute("security_expert", code).await?;
    let performance_review = executor.execute("performance_expert", code).await?;

    println!("Security Review:\n{}", security_review.messages.join("\n"));
    println!("\nPerformance Review:\n{}", performance_review.messages.join("\n"));

    Ok(())
}
```

### Pattern 5: Error Recovery

**Use Case**: Handle subagent failures gracefully

```rust
use claude_agent_sdk::subagents::SubagentError;

async fn error_recovery() -> anyhow::Result<()> {
    let mut executor = SubagentExecutor::new();
    executor.register(subagent)?;

    match executor.execute("subagent", "task").await {
        Ok(output) => {
            println!("Success: {}", output.messages.join("\n"));
        }
        Err(SubagentError::NotFound(name)) => {
            eprintln!("Subagent '{}' not found", name);
            // Fallback to main agent
        }
        Err(SubagentError::ExecutionFailed(msg)) => {
            eprintln!("Execution failed: {}", msg);
            // Retry with different approach
        }
        Err(SubagentError::InvalidInput(msg)) => {
            eprintln!("Invalid input: {}", msg);
            // Rephrase and retry
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
    }

    Ok(())
}
```

---

## Advanced Topics

### Tool Whitelisting

Restrict subagent access to specific tools:

```rust
let subagent = Subagent::builder()
    .name("file_reader")
    .description("File reading specialist")
    .instructions("Read and summarize files")
    .allowed_tools(vec![
        "read_file".to_string(),
        "list_directory".to_string(),
    ])
    .build();
```

**Security Note**: Always use tool whitelisting in production.

### Turn Limiting

Prevent subagents from running indefinitely:

```rust
let subagent = Subagent::builder()
    .name("quick_reviewer")
    .max_turns(Some(3)) // Maximum 3 turns
    .build();
```

### Model Selection

Use different models for different subagents:

```rust
let simple_agent = Subagent::builder()
    .name("summarizer")
    .model(Some("claude-haiku-4")) // Fast, cheap
    .build();

let complex_agent = Subagent::builder()
    .name("analyst")
    .model(Some("claude-opus-4")) // Powerful, expensive
    .build();
```

### Cost Control

Combine turn limits and model selection:

```rust
let budget_agent = Subagent::builder()
    .name("budget_agent")
    .model(Some("claude-haiku-4")) // Cheapest model
    .max_turns(Some(2))            // Minimal turns
    .build();
```

---

## Best Practices

### 1. Clear Instructions

✅ **Good**:
```rust
.instructions(
    "Review code for SQL injection vulnerabilities. \
     Check: 1) User input handling, 2) Query construction, \
     3) Parameter usage. Provide specific line numbers and fixes."
)
```

❌ **Bad**:
```rust
.instructions("Check security")
```

### 2. Specific Tool Access

✅ **Good**:
```rust
.allowed_tools(vec![
    "read_file".to_string(),
    "search_code".to_string(),
])
```

❌ **Bad**:
```rust
.allowed_tools(vec![]) // Too restrictive
// OR
.allowed_tools(vec!["*".to_string()]) // Too permissive
```

### 3. Descriptive Names

✅ **Good**:
```rust
.name("security_reviewer")
.description("Reviews code for security vulnerabilities")
```

❌ **Bad**:
```rust
.name("agent1")
.description("Does stuff")
```

### 4. Set Resource Limits

```rust
let subagent = Subagent::builder()
    .max_turns(Some(5))  // Prevent infinite loops
    .model(Some("claude-sonnet-4")) // Balance cost/performance
    .build();
```

### 5. Error Handling

```rust
match executor.execute("agent", "task").await {
    Ok(output) => { /* handle success */ }
    Err(SubagentError::NotFound(name)) => { /* handle not found */ }
    Err(SubagentError::ExecutionFailed(e)) => { /* handle failure */ }
    Err(e) => { /* handle other errors */ }
}
```

---

## Examples

### Example 1: CI/CD Pipeline

```rust
use claude_agent_sdk::subagents::{Subagent, SubagentExecutor};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut executor = SubagentExecutor::new();

    // Linter
    let linter = Subagent::builder()
        .name("linter")
        .description("Code linter")
        .instructions("Check code style and conventions")
        .allowed_tools(vec!["read_file".to_string()])
        .build();

    // Test generator
    let test_gen = Subagent::builder()
        .name("test_generator")
        .description("Test generator")
        .instructions("Generate comprehensive unit tests")
        .allowed_tools(vec!["read_file".to_string()])
        .build();

    // Documentation
    let doc_writer = Subagent::builder()
        .name("doc_writer")
        .description("Documentation writer")
        .instructions("Write clear documentation")
        .allowed_tools(vec!["read_file".to_string()])
        .build();

    executor.register(linter)?;
    executor.register(test_gen)?;
    executor.register(doc_writer)?;

    // Run CI/CD pipeline
    let file_path = "src/auth.rs";

    println!("=== Linting ===");
    let lint_result = executor.execute("linter",
        &format!("Lint this file: {}", file_path)
    ).await?;
    println!("{}", lint_result.messages.join("\n"));

    println!("\n=== Generating Tests ===");
    let test_result = executor.execute("test_generator",
        &format!("Generate tests for: {}", file_path)
    ).await?;
    println!("{}", test_result.messages.join("\n"));

    println!("\n=== Writing Documentation ===");
    let doc_result = executor.execute("doc_writer",
        &format!("Document this file: {}", file_path)
    ).await?;
    println!("{}", doc_result.messages.join("\n"));

    Ok(())
}
```

### Example 2: Multi-Language Support

```rust
async fn multi_language_support() -> anyhow::Result<()> {
    let mut executor = SubagentExecutor::new();

    // Rust expert
    let rust_expert = Subagent::builder()
        .name("rust_expert")
        .description("Rust programming expert")
        .instructions("Provide expert Rust code and explanations")
        .allowed_tools(vec![])
        .build();

    // Python expert
    let python_expert = Subagent::builder()
        .name("python_expert")
        .description("Python programming expert")
        .instructions("Provide expert Python code and explanations")
        .allowed_tools(vec![])
        .build();

    executor.register(rust_expert)?;
    executor.register(python_expert)?;

    let task = "Implement a function to validate email addresses";

    // Get implementations in both languages
    let rust_impl = executor.execute("rust_expert",
        &format!("{} in Rust", task)
    ).await?;

    let python_impl = executor.execute("python_expert",
        &format!("{} in Python", task)
    ).await?;

    println!("Rust Implementation:\n{}", rust_impl.messages.join("\n"));
    println!("\nPython Implementation:\n{}", python_impl.messages.join("\n"));

    Ok(())
}
```

---

## Troubleshooting

### Common Issues

#### Issue 1: "Subagent not found"

**Cause**: Misspelled name or not registered

**Solution**:
```rust
// Check if registered
if !executor.has_subagent("my_agent") {
    executor.register(my_agent)?;
}

// Use correct name
executor.execute("my_agent", "task").await?; // Correct spelling
```

#### Issue 2: "Execution failed"

**Cause**: Invalid instructions or input

**Solution**:
```rust
// Verify instructions are clear
let agent = Subagent::builder()
    .instructions("Specific, actionable instructions") // Not vague
    .build();

// Verify input format
let result = executor.execute("agent", "Clear task description").await?;
```

#### Issue 3: Subagent runs too long

**Cause**: No turn limit set

**Solution**:
```rust
let agent = Subagent::builder()
    .max_turns(Some(5)) // Limit turns
    .build();
```

### Getting Help

- Check [examples/](../../../examples/)
- Review [tests/](../../../tests/)
- Open an [issue](https://github.com/louloulin/claude-agent-sdk-rs/issues)

---

**Document Version**: 1.0.0
**Last Updated**: 2026-01-13
**Maintainer**: Loulou Lin
