//! 自动安装 Claude Code CLI 示例
//!
//! 此示例展示如何启用 SDK 的自动 CLI 安装功能

use claude_agent_sdk::{ClaudeClient, ClaudeAgentOptions};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("🚀 Claude Agent SDK - Auto-Install Example\n");

    // 方式 1: 通过环境变量启用
    // export CLAUDE_AUTO_INSTALL_CLI=true
    //
    // 方式 2: 通过代码配置启用

    let options = ClaudeAgentOptions::builder()
        .auto_install_cli(true)
        .build();

    println!("📦 Creating client with auto-install enabled...");
    println!("   If Claude CLI is not found, it will be downloaded automatically.\n");

    // 创建客户端（会触发自动安装）
    let client = ClaudeClient::new(options)?;

    println!("✅ Client created successfully!\n");

    // 使用客户端进行查询
    println!("💬 Sending query to Claude...");
    let response = client.query("Hello, Claude! Please respond with a brief greeting.").await?;

    println!("\n📝 Response:");
    println!("{}", response);

    Ok(())
}

/// 示例：带进度回调的自动安装
#[allow(dead_code)]
async fn example_with_progress_callback() -> Result<(), Box<dyn std::error::Error>> {
    use claude_agent_sdk::internal::cli_installer::InstallProgress;

    let options = ClaudeAgentOptions::builder()
        .auto_install_cli(true)
        .cli_install_callback(Some(Arc::new(|progress| {
            match progress {
                InstallProgress::Checking(msg) => {
                    println!("🔍 {}", msg);
                }
                InstallProgress::Downloading { current, total } => {
                    if let Some(total) = total {
                        let progress = (current as f64 / total as f64 * 100.0) as u32;
                        println!("⬇️  Downloading: {}% ({}/{})", progress, current, total);
                    } else {
                        println!("⬇️  Downloading: {} bytes", current);
                    }
                }
                InstallProgress::Installing(msg) => {
                    println!("🔧 {}", msg);
                }
                InstallProgress::Done(path) => {
                    println!("✅ Installation complete: {}", path.display());
                }
                InstallProgress::Failed(err) => {
                    eprintln!("❌ {}", err);
                }
            }
        })))
        .build();

    let client = ClaudeClient::new(options)?;
    let response = client.query("Test query").await?;

    println!("{}", response);
    Ok(())
}
