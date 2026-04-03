# 故障排除指南 | Troubleshooting Guide

## ❌ 常见问题与解决方案

### 1. "Unknown engine" 错误

**错误信息：**
```
❌ Unknown engine: xxx
```

**解决方案：**
```bash
# 查看可用引擎
de status

# 切换到有效引擎
de switch opencode
de switch claude
```

---

### 2. API 调用超时

**错误信息：**
```
❌ Request timeout (120 seconds)
```

**解决方案：**

```bash
# 增加超时时间
de run -p "your prompt" --timeout-secs 300

# 检查网络连接
curl https://api.moonshot.cn/v1/models

# 验证 API Key
echo $MOONSHOT_API_KEY
```

---

### 3. "no valid provider available" 错误

**错误信息：**
```
Error: no valid provider available for agent coder
```

**解决方案：**

```bash
# 1. 检查 API Key 是否设置
# Linux/Mac
echo $MOONSHOT_API_KEY

# Windows PowerShell
echo $env:MOONSHOT_API_KEY

# 2. 检查配置文件
cat ~/.opencode.json

# 3. 重新运行配置向导
# Linux/Mac
./scripts/setup.sh

# Windows
.\scripts\setup.bat
```

---

### 4. 命令找不到

**错误信息：**
```
bash: de: command not found
```

**解决方案：**

```bash
# 使用完整路径
./bin/de.exe run -p "hello"

# 或添加到 PATH
# Linux/Mac
echo 'export PATH="$PATH:/path/to/dual-engine/bin"' >> ~/.bashrc
source ~/.bashrc

# Windows PowerShell
$env:PATH += ";C:\path\to\dual-engine\bin"
```

---

### 5. 编译错误

**Rust 编译错误：**
```bash
# 更新 Rust 工具链
rustup update

# 清理并重新编译
cargo clean
cargo build --release
```

**Go 编译错误：**
```bash
# 更新 Go
go version

# 清理并重新编译
cd src/opencode
go clean
go build -o ../../bin/opencode .
```

---

### 6. TUI 无法启动

**错误信息：**
```
det.exe 启动失败
```

**解决方案：**

```bash
# 检查终端兼容性
# 确保使用支持 Unicode 的终端

# Windows: 使用 Windows Terminal
# https://aka.ms/terminal

# Linux/Mac: 使用现代终端
```

---

## 🔧 调试技巧

### 启用调试日志

```bash
# Rust 日志级别
RUST_LOG=debug ./bin/de.exe run -p "test"

# 查看详细输出
./bin/de.exe run -p "test" 2>&1 | tee debug.log
```

### 检查配置

```bash
# 查看加载的配置文件
cat ~/.opencode.json
cat .opencode.json

# 验证 JSON 格式
python -m json.tool ~/.opencode.json
```

### 测试 API 连接

```bash
# MoonShot API 测试
curl -X POST https://api.moonshot.cn/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $MOONSHOT_API_KEY" \
  -d '{"model": "moonshot-v1-8k", "messages": [{"role": "user", "content": "hi"}]}'
```

---

## 📞 获取帮助

### 内置帮助

```bash
de --help
de run --help
de setup
```

### 文档资源

- [README.md](./README.md) - 项目说明
- [QUICKSTART.md](./QUICKSTART.md) - 快速开始
- [USAGE.md](./USAGE.md) - 使用指南
- [GitHub Issues](https://github.com/walkzzz/Dual-Engine/issues) - 问题反馈

### 社区支持

- 提交 Issue: https://github.com/walkzzz/Dual-Engine/issues
- 讨论区：https://github.com/walkzzz/Dual-Engine/discussions

---

## 📋 检查清单

遇到问题时，按顺序检查：

- [ ] API Key 已正确设置
- [ ] 配置文件格式正确
- [ ] 网络连接正常
- [ ] 二进制文件存在且可执行
- [ ] 终端支持 Unicode
- [ ] 有足够的系统资源

---

**最后更新：** 2026-04-03