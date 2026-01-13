# Agent System

This document describes the agent system architecture in the investment analysis platform.

## Table of Contents

- [Overview](#overview)
- [Agent Architecture](#agent-architecture)
- [Agent Types](#agent-types)
- [Agent Lifecycle](#agent-lifecycle)
- [Agent Communication](#agent-communication)
- [Implementation Details](#implementation-details)
- [Examples](#examples)

---

## Overview

The agent system provides a modular, extensible framework for building intelligent investment analysis agents. Each agent specializes in a specific domain and can be orchestrated independently or in coordination with other agents.

### Key Design Principles

1. **Single Responsibility** - Each agent has one clear purpose
2. **Composability** - Agents can be combined and orchestrated
3. **Stateless** - Agents don't maintain internal state between executions
4. **Type Safety** - Strong typing for inputs and outputs
5. **Async-First** - All operations are async for scalability

---

## Agent Architecture

### Core Trait

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    /// Execute the agent with given input
    async fn execute(&self, input: AgentInput) -> Result<AgentOutput>;

    /// Get agent metadata
    fn metadata(&self) -> AgentMetadata;
}
```

### AgentInput

```rust
pub struct AgentInput {
    /// Input data (can be stock symbol, query, or structured data)
    pub data: InputData,

    /// Optional configuration for this execution
    pub config: Option<AgentConfig>,

    /// Execution context (session ID, user ID, etc.)
    pub context: ExecutionContext,
}

pub enum InputData {
    Single(String),              // Single stock symbol
    Multiple(Vec<String>),       // Multiple symbols
    Query(String),               // Natural language query
    Structured(serde_json::Value), // Structured data
}
```

### AgentOutput

```rust
pub struct AgentOutput {
    /// Analysis results
    pub result: AgentResult,

    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,

    /// Additional metadata
    pub metadata: OutputMetadata,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

pub enum AgentResult {
    Analysis(AnalysisReport),     // Investment analysis
    Recommendation(Recommendation), // Trading recommendation
    Data(DataSnapshot),           // Market data
    Signal(TradingSignal),        // Trading signal
    Report(BacktestReport),       // Backtest results
}
```

---

## Agent Types

### 1. ValueInvestmentAgent

**Purpose**: Graham-Buffett value investing analysis

**Responsibilities**:
- Calculate intrinsic value using Graham formula
- Analyze financial health metrics
- Assess margin of safety
- Generate value ratings

**Input**: Stock symbol or financial data
**Output**: Value analysis report with buy/hold/sell recommendation

```rust
use investintel_agent::ValueInvestmentAgent;

let agent = ValueInvestmentAgent::new();
let input = AgentInput::new("AAPL");
let output = agent.execute(input).await?;

match output.result {
    AgentResult::Analysis(report) => {
        println!("Intrinsic Value: ${}", report.intrinsic_value);
        println!("Margin of Safety: {}%", report.margin_of_safety);
        println!("Recommendation: {:?}", report.recommendation);
    }
    _ => {}
}
```

**Key Metrics**:
- Intrinsic value (Graham formula)
- P/E ratio, P/B ratio
- Debt-to-equity ratio
- Current ratio, quick ratio
- Free cash flow
- Dividend yield

### 2. QualityValueAgent

**Purpose**: Warren Buffett's quality value investing approach

**Responsibilities**:
- Assess business quality (moat, management, brand)
- Analyze competitive advantage
- Evaluate long-term growth potential
- Calculate fair value for quality businesses

**Input**: Stock symbol with business analysis
**Output**: Quality assessment with fair value estimate

```rust
use investintel_agent::QualityValueAgent;

let agent = QualityValueAgent::new();
let input = AgentInput::new_with_context("MOAT", AnalysisContext::Quality);
let output = agent.execute(input).await?;

let quality_report = output.as_quality_report()?;
println!("Quality Score: {}/100", quality_report.score);
println!("Moat Rating: {}", quality_report.moat_rating);
```

**Key Metrics**:
- Moat strength (1-10)
- Return on Invested Capital (ROIC)
- Free cash flow consistency
- Brand value assessment
- Management quality score

### 3. PortfolioManagerAgent

**Purpose**: Modern Portfolio Theory and optimization

**Responsibilities**:
- Calculate efficient frontier
- Optimize portfolio weights
- Assess portfolio risk-return
- Generate allocation recommendations

**Input**: List of stock symbols
**Output**: Optimized portfolio allocation

```rust
use investintel_agent::PortfolioManagerAgent;

let agent = PortfolioManagerAgent::new();
let input = AgentInput::new(vec!["AAPL", "MSFT", "GOOGL"]);
let output = agent.execute(input).await?;

let portfolio = output.as_portfolio()?;
println!("Optimal Weights:");
for (symbol, weight) in &portfolio.weights {
    println!("  {}: {:.2}%", symbol, weight * 100.0);
}
println!("Expected Return: {:.2}%", portfolio.expected_return * 100.0);
println!("Expected Risk: {:.2}%", portfolio.risk * 100.0);
```

**Key Metrics**:
- Covariance matrix
- Efficient frontier points
- Optimal weights (mean-variance optimization)
- Sharpe ratio
- Portfolio beta
- Diversification ratio

### 4. TradingAdvisorAgent

**Purpose**: Trading advice and position sizing

**Responsibilities**:
- Generate trading signals
- Calculate position sizes (Kelly criterion)
- Set stop-loss and take-profit levels
- Assess risk-reward ratios

**Input**: Stock symbol with market data
**Output**: Trading recommendation with position sizing

```rust
use investintel_agent::TradingAdvisorAgent;

let agent = TradingAdvisorAgent::new();
let input = AgentInput::new_with_data("TSLA", market_data);
let output = agent.execute(input).await?;

let signal = output.as_trading_signal()?;
println!("Signal: {:?}", signal.direction);
println!("Position Size: {:.2}%", signal.position_size * 100.0);
println!("Entry: ${}", signal.entry_price);
println!("Stop Loss: ${}", signal.stop_loss);
println!("Take Profit: ${}", signal.take_profit);
```

**Key Metrics**:
- Trading signal (buy/sell/hold)
- Position size (Kelly criterion)
- Risk-reward ratio
- Entry/exit points
- Stop-loss level
- Confidence level

### 5. DividendInvestmentAgent

**Purpose**: Dividend investing analysis

**Responsibilities**:
- Analyze dividend sustainability
- Calculate dividend yield
- Assess dividend growth history
- Generate income projections

**Input**: Stock symbol
**Output**: Dividend analysis report

```rust
use investintel_agent::DividendInvestmentAgent;

let agent = DividendInvestmentAgent::new();
let input = AgentInput::new("JNJ");
let output = agent.execute(input).await?;

let dividend_report = output.as_dividend_report()?;
println!("Yield: {:.2}%", dividend_report.yield * 100.0);
println!("Payout Ratio: {:.2}%", dividend_report.payout_ratio * 100.0);
println!("Growth Rate: {:.2}%", dividend_report.growth_rate * 100.0);
```

**Key Metrics**:
- Dividend yield
- Payout ratio
- Dividend growth rate
- Yield on cost
- Dividend sustainability score
- Income projection

### 6. KellyPositionAgent

**Purpose**: Scientific position sizing using Kelly criterion

**Responsibilities**:
- Calculate optimal position sizes
- Assess Kelly fraction (full, half, quarter)
- Evaluate risk of ruin
- Generate position sizing recommendations

**Input**: Trading opportunity parameters
**Output**: Optimal position sizing recommendation

```rust
use investintel_agent::KellyPositionAgent;

let agent = KellyPositionAgent::new();
let input = AgentInput::new_with_params(win_rate = 0.55, avg_win = 200, avg_loss = 100);
let output = agent.execute(input).await?;

let kelly = output.as_kelly_result()?;
println!("Kelly %: {:.2}%", kelly.kelly_percentage * 100.0);
println!("Half-Kelly: {:.2}%", kelly.half_kelly * 100.0);
println!("Position Size: ${}", kelly.position_size);
```

**Key Metrics**:
- Kelly criterion percentage
- Half-Kelly / Quarter-Kelly
- Expected growth rate
- Risk of ruin
- Optimal position size
- Confidence interval

---

## Agent Lifecycle

### 1. Initialization

```rust
let agent = ValueInvestmentAgent::new();
// Agent is now ready to execute
```

### 2. Execution

```rust
let input = AgentInput::new("AAPL");
let output = agent.execute(input).await?;
```

### 3. Result Processing

```rust
match output.result {
    AgentResult::Analysis(report) => {
        // Handle analysis report
    }
    AgentResult::Recommendation(rec) => {
        // Handle recommendation
    }
    _ => {}
}
```

### 4. Error Handling

```rust
if let Err(e) = agent.execute(input).await {
    eprintln!("Agent execution failed: {}", e);

    // Check error type
    if let Some(AgentError::DataNotFound) = e.downcast_ref::<AgentError>() {
        // Handle missing data
    }
}
```

---

## Agent Communication

### Direct Execution

```rust
let agent = ValueInvestmentAgent::new();
let output = agent.execute(input).await?;
```

### Orchestrated Execution

See [Orchestration System](orchestration.md) for details on multi-agent coordination.

### Message Passing

Agents communicate through typed messages:

```rust
pub struct AgentMessage {
    pub from: AgentId,
    pub to: AgentId,
    pub content: MessageContent,
    pub timestamp: DateTime<Utc>,
}

pub enum MessageContent {
    Request(AgentInput),
    Response(AgentOutput),
    Error(String),
}
```

---

## Implementation Details

### Error Handling

All agents return `Result<AgentOutput>`:

```rust
pub type Result<T> = std::result::Result<T, AgentError>;

pub enum AgentError {
    DataNotFound(String),
    InvalidInput(String),
    ExecutionFailed(String),
    Timeout(Duration),
    ExternalApiError(String),
}
```

### Performance Considerations

1. **Caching**: Agents cache expensive calculations
2. **Parallel Execution**: Multiple agents can run concurrently
3. **Streaming**: Large data streams are processed incrementally
4. **Resource Limits**: Each agent has memory and CPU limits

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_value_agent() {
        let agent = ValueInvestmentAgent::new();
        let input = AgentInput::new("TEST");
        let output = agent.execute(input).await;

        assert!(output.is_ok());
    }
}
```

---

## Examples

### Example 1: Single Stock Analysis

```rust
use investintel_agent::{Agent, ValueInvestmentAgent};
use investintel_agent::types::{AgentInput, InputData};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let agent = ValueInvestmentAgent::new();
    let input = AgentInput {
        data: InputData::Single("AAPL".to_string()),
        config: None,
        context: ExecutionContext::default(),
    };

    let output = agent.execute(input).await?;

    if let AgentResult::Analysis(report) = output.result {
        println!("Analysis for AAPL:");
        println!("  Intrinsic Value: ${}", report.intrinsic_value);
        println!("  Current Price: ${}", report.current_price);
        println!("  Margin of Safety: {:.1}%", report.margin_of_safety);
    }

    Ok(())
}
```

### Example 2: Portfolio Optimization

```rust
use investintel_agent::{Agent, PortfolioManagerAgent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let agent = PortfolioManagerAgent::new();
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN"];
    let input = AgentInput::new(symbols);

    let output = agent.execute(input).await?;
    let portfolio = output.as_portfolio()?;

    println!("Optimal Portfolio Allocation:");
    for (symbol, weight) in &portfolio.weights {
        println!("  {}: {:.2}%", symbol, weight * 100.0);
    }

    println!("\nRisk-Return Profile:");
    println!("  Expected Return: {:.2}%", portfolio.expected_return * 100.0);
    println!("  Expected Risk: {:.2}%", portfolio.risk * 100.0);
    println!("  Sharpe Ratio: {:.2}", portfolio.sharpe_ratio);

    Ok(())
}
```

### Example 3: Multi-Agent Analysis

```rust
use investintel_agent::{
    ValueInvestmentAgent, TradingAdvisorAgent, KellyPositionAgent
};
use futures::join;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let symbol = "TSLA";

    // Run multiple agents in parallel
    let value_future = async {
        let agent = ValueInvestmentAgent::new();
        agent.execute(AgentInput::new(symbol)).await
    };

    let trading_future = async {
        let agent = TradingAdvisorAgent::new();
        agent.execute(AgentInput::new(symbol)).await
    };

    let kelly_future = async {
        let agent = KellyPositionAgent::new();
        agent.execute(AgentInput::new(symbol)).await
    };

    let (value_result, trading_result, kelly_result) = join!(
        value_future,
        trading_future,
        kelly_future
    );

    // Combine results
    println!("Multi-Agent Analysis for {}:", symbol);
    println!("{:#?}", value_result?);
    println!("{:#?}", trading_result?);
    println!("{:#?}", kelly_result?);

    Ok(())
}
```

---

## Related Documentation

- [Orchestration System](orchestration.md) - Multi-agent coordination
- [Skills System](skills.md) - Modular skill framework
- [Investment Analysis Guide](../guides/investment-analysis.md) - User guide

---

**Last Updated**: 2026-01-12
