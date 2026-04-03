# Dual-Engine 快速参考卡

## 🚀 3 分钟上手

```bash
# 1. 设置 API Key
export MOONSHOT_API_KEY=sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb

# 2. 运行第一个命令
cd D:/TraeWorkspace/HHHHH/dual-engine
./bin/de.exe run -p "你好"

# 3. 完成！
```

---

## 📋 常用命令

| 命令 | 说明 |
|-----|------|
| `de run -p "问题"` | 运行对话 |
| `de status` | 查看状态 |
| `de switch opencode` | 切换 OpenCode |
| `de switch claude` | 切换 Claude |
| `det` | TUI 界面 |

---

## 🔑 API Key 配置

### MoonShot（推荐新手）
```bash
export MOONSHOT_API_KEY=sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb
```

### DashScope
```bash
export DASHSCOPE_API_KEY=your-key
export LOCAL_ENDPOINT=https://dashscope.aliyuncs.com/compatible-mode/v1
```

---

## 💡 实用示例

```bash
# 解释概念
de run -p "什么是 RESTful API"

# 生成代码
de run -p "写一个 Python 快速排序"

# 代码审查
de run -p "审查 main.py 的问题"

# 编写测试
de run -p "为这个函数写单元测试"

# 调试帮助
de run -p "IndexError 是什么意思"
```

---

## ⚙️ 配置文件模板

**~/.opencode.json**
```json
{
  "providers": {
    "moonshot": {
      "apiKey": "你的 key",
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

---

## 🎯 模型选择

| 场景 | 推荐模型 |
|-----|---------|
| 日常对话 | `moonshot-v1-8k` |
| 代码开发 | `moonshot-v1-32k` |
| 长文档 | `moonshot-v1-128k` |
| 编程专用 | `qwen2.5-coder-32b` |

---

## ❗ 常见错误

| 错误 | 解决 |
|-----|------|
| `no valid provider` | 检查 API Key |
| `token limit` | 用更大模型或减小 maxTokens |
| `command not found` | 使用完整路径 |

---

## 📖 完整文档

查看 `USAGE.md` 获取详细指南。