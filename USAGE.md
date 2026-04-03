# Dual-Engine CLI 使用指南

## 📖 目录

1. [快速开始](#快速开始)
2. [配置说明](#配置说明)
3. [CLI 命令](#cli-命令)
4. [使用案例](#使用案例)
5. [常见问题](#常见问题)

---

## 🚀 快速开始

### 1. 设置 API Key

**MoonShot (推荐)**
```bash
# Windows PowerShell
$env:MOONSHOT_API_KEY="sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb"

# Windows CMD
set MOONSHOT_API_KEY=sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb

# Linux/Mac
export MOONSHOT_API_KEY=sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb
```

**DashScope (通义千问)**
```bash
export DASHSCOPE_API_KEY="your-dashscope-key"
export LOCAL_ENDPOINT="https://dashscope.aliyuncs.com/compatible-mode/v1"
```

### 2. 运行第一个命令

```bash
# 进入项目目录
cd D:/TraeWorkspace/HHHHH/dual-engine

# 运行简单对话
./bin/de.exe run -p "你好"
```

---

## ⚙️ 配置说明

### 方式一：环境变量（临时）

```bash
# 设置 API Key（每次打开终端需要重新设置）
export MOONSHOT_API_KEY="your-key"
```

### 方式二：配置文件（推荐）

**全局配置** `~/.opencode.json` (Windows: `C:\Users\你的用户名\.opencode.json`)

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

**项目配置** `.opencode.json` (放在项目根目录)

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
      "maxTokens": 2048
    },
    "summarizer": {
      "model": "moonshot.moonshot-v1-8k",
      "maxTokens": 1024
    }
  }
}
```

### 可用模型

| 提供商 | 模型 ID | 上下文 | 说明 |
|-------|--------|--------|------|
| MoonShot | `moonshot.moonshot-v1-8k` | 8K | 快速、经济 |
| MoonShot | `moonshot.moonshot-v1-32k` | 32K | **推荐**，平衡 |
| MoonShot | `moonshot.moonshot-v1-128k` | 128K | 超长文档 |
| DashScope | `dashscope.qwen2.5-coder-32b-instruct` | 32K | 编程专用 |
| DashScope | `dashscope.qwen-plus` | 32K | 通用 |

---

## 💻 CLI 命令

### 查看帮助

```bash
# 查看主帮助
./bin/de.exe --help

# 查看子命令帮助
./bin/de.exe run --help
./bin/de.exe switch --help
./bin/de.exe status --help
```

### 引擎切换

```bash
# 查看当前引擎
./bin/de.exe status

# 切换到 OpenCode 引擎
./bin/de.exe switch opencode

# 切换到 Claude 引擎
./bin/de.exe switch claude
```

### 运行对话

```bash
# 简单对话
./bin/de.exe run -p "解释一下什么是递归"

# 代码相关
./bin/de.exe run -p "帮我写一个快速排序函数"

# 代码审查
./bin/de.exe run -p "审查这个文件的代码风格"
```

### TUI 界面

```bash
# 启动 TUI 界面（交互式）
./bin/det.exe
```

---

## 📚 使用案例

### 案例 1: 解释代码概念

```bash
./bin/de.exe run -p "用简单的例子解释什么是闭包"
```

**输出示例：**
```
闭包是指函数可以访问其外部作用域的变量，即使函数在外部作用域之外执行。

例如（JavaScript）：
function outer() {
    let count = 0;
    return function inner() {
        count++;
        return count;
    }
}

const counter = outer();
counter(); // 1
counter(); // 2
```

### 案例 2: 生成代码

```bash
./bin/de.exe run -p "用 Python 写一个读取 CSV 文件并统计每行数据的函数"
```

**输出示例：**
```python
import csv

def count_csv_rows(filename):
    """读取 CSV 文件并返回行数"""
    with open(filename, 'r', encoding='utf-8') as f:
        reader = csv.reader(f)
        rows = list(reader)
    return len(rows)

# 使用示例
row_count = count_csv_rows('data.csv')
print(f'共有 {row_count} 行数据')
```

### 案例 3: 代码审查

```bash
./bin/de.exe run -p "审查当前目录的 main.py 文件，指出潜在问题"
```

### 案例 4: 重构代码

```bash
./bin/de.exe run -p "重构这段代码，使其更符合 Python PEP8 规范"
```

### 案例 5: 编写测试

```bash
./bin/de.exe run -p "为这个函数编写单元测试"
```

### 案例 6: 调试帮助

```bash
./bin/de.exe run -p "这个错误是什么意思：IndexError: list index out of range"
```

### 案例 7: 生成文档

```bash
./bin/de.exe run -p "为这个函数生成文档字符串"
```

### 案例 8: 算法实现

```bash
./bin/de.exe run -p "实现一个二分查找算法，包含注释"
```

---

## 🔧 进阶用法

### 在项目中使用

1. 在项目根目录创建 `.opencode.json`
2. 运行命令时会自动读取项目配置

```bash
cd D:/TraeWorkspace/HHHHH/my-project

# 创建项目配置
echo '{
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
}' > .opencode.json

# 运行
D:/TraeWorkspace/HHHHH/dual-engine/bin/de.exe run -p "帮我优化这个项目的代码结构"
```

### 使用不同引擎

```bash
# 使用 OpenCode + MoonShot
./bin/de.exe switch opencode
./bin/de.exe run -p "分析这段代码"

# 使用 Claude（需要 ANTHROPIC_API_KEY）
./bin/de.exe switch claude
./bin/de.exe run -p "分析这段代码"
```

---

## ❓ 常见问题

### Q1: "no valid provider available" 错误

**原因：** API Key 未设置或配置文件错误

**解决：**
```bash
# 检查环境变量
echo $MOONSHOT_API_KEY

# 检查配置文件
cat ~/.opencode.json
```

### Q2: "token limit exceeded" 错误

**原因：** 请求的 token 数超过模型限制

**解决：** 使用更大的模型或减少 `maxTokens`
```json
{
  "agents": {
    "coder": {
      "model": "moonshot.moonshot-v1-32k",
      "maxTokens": 2048
    }
  }
}
```

### Q3: 命令找不到

**解决：** 使用完整路径或添加到 PATH
```bash
# 临时使用完整路径
D:/TraeWorkspace/HHHHH/dual-engine/bin/de.exe

# 添加到 PATH（Windows）
setx PATH "%PATH%;D:\TraeWorkspace\HHHHH\dual-engine\bin"
```

### Q4: API Key 无效

**解决：**
1. 检查 Key 是否正确复制（无空格）
2. 在 MoonShot 控制台验证 Key 状态
3. 确认 Key 有足够的额度

### Q5: 响应太慢

**解决：**
1. 使用较小的模型（8k 比 32k 快）
2. 减少 `maxTokens` 值
3. 检查网络连接

---

## 📞 获取帮助

遇到问题可以：

1. 查看调试日志
```bash
./bin/de.exe run -p "hello" 2>&1 | tee debug.log
```

2. 检查配置文件
```bash
cat ~/.opencode.json
cat .opencode.json
```

3. 测试 API 连接
```bash
curl -X POST https://api.moonshot.cn/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_KEY" \
  -d '{"model": "moonshot-v1-8k", "messages": [{"role": "user", "content": "hi"}]}'
```

---

## 🎯 最佳实践

1. **使用项目配置** - 每个项目独立的 `.opencode.json`
2. **合理设置 maxTokens** - 避免超限
3. **选择合适的模型** - 简单任务用 8k，复杂用 32k
4. **保存常用命令** - 创建脚本或别名

```bash
# 添加到 ~/.bashrc 或 ~/.zshrc
alias de='D:/TraeWorkspace/HHHHH/dual-engine/bin/de.exe'
alias det='D:/TraeWorkspace/HHHHH/dual-engine/bin/det.exe'
```

---

**祝使用愉快！** 🎉