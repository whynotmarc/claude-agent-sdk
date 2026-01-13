# Public API Reference

Complete API reference for Claude Agent SDK Rust.

## Table of Contents

- [Core API](#core-api)
- [Query API](#query-api)
- [Client API](#client-api)
- [Configuration](#configuration)
- [Hooks API](#hooks-api)
- [Tools API](#tools-api)
- [Types](#types)

---

## Core API

### ClaudeClient

Main client for bidirectional streaming communication with Claude.

#### Location
`crates/claude-agent-sdk/src/client/mod.rs`

#### Example

```rust
use claude_agent_sdk::{ClaudeClient, ClaudeAgentOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = ClaudeClient::new(ClaudeAgentOptions::default());
    client.connect().await?;

    client.query("Hello, Claude!").await?;

    loop {
        match client.receive_message().await? {
            Some(Message::Assistant(msg)) => {
                // Handle assistant message
            }
            Some(Message::Result(_)) => break,
            None => break,
            _ => continue,
        }
    }

    client.disconnect().await?;
    Ok(())
}
```

#### Methods

| Method | Description |
|--------|-------------|
| `new(options)` | Create new client |
| `connect(&mut self)` | Connect to Claude |
| `query(&mut self, prompt)` | Send query |
| `receive_message(&mut self)` | Receive next message |
| `interrupt(&mut self)` | Interrupt current operation |
| `disconnect(&mut self)` | Disconnect from Claude |
| `new_session(&mut self, id, prompt)` | Start new session |

---

## Query API

Simple one-shot query functions.

### query

Collect all messages from a conversation.

```rust
use claude_agent_sdk::query;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let messages = query("What is 2 + 2?", None).await?;

    for message in messages {
        println!("{:?}", message);
    }

    Ok(())
}
```

**Signature**:
```rust
pub async fn query(
    prompt: &str,
    options: Option<ClaudeAgentOptions>
) -> Result<Vec<Message>>
```

### query_stream

Stream messages for memory efficiency.

```rust
use claude_agent_sdk::{query_stream, Message};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = query_stream("Explain Rust", None).await?;

    while let Some(result) = stream.next().await {
        let message = result?;
        // Process message
    }

    Ok(())
}
```

**Signature**:
```rust
pub async fn query_stream(
    prompt: &str,
    options: Option<ClaudeAgentOptions>
) -> Result<impl Stream<Item = Result<Message>>>
```

---

## Configuration

### ClaudeAgentOptions

Main configuration for Claude interactions.

```rust
use claude_agent_sdk::{ClaudeAgentOptions, PermissionMode, SystemPrompt};

let options = ClaudeAgentOptions {
    // Model Selection
    model: Some("claude-sonnet-4-5".to_string()),
    fallback_model: Some("claude-haiku-4-5".to_string()),

    // Conversation Control
    max_turns: Some(10),
    max_tokens: Some(4096),

    // Permissions
    permission_mode: Some(PermissionMode::AcceptEdits),
    allowed_tools: vec!["Read".to_string(), "Write".to_string()],

    // System Prompt
    system_prompt: Some(SystemPrompt {
        text: "You are a helpful assistant.".to_string(),
        ..Default::default()
    }),

    // Hooks
    hooks: Some(hooks_map),

    // MCP Servers
    mcp_servers: Some(McpServers::Dict(servers)),

    // Budget Control
    max_budget_usd: Some(10.0),

    // Extended Thinking
    max_thinking_tokens: Some(20000),

    ..Default::default()
};
```

### Builder Pattern

```rust
let options = ClaudeAgentOptions::builder()
    .model("claude-sonnet-4-5")
    .max_turns(5)
    .permission_mode(PermissionMode::AcceptEdits)
    .allowed_tools(vec!["Read".to_string()])
    .build();
```

### Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `model` | `Option<String>` | `None` | Model name |
| `fallback_model` | `Option<String>` | `None` | Fallback model |
| `max_turns` | `Option<u32>` | `None` | Max conversation turns |
| `max_tokens` | `Option<u32>` | `None` | Max tokens per response |
| `permission_mode` | `Option<PermissionMode>` | `None` | Permission handling |
| `allowed_tools` | `Vec<String>` | `[]` | Allowed tool names |
| `system_prompt` | `Option<SystemPromptConfig>` | `None` | System prompt |
| `hooks` | `Option<HashMap<String, Vec<HookMatcher>>>` | `None` | Hooks |
| `mcp_servers` | `Option<McpServers>` | `None` | MCP servers |
| `max_budget_usd` | `Option<f64>` | `None` | Budget limit |
| `max_thinking_tokens` | `Option<u32>` | `None` | Thinking limit |
| `extra_args` | `Option<HashMap<String, Option<String>>>` | `None` | Extra CLI args |

---

## Hooks API

### Hook Types

```rust
pub enum Hook {
    PreToolUse(Arc<dyn Fn(HookInput) -> Result<HookOutput>>),
    PostToolUse(Arc<dyn Fn(HookInput) -> Result<HookOutput>>),
    PermissionRequest(Arc<dyn Fn(PermissionInput) -> Result<PermissionOutput>>),
}
```

### HookMatcher

```rust
pub struct HookMatcher {
    pub matcher: Option<String>,  // Tool name pattern
    pub hooks: Vec<Hook>,
}
```

### Example

```rust
use claude_agent_sdk::{Hook, HookMatcher, HookInput, HookOutput};
use std::sync::Arc;

async fn my_hook(input: HookInput) -> anyhow::Result<HookOutput> {
    // Intercept and potentially modify tool use
    Ok(HookOutput::default())
}

let mut hooks = HashMap::new();
hooks.insert("PreToolUse".to_string(), vec![
    HookMatcher {
        matcher: Some("Bash".to_string()),
        hooks: vec![Hook::new(Arc::new(my_hook))],
    }
]);
```

---

## Tools API

### Create Tool

```rust
use claude_agent_sdk::{tool, ToolResult, McpToolResultContent};

async fn greet_handler(args: serde_json::Value) -> anyhow::Result<ToolResult> {
    let name = args["name"].as_str().unwrap_or("World");
    Ok(ToolResult {
        content: vec![McpToolResultContent::Text {
            text: format!("Hello, {}!", name),
        }],
        is_error: false,
    })
}

let greet_tool = tool!(
    name: "greet",
    description: "Greet a user",
    input_schema: json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"}
        },
        "required": ["name"]
    }),
    handler: greet_handler
);
```

### MCP Server

```rust
use claude_agent_sdk::create_sdk_mcp_server;

let server = create_sdk_mcp_server(
    "my-server",      // Server name
    "1.0.0",           // Version
    vec![greet_tool]  // Tools
);
```

---

## Types

### Message

```rust
pub enum Message {
    User(UserMessage),
    Assistant(AssistantMessage),
    System(SystemMessage),
    Result(ResultMessage),
}
```

### ContentBlock

```rust
pub enum ContentBlock {
    Text(TextBlock),
    ToolUse(ToolUseBlock),
    ToolResult(ToolResultBlock),
    Thinking(ThinkingBlock),
}
```

### PermissionMode

```rust
pub enum PermissionMode {
    BypassPermissions,  // Auto-accept all
    AcceptEdits,         // Auto-accept edits
    ManualCallbacks,     // Custom callbacks
    DenyAll,            // Block all
}
```

---

## Investment Analysis API

### InvestmentAssistant

Main investment analysis coordinator.

```rust
use investintel_agent::InvestmentAssistant;

let assistant = InvestmentAssistant::new();

// Analyze stock
let analysis = assistant.analyze_stock("AAPL").await?;

// Chat interface
let response = assistant.chat("Analyze MSFT").await?;
```

### Agents

#### ValueInvestmentAgent

Graham-Buffett value investment analysis.

```rust
use investintel_agent::ValueInvestmentAgent;

let agent = ValueInvestmentAgent::new();
let input = AgentInput::new("AAPL");
let output = agent.execute(input).await?;
```

#### PortfolioManagerAgent

Portfolio management and optimization.

```rust
use investintel_agent::PortfolioManagerAgent;

let agent = PortfolioManagerAgent::new();
let input = AgentInput::new(vec!["AAPL", "MSFT"]);
let output = agent.execute(input).await?;
```

#### TradingAdvisorAgent

Trading advice and position sizing.

```rust
use investintel_agent::TradingAdvisorAgent;

let agent = TradingAdvisorAgent::new();
let input = AgentInput::new("AAPL");
let output = agent.execute(input).await?;
```

### Backtesting

```rust
use investintel_agent::backtest::{
    BacktestEngine, BacktestConfig, BacktestDataset,
    GrahamStrategy, DateRange
};

// Create configuration
let config = BacktestConfig {
    initial_capital: 100_000.0,
    date_range: DateRange::new("2023-01-01", "2023-12-31").unwrap(),
    ..Default::default()
};

// Create dataset
let dataset = BacktestDataset::generate_mock_data(
    vec!["AAPL"],
    &config.date_range,
    150.0
);

// Run backtest
let engine = BacktestEngine::new(config, dataset);
let strategy = GrahamStrategy::new();
let result = engine.run(&strategy)?;
```

---

## Data Sources

### Market Data Provider

```rust
use investintel_agent::market_data::{get_realtime_quote, get_fundamental_data};

// Get real-time quote
let quote = get_realtime_quote("AAPL").await?;

// Get fundamental data
let fundamental = get_fundamental_data("AAPL").await?;
```

### WebSocket Streaming

```rust
use investintel_agent::data::websocket_enhanced::{
    EnhancedMarketDataStream, subscribe_realtime_ticker
};

let stream = EnhancedMarketDataStream::new();
stream.connect_binance(vec!["BTCUSDT".to_string()]).await?;

// Subscribe to ticker data
let mut rx = stream.subscribe_all();
while let Ok(tick) = rx.recv().await {
    println!("Price: {}", tick.price);
}
```

---

## Trading API

### Order Manager

```rust
use investintel_agent::trading::{
    OrderManager, OrderRequest, Exchange,
    BinanceFuturesClient, OkxClient, RiskEngine
};

// Create order manager
let order_manager = OrderManager::new(
    BinanceFuturesClient::new(
        "api_key".to_string(),
        "secret".to_string(),
        true  // testnet
    ),
    OkxClient::new(
        "api_key".to_string(),
        "secret".to_string(),
        "passphrase".to_string(),
        true  // testnet
    ),
    RiskEngine::new(10000.0, 1000.0, 5000.0, 20, vec![])?,
);

// Place order
let order = OrderRequest {
    symbol: "BTCUSDT".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Market,
    quantity: 0.001,
    ..Default::default()
};

let receipt = order_manager.place_order(order, Exchange::Binance).await?;
```

---

## MCP Integration

### MCP Gateway

```rust
use investintel_agent::mcp::{MCPGateway, GatewayConfig};

let gateway = MCPGateway::new(GatewayConfig::default()).await?;

// Query data source
let data = gateway.query_data("AAPL").await?;

// Execute trade
let result = gateway.execute_trade(order).await?;
```

---

## Error Handling

All functions return `Result<T>`:

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let messages = query("Hello", None).await?;
    Ok(())
}
```

Common error types:
- `AgentError` - Agent execution errors
- `InvestError` - Investment analysis errors
- `DataError` - Data source errors
- `TradingError` - Trading errors

---

## Async Patterns

Most API calls are async. Use Tokio runtime:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Async code here
    Ok(())
}
```

For existing async code, use `.await`:

```rust
async fn my_function() -> Result<()> {
    let result = query("test", None).await?;
    Ok(())
}
```

---

## Related Documentation

- [Agent System API](../architecture/agents.md)
- [Orchestration API](../architecture/orchestration.md)
- [Skills API](../architecture/skills.md)
- [Trading API](trading-api.md)

---

**API Documentation**: Run `cargo doc --open` for full API docs

**Last Updated**: 2026-01-12
