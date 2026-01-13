# Orchestration System

This document describes the orchestration system for coordinating multiple agents.

## Table of Contents

- [Overview](#overview)
- [Orchestration Patterns](#orchestration-patterns)
- [Orchestrator Types](#orchestrator-types)
- [Implementation](#implementation)
- [Examples](#examples)
- [Best Practices](#best-practices)

---

## Overview

The orchestration system enables coordinated execution of multiple agents, supporting various patterns from simple sequential flows to complex parallel workflows.

### Key Features

- **Parallel Execution** - Run multiple agents concurrently
- **Sequential Execution** - Chain agents in dependency order
- **Hierarchical Orchestration** - Nest orchestrators for complex workflows
- **Result Aggregation** - Combine results from multiple agents
- **Error Handling** - Graceful failure handling and fallbacks
- **Resource Management** - Control concurrency and resource usage

---

## Orchestration Patterns

### 1. Parallel Orchestration

Execute multiple agents simultaneously and collect all results.

```rust
use claude_agent_sdk::orchestration::{ParallelOrchestrator, AgentTask};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let orchestrator = ParallelOrchestrator::new();

    let tasks = vec![
        AgentTask::new("value", value_agent, input1.clone()),
        AgentTask::new("trading", trading_agent, input2.clone()),
        AgentTask::new("kelly", kelly_agent, input3.clone()),
    ];

    let results = orchestrator.execute(tasks).await?;

    for (name, result) in results {
        println!("{}: {:?}", name, result);
    }

    Ok(())
}
```

**Use Cases**:
- Analyzing multiple stocks simultaneously
- Running different analysis strategies in parallel
- Comparing multiple models

### 2. Sequential Orchestration

Chain agents where output of one becomes input to next.

```rust
use claude_agent_sdk::orchestration::{SequentialOrchestrator, AgentChain};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let orchestrator = SequentialOrchestrator::new();

    let chain = AgentChain::new()
        .then(value_agent)           // Analyze value
        .then(quality_agent)          // Assess quality
        .then(trading_agent)          // Generate trade signal
        .then(kelly_agent);           // Calculate position size

    let result = orchestrator.execute(chain, initial_input).await?;

    println!("Final result: {:?}", result);

    Ok(())
}
```

**Use Cases**:
- Multi-stage analysis pipelines
- Data preprocessing → Analysis → Recommendation
- Cascading agent workflows

### 3. Hierarchical Orchestration

Nest orchestrators for complex workflows.

```rust
use claude_agent_sdk::orchestration::{HierarchicalOrchestrator, OrchestratorNode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut orchestrator = HierarchicalOrchestrator::new();

    // Level 1: Analysis (parallel)
    let analysis_node = OrchestratorNode::Parallel(parallel_tasks);

    // Level 2: Synthesis (sequential)
    let synthesis_node = OrchestratorNode::Sequential(synthesis_chain);

    // Build hierarchy
    orchestrator
        .add_node("analysis", analysis_node)?
        .add_node("synthesis", synthesis_node)?
        .add_dependency("synthesis", "analysis")?;

    let results = orchestrator.execute(initial_input).await?;

    Ok(())
}
```

**Use Cases**:
- Complex multi-stage workflows
- Conditional agent execution
- Workflow with error recovery

---

## Orchestrator Types

### ParallelOrchestrator

```rust
pub struct ParallelOrchestrator {
    max_concurrency: Option<usize>,  // Limit concurrent tasks
    timeout: Option<Duration>,        // Per-task timeout
}

impl ParallelOrchestrator {
    pub fn new() -> Self;

    pub fn with_concurrency(mut self, n: usize) -> Self;

    pub fn with_timeout(mut self, timeout: Duration) -> Self;

    pub async fn execute(
        &self,
        tasks: Vec<AgentTask>
    ) -> Result<HashMap<String, AgentOutput>>;
}
```

**Example**:

```rust
let orchestrator = ParallelOrchestrator::new()
    .with_concurrency(5)
    .with_timeout(Duration::from_secs(30));

let results = orchestrator.execute(tasks).await?;
```

### SequentialOrchestrator

```rust
pub struct SequentialOrchestrator {
    stop_on_error: bool,              // Stop on first error
    continue_on_failure: bool,        // Continue with fallback values
}

impl SequentialOrchestrator {
    pub fn new() -> Self;

    pub fn stop_on_error(mut self, stop: bool) -> Self;

    pub fn continue_on_failure(mut self, cont: bool) -> Self;

    pub async fn execute(
        &self,
        chain: AgentChain,
        initial_input: AgentInput
    ) -> Result<AgentOutput>;
}
```

**Example**:

```rust
let orchestrator = SequentialOrchestrator::new()
    .stop_on_error(true)
    .continue_on_failure(false);

let result = orchestrator.execute(chain, input).await?;
```

### HierarchicalOrchestrator

```rust
pub struct HierarchicalOrchestrator {
    nodes: HashMap<String, OrchestratorNode>,
    dependencies: HashMap<String, Vec<String>>,
}

impl HierarchicalOrchestrator {
    pub fn new() -> Self;

    pub fn add_node(
        &mut self,
        name: String,
        node: OrchestratorNode
    ) -> Result<()>;

    pub fn add_dependency(
        &mut self,
        node: String,
        depends_on: String
    ) -> Result<()>;

    pub async fn execute(
        &self,
        initial_input: AgentInput
    ) -> Result<HashMap<String, AgentOutput>>;
}
```

**Example**:

```rust
let mut orchestrator = HierarchicalOrchestrator::new();

orchestrator
    .add_node("fetch_data", fetch_node)?
    .add_node("analyze", analyze_node)?
    .add_node("report", report_node)?
    .add_dependency("analyze", "fetch_data")?
    .add_dependency("report", "analyze")?;

let results = orchestrator.execute(input).await?;
```

---

## Implementation

### AgentTask

```rust
pub struct AgentTask {
    pub name: String,
    pub agent: Arc<dyn Agent>,
    pub input: AgentInput,
    pub priority: TaskPriority,
}

pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl AgentTask {
    pub fn new(
        name: impl Into<String>,
        agent: impl Agent + 'static,
        input: AgentInput
    ) -> Self;
}
```

### AgentChain

```rust
pub struct AgentChain {
    links: Vec<ChainLink>,
}

pub struct ChainLink {
    pub agent: Arc<dyn Agent>,
    pub transform: Option<Box<dyn TransformOutput>>,
}

impl AgentChain {
    pub fn new() -> Self;

    pub fn then(
        mut self,
        agent: impl Agent + 'static
    ) -> Self;

    pub fn then_with_transform(
        mut self,
        agent: impl Agent + 'static,
        transform: impl Fn(AgentOutput) -> AgentInput + 'static
    ) -> Self;
}
```

### ResultAggregation

```rust
pub trait ResultAggregator: Send + Sync {
    fn aggregate(
        &self,
        results: Vec<AgentOutput>
    ) -> Result<AgentOutput>;
}

// Built-in aggregators
pub struct FirstSuccessfulAggregator;
pub struct MajorityVoteAggregator;
pub struct AverageConfidenceAggregator;
pub struct CustomAggregator<F>(F);
```

---

## Examples

### Example 1: Parallel Multi-Stock Analysis

```rust
use claude_agent_sdk::orchestration::{ParallelOrchestrator, AgentTask};
use investintel_agent::{ValueInvestmentAgent, TradingAdvisorAgent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN", "TSLA"];
    let mut tasks = Vec::new();

    // Create analysis tasks for each symbol
    for symbol in symbols {
        let value_agent = ValueInvestmentAgent::new();
        let trading_agent = TradingAdvisorAgent::new();

        tasks.push(AgentTask::new(
            format!("{}_value", symbol),
            value_agent,
            AgentInput::new(symbol)
        ));

        tasks.push(AgentTask::new(
            format!("{}_trading", symbol),
            trading_agent,
            AgentInput::new(symbol)
        ));
    }

    // Execute all tasks in parallel
    let orchestrator = ParallelOrchestrator::new()
        .with_concurrency(10)
        .with_timeout(Duration::from_secs(60));

    let results = orchestrator.execute(tasks).await?;

    // Aggregate results
    for (name, result) in results {
        println!("{}: confidence={:.2}", name, result.confidence);
    }

    Ok(())
}
```

### Example 2: Sequential Analysis Pipeline

```rust
use claude_agent_sdk::orchestration::{SequentialOrchestrator, AgentChain};
use investintel_agent::{
    ValueInvestmentAgent, QualityValueAgent,
    TradingAdvisorAgent, KellyPositionAgent
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Build analysis pipeline
    let chain = AgentChain::new()
        .then(ValueInvestmentAgent::new())
        .then_with_transform(
            QualityValueAgent::new(),
            |output| {
                // Transform value analysis to quality analysis input
                AgentInput::new_with_data("symbol", extract_data(output))
            }
        )
        .then(TradingAdvisorAgent::new())
        .then(KellyPositionAgent::new());

    let orchestrator = SequentialOrchestrator::new()
        .stop_on_error(true);

    let result = orchestrator
        .execute(chain, AgentInput::new("AAPL"))
        .await?;

    println!("Final recommendation: {:?}", result);

    Ok(())
}
```

### Example 3: Hierarchical Workflow

```rust
use claude_agent_sdk::orchestration::HierarchicalOrchestrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut orchestrator = HierarchicalOrchestrator::new();

    // Level 1: Data fetching (parallel)
    let data_tasks = vec![
        create_fetch_task("yahoo"),
        create_fetch_task("alpha_vantage"),
    ];
    let data_node = OrchestratorNode::Parallel(data_tasks);

    // Level 2: Analysis (parallel)
    let analysis_tasks = vec![
        create_analysis_task("value"),
        create_analysis_task("quality"),
        create_analysis_task("trading"),
    ];
    let analysis_node = OrchestratorNode::Parallel(analysis_tasks);

    // Level 3: Synthesis (sequential)
    let synthesis_chain = create_synthesis_chain();
    let synthesis_node = OrchestratorNode::Sequential(synthesis_chain);

    // Build workflow
    orchestrator
        .add_node("fetch_data", data_node)?
        .add_node("analyze", analysis_node)?
        .add_node("synthesize", synthesis_node)?
        .add_dependency("analyze", "fetch_data")?
        .add_dependency("synthesize", "analyze")?;

    let results = orchestrator
        .execute(AgentInput::new("AAPL"))
        .await?;

    println!("Workflow complete: {:#?}", results);

    Ok(())
}
```

### Example 4: Error Recovery with Fallback

```rust
use claude_agent_sdk::orchestration::{SequentialOrchestrator, AgentChain};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let chain = AgentChain::new()
        .then(PrimaryAnalysisAgent::new())
        .then_with_fallback(
            SecondaryAnalysisAgent::new(),
            |error| {
                // Fallback if primary fails
                eprintln!("Primary failed: {}, using fallback", error);
                AgentInput::new_with_fallback_strategy()
            }
        )
        .then(RecommendationAgent::new());

    let orchestrator = SequentialOrchestrator::new()
        .stop_on_error(false)
        .continue_on_failure(true);

    let result = orchestrator.execute(chain, input).await?;

    Ok(())
}
```

---

## Best Practices

### 1. Choose the Right Pattern

**Use Parallel When**:
- Tasks are independent
- Order doesn't matter
- Performance is critical

**Use Sequential When**:
- Tasks have dependencies
- Output of one feeds into next
- Order matters

**Use Hierarchical When**:
- Complex workflow with both patterns
- Need conditional execution
- Multiple stages with dependencies

### 2. Error Handling

```rust
// Stop on first error
let orchestrator = SequentialOrchestrator::new()
    .stop_on_error(true);

// Continue and collect errors
let orchestrator = ParallelOrchestrator::new()
    .collect_errors(true);

// Provide fallback values
let chain = AgentChain::new()
    .then(primary_agent)
    .then_with_fallback(backup_agent);
```

### 3. Resource Management

```rust
// Limit concurrency to avoid overwhelming APIs
let orchestrator = ParallelOrchestrator::new()
    .with_concurrency(5);

// Set timeout to prevent hanging
let orchestrator = ParallelOrchestrator::new()
    .with_timeout(Duration::from_secs(30));

// Use priorities for critical tasks
let task = AgentTask::new(name, agent, input)
    .with_priority(TaskPriority::High);
```

### 4. Result Aggregation

```rust
// Take first successful result
orchestrator.set_aggregator(FirstSuccessfulAggregator);

// Majority vote
orchestrator.set_aggregator(MajorityVoteAggregator);

// Average confidence
orchestrator.set_aggregator(AverageConfidenceAggregator);

// Custom aggregation
orchestrator.set_aggregator(CustomAggregator(|results| {
    // Custom logic to combine results
    aggregate_results(results)
}));
```

### 5. Performance Optimization

```rust
// Batch similar tasks
let tasks: Vec<_> = symbols
    .iter()
    .map(|symbol| AgentTask::new(symbol, agent.clone(), input))
    .collect();

// Reuse agent instances
let agent = Arc::new(ValueInvestmentAgent::new());
let tasks = vec![
    AgentTask::new("task1", agent.clone(), input1),
    AgentTask::new("task2", agent.clone(), input2),
];

// Use streaming for large result sets
let results = orchestrator.execute_streaming(tasks).await?;
while let Some(result) = results.next().await {
    process_result(result);
}
```

---

## Performance Considerations

### Concurrency Limits

```rust
// Too high: May overwhelm external APIs
.with_concurrency(100)  // ❌

// Just right: Balance speed and resource usage
.with_concurrency(10)   // ✅
```

### Timeout Configuration

```rust
// Too short: Premature timeouts
.with_timeout(Duration::from_secs(1))   // ❌

// Too long: Wastes time on failed tasks
.with_timeout(Duration::from_secs(300)) // ❌

// Just right: Allow completion but fail fast
.with_timeout(Duration::from_secs(30))  // ✅
```

### Memory Management

```rust
// Stream results instead of collecting all
let stream = orchestrator.execute_streaming(tasks).await?;

// Process incrementally to avoid memory spike
while let Some(result) = stream.next().await {
    process(result);  // Handle and drop
}
```

---

## Troubleshooting

### Deadlocks

**Problem**: Agents waiting on each other

**Solution**: Use lock-free design, avoid circular dependencies

```rust
// ❌ Circular dependency
orchestrator.add_dependency("A", "B")?;
orchestrator.add_dependency("B", "A")?;  // Deadlock!

// ✅ Proper hierarchy
orchestrator.add_dependency("B", "A")?;
orchestrator.add_dependency("C", "B")?;
```

### Timeout Errors

**Problem**: Agents taking too long

**Solution**: Increase timeout or optimize agent

```rust
// Add timeout
.with_timeout(Duration::from_secs(60))

// Or optimize slow agent
// - Add caching
// - Use streaming
// - Parallelize internal operations
```

### Memory Issues

**Problem**: Too many results in memory

**Solution**: Use streaming or batching

```rust
// ❌ Collect all results
let results = orchestrator.execute(large_task_set).await?;

// ✅ Stream results
let stream = orchestrator.execute_streaming(large_task_set).await?;
while let Some(result) = stream.next().await {
    // Process and drop
}
```

---

## Related Documentation

- [Agent System](agents.md) - Agent architecture
- [Skills System](skills.md) - Modular skill framework
- [Investment Analysis Guide](../guides/investment-analysis.md) - Usage examples

---

**Last Updated**: 2026-01-12
