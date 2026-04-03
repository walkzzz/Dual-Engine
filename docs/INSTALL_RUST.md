# Rust 工具链安装指南

本指南介绍如何安装 Rust 工具链以进行代码格式化检查和 lint 检查。

---

## 🚀 快速安装

### Windows

**方法 1: 使用 rustup (推荐)**

```powershell
# 下载并运行安装程序
winget install Rustlang.Rustup.MSVC
# 或
choco install rustup

# 或者手动下载
# 访问 https://rustup.rs 下载 rustup-init.exe
```

**方法 2: 使用 Scoop**

```powershell
scoop install rust
```

### Linux

```bash
# 使用 rustup (推荐)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或者使用包管理器
# Ubuntu/Debian
sudo apt install rustc cargo

# Fedora/RHEL
sudo dnf install rust cargo

# Arch Linux
sudo pacman -S rust
```

### macOS

```bash
# 使用 rustup (推荐)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或者使用 Homebrew
brew install rust
```

---

## ✅ 验证安装

```bash
# 检查 Rust 版本
rustc --version

# 检查 Cargo 版本
cargo --version

# 检查 rustup 版本
rustup --version
```

预期输出：
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
rustup 1.26.0 (5af9b9484 2023-04-05)
```

---

## 🛠️ 安装额外组件

```bash
# 安装 rustfmt (代码格式化)
rustup component add rustfmt

# 安装 clippy (lint 工具)
rustup component add clippy

# 安装文档
rustup component add rust-docs

# 安装源码
rustup component add rust-src
```

---

## 📝 使用工具

### 代码格式化

```bash
# 检查代码格式
cargo fmt --all -- --check

# 格式化代码
cargo fmt --all
```

### Lint 检查

```bash
# 运行 Clippy
cargo clippy --workspace --all-targets

# 严格模式 (警告视为错误)
cargo clippy --workspace --all-targets -- -D warnings

# 生成 HTML 报告
cargo clippy --workspace --all-targets --message-format=json | clippy-html-reporter
```

### 运行测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定测试
cargo test --test engine_integration_tests

# 生成覆盖率报告
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## 🔧 配置

### rustfmt.toml

项目根目录已包含 `rustfmt.toml` 配置文件：

```toml
# 最大行宽
max_width = 120

# 缩进空格数
tab_spaces = 4

# 移除尾部空格
trim_trailing_whitespace = true

# 导入排序
reorder_imports = true

# Edition
edition = "2021"
```

### Clippy 配置

在 `Cargo.toml` 中添加：

```toml
[workspace.lints.clippy]
# 允许/禁止的 lint 规则
ptr_as_ptr = "warn"
undocumented_unsafe_blocks = "deny"
```

---

## 🐛 常见问题

### Q: rustup 安装超时？

**A:** 使用国内镜像：

```bash
# 使用清华大学镜像
export RUSTUP_DIST_SERVER="https://mirrors.tuna.tsinghua.edu.cn/rustup"
export RUSTUP_UPDATE_ROOT="https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup"
curl --proto '=https' --tlsv1.2 -sSf https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup/init.rs | sh
```

### Q: cargo 命令找不到？

**A:** 添加 Cargo bin 到 PATH：

```bash
# Linux/macOS
export PATH="$HOME/.cargo/bin:$PATH"

# Windows PowerShell
$env:PATH += ";$HOME\.cargo\bin"
```

### Q: 更新 Rust？

**A:**

```bash
# 更新工具链
rustup update

# 更新到特定版本
rustup install 1.75.0
rustup default 1.75.0
```

---

## 📚 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rustup 文档](https://rust-lang.github.io/rustup/)
- [Clippy 使用指南](https://github.com/rust-lang/rust-clippy)
- [Rust 风格指南](https://doc.rust-lang.org/style-guide/rust/)

---

**安装完成后运行：**

```bash
# 格式化检查
cargo fmt --all -- --check

# Clippy 检查
cargo clippy --workspace --all-targets -- -D warnings

# 运行测试
cargo test --workspace
```