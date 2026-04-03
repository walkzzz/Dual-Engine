# Dual-Engine FAQ - 常见问题解答

> **最后更新：** 2026-04-03

---

## 📌 目录

- [安装问题](#安装问题)
- [配置问题](#配置问题)
- [使用问题](#使用问题)
- [性能问题](#性能问题)
- [开发相关](#开发相关)

---

## 安装问题

### Q: 运行时提示找不到 `de.exe` 或 `de` 命令？

**A:** 请确认：

1. **使用完整路径运行**
   ```bash
   D:/TraeWorkspace/HHHHH/dual-engine/bin/de.exe run -p "hello"
   ```

2. **或添加到 PATH 环境变量**
   ```bash
   # Windows PowerShell
   $env:PATH += ";D:\TraeWorkspace\HHHHH\dual-engine\bin"
   
   # Linux/Mac
   export PATH="$PATH:/path/to/dual-engine/bin"
   ```

3. **运行全局安装脚本**
   ```bash
   # Windows
   .\install.bat
   
   # Linux/Mac
   ./install.sh
   ```

---

### Q: 编译失败，提示缺少依赖？

**A:** 安装必要的工具链：

```bash
# 安装 Rust (1.70+)
rustup install stable

# 安装 Go (1.21+) - 用于 OpenCode
# https://go.dev/dl/
```

---

## 配置问题

### Q: "no valid provider available" 错误？

**A:** API Key 未正确配置。

**解决方案：**

```bash
# 1. 检查 API Key 是否设置
echo $MOONSHOT_API_KEY

# 2. 检查配置文件
cat ~/.opencode.json

# 3. 使用配置向导
de setup

# 4. 手动配置
cat > ~/.opencode.json << EOF
{
  "providers": {
    "moonshot": {
      "apiKey": "sk-your-api-key",
      "disabled": false
    }
  },
  "agents": {
    "coder": {
      "model": "moonshot.moonshot-v1-32k",
      "maxTokens": 4096
    }
  }
}
EOF
```

---

### Q: 如何切换 AI 提供商？

**A:** 修改配置文件 `~/.opencode.json`：

```json
{
  "providers": {
    "moonshot": {
      "apiKey": "your-moonshot-key",
      "disabled": false
    },
    "dashscope": {
      "apiKey": "your-dashscope-key",
      "disabled": true
    }
  }
}
```

---

## 使用问题

### Q: API 调用超时？

**A:** 增加超时时间或检查网络：

```bash
# 增加超时时间（默认 120 秒）
de run -p "hello" --timeout-secs 300

# 检查网络连接
curl https://api.moonshot.cn/v1/models

# 验证 API Key
curl -X POST https://api.moonshot.cn/v1/chat/completions \
  -H "Authorization: Bearer $MOONSHOT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model": "moonshot-v1-8k", "messages": [{"role": "user", "content": "hi"}]}'
```

---

### Q: 如何查看当前引擎状态？

**A:** 使用 `status` 命令：

```bash
de status
```

输出示例：
```
🔧 Current engine: OpenCode
📦 Available engines: [OpenCode, Claude]
📁 OpenCode path: None
📁 Claude path: None
```

---

### Q: 如何在项目中使用不同配置？

**A:** 在项目根目录创建 `.opencode.json`：

```json
{
  "providers": {
    "moonshot": {
      "apiKey": "project-specific-key",
      "disabled": false
    }
  },
  "agents": {
    "coder": {
      "model": "moonshot.moonshot-v1-8k",
      "maxTokens": 2048
    }
  }
}
```

本地配置会覆盖全局配置 `~/.opencode.json`。

---

## 性能问题

### Q: 内存占用过高？

**A:** 引擎会自动释放闲置资源，也可手动清理：

```bash
# 查看资源占用
de status

# 重启 CLI 释放资源
# 或等待自动回收（默认 5 分钟闲置）
```

---

### Q: 响应速度慢？

**A:** 可能原因：

1. **网络延迟** - 检查 API 连接
2. **模型过大** - 使用较小的模型
3. **Prompt 过长** - 简化输入

```bash
# 使用更快的模型
# moonshot-v1-8k > moonshot-v1-32k > moonshot-v1-128k

# 减少 maxTokens
# 在配置文件中设置
{
  "agents": {
    "coder": {
      "maxTokens": 1024
    }
  }
}
```

---

## 开发相关

### Q: 如何贡献代码？

**A:** 参考以下步骤：

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
4. **推送并创建 PR**
   ```bash
   git push origin feature/your-feature
   ```

详见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

---

### Q: 如何运行测试？

**A:**

```bash
# Rust 测试
cargo test --workspace

# 格式化检查
cargo fmt --all -- --check

# Lint 检查
cargo clippy --workspace -- -D warnings
```

---

### Q: 如何调试问题？

**A:** 启用详细日志：

```bash
# 设置日志级别
export RUST_LOG=debug

# 运行并保存日志
de run -p "test" 2>&1 | tee debug.log
```

---

## 📞 其他帮助

### 文档资源

- [README.md](./README.md) - 项目说明
- [QUICKSTART.md](./QUICKSTART.md) - 快速开始
- [USAGE.md](./USAGE.md) - 使用指南
- [CONTRIBUTING.md](./docs/CONTRIBUTING.md) - 贡献指南
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md) - 架构设计
- [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) - 故障排除

### 社区支持

- **GitHub Issues:** https://github.com/walkzzz/Dual-Engine/issues
- **GitHub Discussions:** https://github.com/walkzzz/Dual-Engine/discussions

---

**未找到答案？** 请提交 Issue 或参与 Discussions 讨论！