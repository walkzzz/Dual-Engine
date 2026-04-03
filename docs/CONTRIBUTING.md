# Dual-Engine 开发指南

> 本文档面向 Dual-Engine 项目贡献者，介绍开发环境搭建、代码规范、提交流程等。

---

## 🚀 快速开始

### 1. 克隆仓库

```bash
git clone git@github.com:walkzzz/Dual-Engine.git
cd Dual-Engine
```

### 2. 安装依赖

**Rust 工具链：**
```bash
# 安装 Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
```

**Go 工具链 (用于 OpenCode 引擎)：**
```bash
# 安装 Go (1.21+)
# Windows: https://go.dev/dl/
# Linux: sudo apt install golang-go
```

### 3. 编译项目

```bash
# 编译 Rust 项目
cargo build --release

# 编译 OpenCode (Go)
cd src/opencode
go build -o ../../bin/opencode .
cd ../..
```

### 4. 运行测试

```bash
# Rust 测试
cargo test --workspace

# 格式化检查
cargo fmt --all -- --check

# Lint 检查
cargo clippy --workspace --all-targets -- -D warnings
```

---

## 📝 代码规范

### Rust 代码风格

- 使用 `rustfmt` 格式化代码
- 遵循 Rust 官方风格指南
- 函数/方法添加文档注释

```bash
# 格式化代码
cargo fmt

# 运行 Clippy
cargo clippy -- -D warnings
```

### 提交信息规范

采用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**类型说明：**

| 类型 | 说明 |
|-----|------|
| `feat` | 新功能 |
| `fix` | Bug 修复 |
| `docs` | 文档更新 |
| `style` | 代码格式 (不影响代码运行) |
| `refactor` | 重构 (既不是新功能也不是修复) |
| `perf` | 性能优化 |
| `test` | 测试相关 |
| `chore` | 构建/工具/配置等 |

**示例：**
```bash
git commit -m "feat(cli): 添加超时配置参数"
git commit -m "fix(engine): 修复引擎切换竞态条件"
git commit -m "docs: 更新架构设计文档"
git commit -m "refactor: 优化 .gitignore 配置"
```

---

## 🏗️ 项目结构

```
Dual-Engine/
├── bin/                      # 编译产物
│   ├── de.exe               # CLI 工具
│   ├── det.exe              # TUI 工具
│   ├── opencode.exe         # OpenCode 引擎
│   └── claude.exe           # Claude 引擎
├── crates/                   # Rust 代码包
│   ├── cli/                 # CLI 入口
│   ├── engine-core/         # 引擎核心接口
│   ├── engine-opencode/     # OpenCode 引擎适配
│   ├── engine-claude/       # Claude 引擎适配
│   ├── shared-types/        # 共享类型定义
│   └── tui/                 # TUI 界面
├── docs/                     # 文档中心
│   ├── ARCHITECTURE.md      # 架构设计
│   └── README.md            # 文档索引
├── scripts/                  # 脚本工具
│   ├── setup.sh             # Linux/Mac配置向导
│   └── setup.bat            # Windows 配置向导
├── src/                      # 第三方源码
│   ├── opencode/            # OpenCode (Go)
│   └── claude-code/         # Claude Code (Rust)
├── tests/                    # 测试用例
├── examples/                 # 示例程序
├── Cargo.toml                # Rust 工作区配置
└── README.md                 # 项目说明
```

---

## 🧪 测试指南

### 单元测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定 crate 测试
cargo test -p engine-core

# 运行测试并生成覆盖率报告
cargo tarpaulin --out Html
```

### 集成测试

```bash
# 运行集成测试
cargo test --test '*'

# 运行特定测试
cargo test --test engine_core_tests
```

### 手动测试

```bash
# CLI 模式
./bin/de.exe run -p "hello"

# TUI 模式
./bin/det.exe

# 引擎切换
./bin/de.exe switch opencode
./bin/de.exe switch claude

# 配置向导
./bin/de.exe setup
```

---

## 🔧 开发工具

### 推荐 IDE 配置

**VSCode 扩展：**
- rust-analyzer
- CodeLLDB (调试)
- Cargo Watch
- GitLens

**IntelliJ IDEA / RustRover：**
- 内置 Rust 支持
- 启用 rust-analyzer

### 有用命令

```bash
# 检查代码格式
cargo fmt --all -- --check

# 运行 Clippy
cargo clippy --workspace -- -D warnings

# 生成文档
cargo doc --workspace --open

# 清理构建缓存
cargo clean

# 更新依赖
cargo update

# 查看依赖树
cargo tree
```

---

## 🌿 分支管理

采用 Git Flow 工作流：

- `main` - 生产分支（仅合并稳定版本）
- `develop` - 开发分支（日常开发）
- `feature/*` - 功能分支
- `hotfix/*` - 紧急修复

```bash
# 创建功能分支
git checkout -b feature/your-feature develop

# 完成功能后合并回 develop
git checkout develop
git merge --no-ff feature/your-feature

# 发布时合并到 main
git checkout main
git merge --no-ff develop
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin main --tags
```

---

## 📤 提交流程

1. **Fork 仓库**
2. **创建功能分支**
   ```bash
   git checkout -b feature/your-feature
   ```
3. **开发并提交**
   ```bash
   git add .
   git commit -m "feat: add your feature"
   ```
4. **推送分支**
   ```bash
   git push origin feature/your-feature
   ```
5. **创建 Pull Request**

---

## 🐛 问题排查

### 常见问题

**编译失败：**
```bash
# 清理并重新编译
cargo clean
cargo build --release
```

**依赖冲突：**
```bash
# 更新依赖
cargo update
```

**格式化问题：**
```bash
# 重新格式化
cargo fmt --all
```

### 获取帮助

- 查看 [TROUBLESHOOTING.md](./TROUBLESHOOTING.md)
- 查看 [FAQ](./docs/FAQ.md)
- 提交 Issue: https://github.com/walkzzz/Dual-Engine/issues

---

## 📚 相关文档

- [README.md](./README.md) - 项目说明
- [QUICKSTART.md](./QUICKSTART.md) - 快速开始
- [USAGE.md](./USAGE.md) - 使用指南
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md) - 架构设计
- [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) - 故障排除

---

**Happy Coding!** 🎉