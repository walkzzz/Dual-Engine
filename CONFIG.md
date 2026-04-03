# Dual-Engine 配置说明

> **更新日期：** 2026-04-03  
> **问题修复：** 引擎与 API 密钥不匹配问题

---

## 🎯 问题说明

**之前的问题：**
- 设置了 `MOONSHOT_API_KEY`
- 但运行的是 `OpenCode` 引擎
- 两者不匹配 → 报错

**现在的解决方案：** 使用 `--provider` 参数

---

## ✅ 正确用法

### 方式 1：使用 --provider 参数（推荐）

```bash
# MoonShot (月之暗面)
export MOONSHOT_API_KEY=sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb
./bin/de.exe --provider moonshot run -p "你好"

# DashScope (通义千问)
export DASHSCOPE_API_KEY=your-key
./bin/de.exe --provider dashscope run -p "hello"

# Groq
export GROQ_API_KEY=your-key
./bin/de.exe --provider groq run -p "hi"
```

### 方式 2：使用配置文件

创建 `~/.opencode.json`：

```json
{
  "providers": {
    "moonshot": {
      "apiKey": "sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb",
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
```

然后运行：
```bash
./bin/de.exe run -p "你好"
```

### 方式 3：使用配置向导

```bash
./bin/de.exe setup
```

按照提示选择 AI 提供商并输入 API Key。

---

## 🔧 可用提供商

| 提供商 | 环境变量 | 端点 | 推荐模型 |
|-------|---------|------|---------|
| **MoonShot** | `MOONSHOT_API_KEY` | `https://api.moonshot.cn/v1` | `moonshot-v1-32k` |
| **DashScope** | `DASHSCOPE_API_KEY` | `https://dashscope.aliyuncs.com/compatible-mode/v1` | `qwen2.5-coder-32b` |
| **Groq** | `GROQ_API_KEY` | `https://api.groq.com/openai/v1` | `mixtral-8x7b` |

---

## 📝 完整示例

### MoonShot 示例

```bash
# 1. 设置 API Key
export MOONSHOT_API_KEY=sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb

# 2. 运行对话
./bin/de.exe --provider moonshot run -p "用 Rust 写一个阶乘函数"

# 3. 查看输出
# 应该看到 AI 回复的代码
```

### DashScope 示例

```bash
# 1. 设置 API Key
export DASHSCOPE_API_KEY=sk-your-key

# 2. 运行对话
./bin/de.exe --provider dashscope run -p "解释什么是快速排序"
```

---

## ⚠️ 常见错误

### 错误 1：引擎与 API 密钥不匹配

**错误信息：**
```
Error: Engine error: OpenCode error: no valid provider available
```

**解决方法：**
```bash
# 使用 --provider 参数
./bin/de.exe --provider moonshot run -p "你好"
```

### 错误 2：API Key 未设置

**错误信息：**
```
Error: Config error: Missing API key: moonshot
```

**解决方法：**
```bash
export MOONSHOT_API_KEY=sk-xxx
```

### 错误 3：超时

**错误信息：**
```
Error: Request timeout (120 seconds)
```

**解决方法：**
```bash
# 增加超时时间
./bin/de.exe --provider moonshot run -p "分析这个项目" --timeout-secs 300
```

---

## 🎯 验证配置

```bash
# 1. 检查环境变量
echo $MOONSHOT_API_KEY

# 2. 检查配置文件
cat ~/.opencode.json

# 3. 测试连接
./bin/de.exe --provider moonshot run -p "hi"
```

---

## 📚 相关文档

- [README.md](./README.md) - 项目说明
- [DEMO.md](./DEMO.md) - 使用演示
- [USAGE.md](./USAGE.md) - 完整使用指南
- [QUICKSTART.md](./QUICKSTART.md) - 快速开始

---

**修复完成！现在可以正常使用 MoonShot/DashScope/Groq API 了！** 🎉
