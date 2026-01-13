# Architecture Overview

This document provides a high-level overview of the Claude Agent SDK Rust architecture and its investment analysis platform.

## Table of Contents

- [System Architecture](#system-architecture)
- [Core Components](#core-components)
- [Design Principles](#design-principles)
- [Technology Stack](#technology-stack)
- [Data Flow](#data-flow)

---

## System Architecture

### Layered Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Layer                       │
│  (investintel-agent: Investment Analysis Platform)           │
│  • Agents: Value, Portfolio, Trading, Dividend, Kelly, Munger│
│  • Skills: 25+ modular analysis skills                      │
│  • Orchestration: Multi-agent coordination                  │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                      SDK Core Layer                          │
│  (claude-agent-sdk: Claude Code CLI Integration)            │
│  • ClaudeClient: Bidirectional streaming                   │
│  • Query API: Simple one-shot queries                      │
│  • Hooks System: Behavior interception                     │
│  • MCP Gateway: Tool integration                            │
│  • Skills System: Skill discovery and execution            │
│  • Orchestration: Agent coordination                        │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    Transport Layer                           │
│  • SubprocessTransport: Claude Code CLI communication       │
│  • Protocol: Control protocol with bidirectional streaming │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                  External Dependencies                        │
│  • Claude Code CLI (subprocess)                            │
│  • Data Sources (Yahoo Finance, Alpha Vantage, WebSocket)   │
│  • Trading APIs (Binance, OKX)                             │
│  • Storage (libSQL)                                        │
└─────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. Claude Client (`ClaudeClient`)

**Purpose**: Main client for bidirectional streaming communication with Claude.

**Key Features**:
- Async message streaming
- Session management
- Dynamic control (interrupt, change permissions)
- Hook integration
- Tool use monitoring

**Location**: `crates/claude-agent-sdk/src/client/`

### 2. Query API

**Purpose**: Simple one-shot query interface.

**Components**:
- `query()` - Collect all messages
- `query_stream()` - Stream messages (memory-efficient)
- `query_with_content()` - Custom content blocks
- `query_stream_with_content()` - Stream custom content

**Location**: `crates/claude-agent-sdk/src/query/`

### 3. Hooks System

**Purpose**: Intercept and control Claude's behavior.

**Hook Types**:
- `PreToolUse` - Before tool execution
- `PostToolUse` - After tool execution
- `PermissionRequest` - Permission decisions

**Location**: `crates/claude-agent-sdk/src/hooks/`

### 4. MCP Gateway

**Purpose**: Unified tool and data source management.

**Features**:
- MCP server management
- Tool discovery
- Smart routing
- Caching layer
- Quality metrics

**Location**: `apps/investintel-agent/src/mcp/`

### 5. Skills System

**Purpose**: Modular skill discovery and execution.

**Components**:
- Skill registry
- Skill discovery (`.skill.md` files)
- Skill executor
- Skill integration

**Location**: `crates/claude-agent-sdk/src/skills/`

### 6. Orchestration System

**Purpose**: Multi-agent coordination.

**Patterns**:
- Parallel orchestration
- Sequential orchestration
- Hierarchical orchestration
- Result aggregation

**Location**: `crates/claude-agent-sdk/src/orchestration/`

---

## Design Principles

### 1. Type Safety

All public APIs use strong Rust types:

```rust
// Messages
pub enum Message {
    Assistant(AssistantMessage),
    User(UserMessage),
    System(SystemMessage),
    Result(ResultMessage),
}

// Configuration
pub struct ClaudeAgentOptions {
    pub model: Option<String>,
    pub max_turns: Option<u32>,
    pub allowed_tools: Vec<String>,
    // ...
}
```

### 2. Async-First

All I/O operations are async using Tokio:

```rust
pub async fn connect(&mut self) -> Result<()>
pub async fn query(&mut self, query: &str) -> Result<()>
pub async fn receive_message(&mut self) -> Result<Option<Message>>
```

### 3. Zero-Copy Streaming

Stream messages without buffering:

```rust
pub fn query_stream(...) -> impl Stream<Item = Result<Message>> {
    // O(1) memory usage
}
```

### 4. Lock-Free Architecture

Avoid deadlocks with careful ownership design:

```rust
pub struct ClaudeClient {
    transport: Arc<Mutex<SubprocessTransport>>,
    // No RwLock to prevent deadlocks
}
```

### 5. Modular Design

Clear separation of concerns:
- Client layer
- Query layer
- Transport layer
- Hooks layer
- Tools layer

---

## Technology Stack

### Core Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| **tokio** | Async runtime | 1.49 |
| **serde** | Serialization | 1.0 |
| **serde_json** | JSON serialization | 1.0 |
| **anyhow** | Error handling | 0.11 |
| **async-trait** | Async traits | 0.1 |
| **futures** | Stream utilities | 0.3 |
| **chrono** | Date/time | 0.4 |
| **tracing** | Logging | 0.1 |

### Investment Platform Dependencies

| Crate | Purpose |
|-------|---------|
| **yahoo-finance-api** | Yahoo Finance data |
| **alpha_vantage** | Alpha Vantage data |
| **tokio-tungstenite** | WebSocket support |
| **rusqlite** | libSQL storage |
| **uuid** | Unique identifiers |
| **thiserror** | Error derivation |

---

## Data Flow

### Query Flow (Simple)

```
User Code
   │
   ├─> query(prompt, options)
   │       │
   │       ├─> Create ClaudeClient
   │       │       │
   │       ├─> Connect to Claude Code CLI
   │       │       │
   │       ├─> Send prompt
   │       │       │
   │       ├─> Receive messages (stream)
   │       │       │
   │       └─> Collect messages
   │               │
   └───────────────> Return Vec<Message>
```

### Bidirectional Flow

```
User Code
   │
   ├─> client.connect()
   │
   ├─> client.query("...")
   │       │
   │       ├─> Send to Claude
   │       │
   │       ├─> Loop: receive_message()
   │       │       │
   │       │       ├─> Message::Assistant
   │       │       ├─> Message::User
   │       │       ├─> Message::ToolUse
   │       │       ├─> Message::Result (stop)
   │       │       └─> ...
   │
   ├─> client.interrupt() (optional)
   │
   └─> client.disconnect()
```

### Agent Orchestration Flow

```
User Request
   │
   ├─> InvestmentOrchestrator
   │       │
   │       ├─> Spawn Agents (parallel)
   │       │       ├─> ValueInvestmentAgent
   │       │       ├─> TradingAdvisorAgent
   │       │       └─> PortfolioManagerAgent
   │       │
   │       ├─> Collect Results
   │       │
   │       └─> Aggregate & Return
   │
   └───────────────> InvestmentRecommendation
```

---

## Component Interaction

### ClaudeClient + Hooks

```
ClaudeClient
   │
   ├─> hooks: HashMap<String, Vec<HookMatcher>>
   │
   ├─> Before Tool Use
   │       │
   │       ├─> Match Hook
   │       │       │
   │       ├─> Execute Hook Callback
   │       │       │
   │       └─> Modify/Block Tool Use
   │
   └─> Continue Tool Execution
```

### Skills System

```
SkillDiscovery
   │
   ├─> Scan .claude/skills/
   │       │
   │       ├─> Parse *.skill.md
   │       │
   │       ├─> Register Skills
   │       │
   │       └─> Load Metadata
   │
   ├─> SkillsExecutor
   │       │
   │       ├─> execute_skill(name, input)
   │       │
   │       ├─> Find Skill
   │       │
   │       ├─> Parse Tool Arguments
   │       │
   │       └─> Execute Skill Logic
   │
   └─> Return ToolResult
```

---

## Performance Considerations

### Memory Management

- **Streaming**: Use `query_stream()` for large conversations
- **Zero-copy**: Avoid unnecessary buffering
- **Arc usage**: Share large data structures

### Concurrency

- **Lock-free**: Minimize mutex usage
- **Async**: Use Tokio for I/O-bound operations
- **Parallel agents**: Run agents concurrently

### Caching

- **Data source cache**: LRU cache for market data
- **Skill cache**: Cache parsed skill metadata
- **MCP connection pooling**: Reuse MCP connections

---

## Security Considerations

### Permission Management

```rust
pub enum PermissionMode {
    BypassPermissions,      // Auto-accept all
    AcceptEdits,            // Auto-accept edits
    ManualCallbacks,        // Custom callbacks
    DenyAll,                // Block all
}
```

### Tool Use Control

- **Explicit allowlist**: Only allowed tools can be used
- **Hooks**: Pre/post hooks for tool monitoring
- **Isolation**: Tools run in controlled environment

### API Keys

- Environment variables
- Configuration files
- Never hardcode credentials

---

## Extension Points

### Custom Transport

Implement `Transport` trait for custom Claude integration:

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
    async fn send(&mut self, data: &[u8]) -> Result<()>;
    async fn receive(&mut self) -> Result<Vec<u8>>;
    async fn disconnect(&mut self) -> Result<()>;
}
```

### Custom Tools

Create custom tools with `tool!` macro:

```rust
let tool = tool!(
    name: "my_tool",
    description: "Does something",
    input_schema: json!({...}),
    handler: my_handler
);
```

### Custom Agents

Implement `Agent` trait:

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    async fn execute(&self, input: AgentInput) -> Result<AgentOutput>;
}
```

---

## Monitoring & Observability

### Logging

```rust
use tracing::{info, warn, error};

#[tracing::instrument]
pub async fn query(...) {
    info!("Starting query");
    // ...
}
```

### Metrics

- Tool execution count
- Agent execution time
- Cache hit rate
- API call latency

### Debugging

Enable debug mode:

```rust
let options = ClaudeAgentOptions {
    stderr_callback: Some(Arc::new(|msg| {
        eprintln!("DEBUG: {}", msg);
    })),
    ..Default::default()
};
```

---

## Related Documentation

- [Agent System Details](agents.md)
- [Orchestration System](orchestration.md)
- [Skills System](skills.md)
- [MCP Integration](../api/mcp-integration.md)

---

**Last Updated**: 2026-01-12
