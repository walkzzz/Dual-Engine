# Dual-Engine 示例程序

本目录包含 Dual-Engine CLI 的使用示例。

---

## 📋 示例目录

- [基础对话](#基础对话)
- [代码生成](#代码生成)
- [代码审查](#代码审查)
- [引擎切换](#引擎切换)
- [配置管理](#配置管理)

---

## 💬 基础对话

### 简单问候

```bash
# 使用默认引擎
de run -p "你好"

# 带超时设置
de run -p "你好" --timeout-secs 60
```

### 知识问答

```bash
de run -p "解释一下什么是 RESTful API"
de run -p "Python 的装饰器是如何工作的？"
de run -p "解释快速排序的原理和实现"
```

---

## 💻 代码生成

### Python 示例

```bash
de run -p "用 Python 写一个快速排序函数，包含注释和测试用例"
```

预期输出：
```python
def quick_sort(arr):
    """
    快速排序实现
    
    Args:
        arr: 待排序列表
        
    Returns:
        排序后的列表
    """
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quick_sort(left) + middle + quick_sort(right)

# 测试
if __name__ == "__main__":
    test_arr = [3, 6, 8, 10, 1, 2, 1]
    print(f"排序前：{test_arr}")
    print(f"排序后：{quick_sort(test_arr)}")
```

### Rust 示例

```bash
de run -p "用 Rust 写一个线程安全的计数器，使用 Mutex 和 Arc"
```

### Web 开发示例

```bash
de run -p "用 HTML/CSS/JS 写一个待办事项列表应用"
```

---

## 🔍 代码审查

### 审查单个文件

```bash
# 将代码粘贴到 prompt 中
de run -p "审查这段代码的问题和改进建议：[粘贴代码]"
```

### 审查项目代码

```bash
# 进入项目目录
cd your-project

# 审查当前目录
de run -p "审查这个项目的代码质量，指出潜在问题"
```

---

## 🔄 引擎切换

### 查看状态

```bash
de status
```

输出：
```
🔧 Current engine: OpenCode
📦 Available engines: [OpenCode, Claude]
📁 OpenCode path: None
📁 Claude path: None
```

### 切换引擎

```bash
# 切换到 OpenCode
de switch opencode

# 切换到 Claude
de switch claude

# 验证切换
de status
```

---

## ⚙️ 配置管理

### 使用配置向导

```bash
# 交互式配置
de setup
```

### 手动配置

**全局配置** (`~/.opencode.json`)：

```json
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
```

**项目配置** (`.opencode.json`)：

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

---

## 🧪 高级用法

### 使用不同 API 提供商

**MoonShot (Kimi)：**
```bash
export MOONSHOT_API_KEY="sk-your-key"
de run -p "hello"
```

**DashScope (通义千问)：**
```bash
export DASHSCOPE_API_KEY="your-key"
export LOCAL_ENDPOINT="https://dashscope.aliyuncs.com/compatible-mode/v1"
de run -p "hello"
```

**Groq：**
```bash
export GROQ_API_KEY="your-key"
de run -p "hello"
```

### 增加超时时间

```bash
# 默认 120 秒
de run -p "分析这个项目"

# 自定义 300 秒
de run -p "分析这个项目" --timeout-secs 300
```

### 启动 TUI 界面

```bash
# 交互式 TUI
det
```

---

## 📚 更多示例

查看以下文档获取更多使用场景：

- [QUICKSTART.md](../QUICKSTART.md) - 快速开始
- [USAGE.md](../USAGE.md) - 完整使用指南
- [FAQ.md](./FAQ.md) - 常见问题

---

**Happy Coding!** 🎉