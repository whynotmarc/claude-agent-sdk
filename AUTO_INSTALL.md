# Claude Code CLI Auto-Installation

## 概述

Claude Agent SDK 现在支持自动下载和安装 Claude Code CLI，无需手动操作。

## 功能特性

- ✅ **自动检测** - 检测 CLI 是否已安装
- ✅ **智能安装** - 优先使用 npm，失败时回退到直接下载
- ✅ **跨平台** - 支持 macOS、Linux、Windows
- ✅ **用户友好** - 实时进度反馈，清晰的错误提示
- ✅ **安全可靠** - 仅从官方源下载，安装到用户本地目录
- ✅ **向后兼容** - 默认禁用，可选启用

## 使用方式

### 方式 1：环境变量（推荐）

最简单的方式，无需修改代码：

```bash
export CLAUDE_AUTO_INSTALL_CLI=true
cargo run
```

### 方式 2：代码配置

在代码中显式启用：

```rust
use claude_agent_sdk::{ClaudeClient, ClaudeAgentOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ClaudeAgentOptions::builder()
        .auto_install_cli(true)
        .build();

    let client = ClaudeClient::new(options)?;
    let response = client.query("Hello!").await?;

    println!("{}", response);
    Ok(())
}
```

### 方式 3：带进度回调

获取实时安装进度：

```rust
use claude_agent_sdk::{ClaudeClient, ClaudeAgentOptions};
use claude_agent_sdk::internal::cli_installer::InstallProgress;
use std::sync::Arc;

let options = ClaudeAgentOptions::builder()
    .auto_install_cli(true)
    .cli_install_callback(Some(Arc::new(|progress| {
        match progress {
            InstallProgress::Downloading { current, total } => {
                println!("Downloaded: {}/{} bytes", current, total.unwrap_or(0));
            }
            InstallProgress::Done(path) => {
                println!("Installed at: {}", path.display());
            }
            _ => {}
        }
    })))
    .build();

let client = ClaudeClient::new(options)?;
```

## 安装过程

### 自动安装流程

```
1. 检查 CLI 是否已安装
   ├─ 已安装 → 直接使用 ✅
   └─ 未安装 → 继续下一步

2. 尝试通过 npm 安装
   ├─ 成功 → 验证并使用 ✅
   ├─ npm 不可用 → 尝试直接下载
   └─ 失败 → 继续下一步

3. 尝试直接下载
   ├─ 成功 → 安装到 ~/.local/bin 或 %USERPROFILE%\AppData\Local\Programs\Claude ✅
   └─ 失败 → 返回错误，提示手动安装 ❌
```

### 安装位置

| 平台 | 安装目录 |
|------|---------|
| **macOS/Linux** | `~/.local/bin/claude` |
| **Windows** | `%USERPROFILE%\AppData\Local\Programs\Claude\claude.exe` |
| **npm 安装** | npm 全局安装路径（通常 `~/.npm-global/bin`） |

## 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `CLAUDE_AUTO_INSTALL_CLI` | 启用自动安装 | `false` |
| `CLAUDE_CLI_PATH` | 手动指定 CLI 路径 | - |
| `SKIP_CLAUDE_CHECK` | 跳过 build.rs 检查 | `false` |

## 错误处理

### 自动安装失败

如果自动安装失败，SDK 会返回清晰的错误消息：

```
Failed to install Claude CLI automatically.
Please install manually: npm install -g @anthropic-ai/claude-code

Error: <具体错误原因>
```

### 手动安装

作为备用方案，您仍然可以手动安装：

```bash
# 使用 npm（推荐）
npm install -g @anthropic-ai/claude-code

# 或使用提供的脚本
./scripts/check_and_install_claude.sh
```

## 故障排除

### 问题：npm 安装失败

**可能原因：**
- npm 未安装
- 网络连接问题
- 权限问题

**解决方案：**
```bash
# 检查 npm 是否可用
npm --version

# 如果 npm 未安装，从 https://nodejs.org/ 安装
```

### 问题：直接下载失败

**可能原因：**
- 网络连接问题
- GitHub Releases 不可访问
- 平台不支持

**解决方案：**
- 检查网络连接
- 使用 npm 安装作为替代
- 手动下载并安装

### 问题：安装成功但找不到 CLI

**可能原因：**
- 安装目录不在 PATH 中

**解决方案：**

**临时方案（使用绝对路径）：**
```rust
let options = ClaudeAgentOptions::builder()
    .cli_path(PathBuf::from("~/.local/bin/claude"))
    .build();
```

**永久方案（添加到 PATH）：**

```bash
# macOS/Linux - 添加到 ~/.bashrc 或 ~/.zshrc
export PATH="$HOME/.local/bin:$PATH"

# Windows - 添加到系统 PATH
# 设置 → 环境变量 → 编辑 PATH → 添加 %USERPROFILE%\AppData\Local\Programs\Claude
```

## 测试

### 运行自动安装示例

```bash
cd crates/claude-agent-sdk
cargo run --example auto_install_cli
```

### 验证安装

```bash
# 检查 CLI 版本
claude --version

# 查看 CLI 路径
which claude  # macOS/Linux
where claude  # Windows
```

## 安全性

### 下载源

自动安装仅从以下官方源下载：
- **npm registry**: `@anthropic-ai/claude-code`
- **GitHub Releases**: `github.com/anthropics/claude-code`

### 验证

- npm 安装：npm 自动验证包完整性
- 直接下载：使用 HTTPS 加密连接

### 权限

- 仅安装到用户本地目录
- **不需要** sudo 或管理员权限
- 不会修改系统目录

## 性能影响

### 首次运行

- 如果 CLI 未安装：需要下载（约 30-100 MB，取决于平台）
- 典型下载时间：10-60 秒（取决于网络速度）

### 后续运行

- 如果 CLI 已安装：无额外开销
- 自动跳过安装步骤

## 最佳实践

### 1. 开发环境

```bash
# 启用自动安装（开发环境推荐）
export CLAUDE_AUTO_INSTALL_CLI=true
```

### 2. 生产环境

```rust
// 生产环境：预装 CLI，禁用自动安装
let options = ClaudeAgentOptions::builder()
    .auto_install_cli(false)  // 明确禁用
    .cli_path(PathBuf::from("/usr/local/bin/claude"))  // 指定路径
    .build();
```

### 3. CI/CD 环境

```yaml
# .github/workflows/test.yml
- name: Install Claude CLI
  run: npm install -g @anthropic-ai/claude-code

# 或启用自动安装
- name: Run tests
  env:
    CLAUDE_AUTO_INSTALL_CLI: true
  run: cargo test
```

## 限制

1. **网络依赖** - 需要互联网连接下载 CLI
2. **平台支持** - 支持 macOS、Linux（x64/arm64）、Windows（x64）
3. **磁盘空间** - 需要约 100 MB 额外空间
4. **npm 版本** - npm 安装需要 npm 7.0 或更高版本

## 常见问题 (FAQ)

### Q: 自动安装会修改我的系统吗？

A: 不会。所有安装都在用户本地目录进行，不需要 sudo 或管理员权限。

### Q: 我可以离线使用吗？

A: 可以。首次需要网络下载 CLI，之后可以离线使用。

### Q: 如何更新 CLI？

A: 使用 npm 更新：`npm update -g @anthropic-ai/claude-code`

### Q: 自动安装会覆盖我现有的 CLI 吗？

A: 不会。自动安装仅在找不到 CLI 时才会安装。

### Q: 可以指定安装目录吗？

A: 目前自动安装使用固定的用户本地目录。如需自定义路径，请手动安装并使用 `cli_path` 配置。

## 反馈与支持

如有问题或建议，请：
- 提交 Issue: https://github.com/louloulin/claude-agent-sdk/issues
- 查看文档: https://docs.claude.com/claude-code
