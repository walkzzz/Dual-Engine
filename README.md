# Dual-Engine CLI

[![CI/CD](https://github.com/walkzzz/Dual-Engine/actions/workflows/ci.yml/badge.svg)](https://github.com/walkzzz/Dual-Engine/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-orange)](https://www.rust-lang.org)

**双引擎 AI 编程助手** - 在 OpenCode 和 Claude Code 之间无缝切换的命令行工具

---

## 🚀 快速开始

### 1. 克隆仓库

```bash
git clone git@github.com:walkzzz/Dual-Engine.git
cd Dual-Engine
```

### 2. 配置 API Key

**方式 A: 环境变量 (临时)**
```bash
# Windows PowerShell
$env:MOONSHOT_API_KEY="sk-your-api-key"

# Linux/Mac
export MOONSHOT_API_KEY="sk-your-api-key"
```

**方式 B: 配置文件 (推荐)**
```bash
# 复制配置模板
cp .opencode.json.example .opencode.json

# 编辑填入你的 API Key
```

### 3. 运行

```bash
# CLI 模式
./bin/de.exe run -p "你好"

# TUI 模式
./bin/det.exe
```

---

## 📦 功能特性

- 🔄 **双引擎切换** - OpenCode / Claude Code 一键切换
- 🤖 **多 AI 提供商** - MoonShot、DashScope、Groq 等
- 📝 **交互式 TUI** - 美观的终端界面
- 🛠️ **丰富工具** - 文件读写、代码执行、搜索等
- 📚 **完整文档** - 详细的使用指南和 API 文档

---

## 📖 文档导航

| 文档 | 说明 |
|-----|------|
| [QUICKSTART.md](./QUICKSTART.md) | 3 分钟快速上手 |
| [USAGE.md](./USAGE.md) | 完整使用指南 |
| [docs/](./docs/) | 技术文档中心 |

---

## 🔧 安装

### 预编译二进制

从 [Releases](https://github.com/walkzzz/Dual-Engine/releases) 下载对应平台的二进制文件

### 源码编译

```bash
# 需要 Rust 1.70+ 和 Go 1.21+
cargo build --release
cd src/opencode && go build -o ../../bin/opencode .
```

---

## 💡 使用示例

### 代码解释
```bash
./bin/de.exe run -p "解释一下快速排序的原理"
```

### 代码生成
```bash
./bin/de.exe run -p "用 Python 写一个 REST API 服务器"
```

### 代码审查
```bash
./bin/de.exe run -p "审查当前目录的代码质量问题"
```

### 切换引擎
```bash
./bin/de.exe switch opencode   # 切换到 OpenCode
./bin/de.exe switch claude     # 切换到 Claude
./bin/de.exe status            # 查看当前状态
```

---

## 🌐 支持的 AI 提供商

| 提供商 | 模型 | 配置方式 |
|-------|------|---------|
| MoonShot | kimi-latest, moonshot-v1-8k/32k/128k | `MOONSHOT_API_KEY` |
| DashScope | qwen2.5-coder-32b-instruct | `DASHSCOPE_API_KEY` + `LOCAL_ENDPOINT` |
| Groq | llama/mixtral 系列 | `GROQ_API_KEY` |
| OpenRouter | 多种开源模型 | `OPENROUTER_API_KEY` |

---

## 🤝 贡献

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](./LICENSE) 文件了解详情

---

## 🙏 致谢

- [OpenCode](https://github.com/opencode-ai/opencode) - AI 编程助手
- [Claude Code](https://github.com/anthropics/claude-code) - Anthropic 的编程助手
- [MoonShot](https://platform.moonshot.cn/) - Kimi 大模型 API

---

**Made with ❤️ by Dual-Engine Team**