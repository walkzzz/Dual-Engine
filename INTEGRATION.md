# Dual Engine - 完整集成方案

## 项目结构

```
dual-engine/
├── Cargo.toml                    # Workspace 根配置
├── go.mod                        # Go 模块配置 (for OpenCode)
│
├── src/                          # 源码目录
│   ├── opencode/                 # OpenCode Go 源码 (~30MB)
│   │   ├── main.go
│   │   ├── cmd/
│   │   └── internal/
│   │
│   └── claude-code/              # Claude Code Rust 源码 (~20MB)
│       ├── Cargo.toml
│       ├── src-rust/
│       │   └── crates/
│       └── spec/
│
├── crates/                       # Rust 集成层
│   ├── engine-core/              # 引擎核心接口
│   ├── engine-opencode/          # OpenCode 集成
│   ├── engine-claude/            # Claude 集成
│   ├── shared-types/             # 共享类型
│   ├── cli/                      # CLI
│   └── tui/                      # TUI
│
├── install.bat                   # Windows 安装
└── install.sh                    # Linux/Mac 安装
```

## 集成方式

### OpenCode (Go) - 编译为静态二进制
```rust
// crates/engine-opencode/src/lib.rs
use std::process::Command;

pub struct OpenCodeEngine {
    binary_path: String,
}

// 调用 OpenCode 二进制
impl OpenCodeEngine {
    pub fn run(&self, prompt: &str) -> Result<String> {
        let output = Command::new(&self.binary_path)
            .arg("-p")
            .arg(prompt)
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
```

### Claude Code (Rust) - 直接调用
```rust
// crates/engine-claude/src/lib.rs
use cc_core::ClaudeCode;  // 从 vendored 导入

pub struct ClaudeEngine {
    claude: ClaudeCode,
}

// 直接调用 Claude Code 内部
impl ClaudeEngine {
    pub fn run(&self, prompt: &str) -> Result<String> {
        Ok(self.claude.run(prompt).await?)
    }
}
```

## 构建步骤

### 1. 复制源码

```bash
# 复制 OpenCode (Go 源码)
cp -r /path/to/opencode/* dual-engine/src/opencode/

# 复制 Claude Code (Rust 源码)
cp -r /path/to/claude-code/* dual-engine/src/claude-code/
```

### 2. 构建 OpenCode

```bash
cd dual-engine/src/opencode
go build -o ../../target/opencode .
```

### 3. 构建 Claude Code

```bash
cd dual-engine/src/claude-code/src-rust
cargo build --release
```

### 4. 构建双引擎 CLI

```bash
cd dual-engine
cargo build --release -p dual-engine-cli
```

## 使用方式

```bash
# CLI 模式
de run -p "hello"          # 使用 OpenCode
de run -p "hello" -e claude # 使用 Claude

# TUI 模式
det                        # 启动 TUI

# 运行时切换
:opencode   # 切换到 OpenCode
:claude     # 切换到 Claude
```

## 并行能力对比

| 引擎 | 工具调用方式 | 最大并行数 |
|------|-------------|-----------|
| OpenCode | 顺序执行 | 1 |
| Claude | 并行执行 | 10+ |