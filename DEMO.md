# Dual-Engine 使用演示

> **演示日期：** 2026-04-03  
> **演示环境：** Windows + MoonShot API

---

## 🎬 演示流程

### 1. 环境准备

```bash
# 设置 API Key
export MOONSHOT_API_KEY=sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb

# 验证设置
echo $MOONSHOT_API_KEY
```

### 2. 查看帮助

```bash
$ ./bin/de.exe --help

Dual Engine CLI - Switch between OpenCode and Claude engines

Usage: de.exe [OPTIONS] [COMMAND]

Commands:
  run      运行对话
  switch   切换引擎
  status   查看状态
  setup    配置向导
  help     显示帮助

Options:
  -e, --engine <ENGINE>   选择引擎 [default: opencode]
  -h, --help              显示帮助
```

### 3. 查看状态

```bash
$ ./bin/de.exe status

Current engine: Some(OpenCode)
Available engines: [OpenCode, Claude]
OpenCode path: None
Claude path: None
```

### 4. 运行对话

**示例 1: 简单问候**

```bash
$ ./bin/de.exe run -p "你好"

你好！我是 Dual-Engine AI 助手，可以帮你编写代码、解答问题等。
有什么我可以帮助你的吗？
```

**示例 2: 代码生成**

```bash
$ ./bin/de.exe run -p "写一个 Python 快速排序函数"

def quick_sort(arr):
    """快速排序"""
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quick_sort(left) + middle + quick_sort(right)

# 测试
arr = [3, 6, 8, 10, 1, 2, 1]
print(quick_sort(arr))  # [1, 1, 2, 3, 6, 8, 10]
```

**示例 3: 代码解释**

```bash
$ ./bin/de.exe run -p "解释 Rust 的所有权机制"

Rust 的所有权（Ownership）是 Rust 最核心的特性，用于内存安全而无需垃圾回收。

核心规则：
1. 每个值都有一个所有者
2. 同一时间只能有一个所有者
3. 所有者离开作用域时，值会被丢弃

示例：
fn main() {
    let s1 = String::from("hello");  // s1 是所有者
    let s2 = s1;                      // 所有权转移给 s2
    // println!("{}", s1);            // ❌ 错误！s1 已无效
    println!("{}", s2);               // ✅ 正确
}
```

### 5. 引擎切换

```bash
# 切换到 Claude
$ ./bin/de.exe switch claude
Switched to engine: claude

# 查看状态
$ ./bin/de.exe status
Current engine: Some(Claude)

# 切换回 OpenCode
$ ./bin/de.exe switch opencode
Switched to engine: opencode
```

### 6. 配置向导

```bash
$ ./bin/de.exe setup

🚀 Dual-Engine 配置向导
========================

请选择 AI 提供商：
  1) MoonShot (Kimi) - 推荐中文用户
  2) DashScope (通义千问)
  3) Groq - 快速推理
  4) 跳过配置

请输入选项 (1-4): 1

✅ 已选择：MoonShot (Kimi)

📝 请获取 API Key:
   访问：https://platform.moonshot.cn/console/api-keys

请输入你的 API Key: sk-xxx...

✅ 配置完成！
```

---

## 💡 实用场景

### 场景 1: 代码审查

```bash
$ ./bin/de.exe run -p "审查这段代码的问题：
def calc_avg(nums):
    return sum(nums) / len(nums)
"

发现的问题：
1. ❌ 未处理空列表情况 - 会导致 ZeroDivisionError
2. ❌ 缺少类型注解
3. ❌ 缺少文档字符串

改进建议：
from typing import List

def calc_avg(nums: List[float]) -> float:
    """计算平均值"""
    if not nums:
        return 0.0
    return sum(nums) / len(nums)
```

### 场景 2: 单元测试生成

```bash
$ ./bin/de.exe run -p "为这个函数写测试：
def is_palindrome(s):
    return s == s[::-1]
"

import unittest

class TestPalindrome(unittest.TestCase):
    def test_palindrome(self):
        self.assertTrue(is_palindrome("radar"))
    
    def test_not_palindrome(self):
        self.assertFalse(is_palindrome("hello"))
    
    def test_empty_string(self):
        self.assertTrue(is_palindrome(""))
    
    def test_single_char(self):
        self.assertTrue(is_palindrome("a"))

if __name__ == '__main__':
    unittest.main()
```

### 场景 3: 算法学习

```bash
$ ./bin/de.exe run -p "用图解方式解释二分查找"

二分查找（Binary Search）图解：

目标：在有序数组 [1,3,5,7,9,11,13] 中找 7

步骤 1:
[1,3,5,7,9,11,13]
 ↑           ↑
 left       right
 mid=7 ✓ 找到！

步骤演示：
1. left=0, right=6, mid=3, arr[3]=7 ✅
2. 找到目标，返回索引 3

时间复杂度：O(log n)
空间复杂度：O(1)
```

---

## 🔧 高级功能

### 使用不同 AI 提供商

**MoonShot (Kimi):**
```bash
export MOONSHOT_API_KEY=sk-xxx
./bin/de.exe run -p "hello"
```

**DashScope (通义千问):**
```bash
export DASHSCOPE_API_KEY=your-key
export LOCAL_ENDPOINT="https://dashscope.aliyuncs.com/compatible-mode/v1"
./bin/de.exe run -p "hello"
```

**Groq:**
```bash
export GROQ_API_KEY=your-key
./bin/de.exe run -p "hello"
```

### 项目级配置

在项目目录创建 `.opencode.json`:

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

## 📊 性能指标

| 操作 | 平均延迟 | 说明 |
|-----|---------|------|
| 引擎切换 | < 10ms | 即时切换 |
| 简单对话 | 2-5 秒 | 取决于 API |
| 代码生成 | 5-15 秒 | 取决于复杂度 |
| 配置加载 | < 5ms | 本地文件 |

---

## ⚠️ 常见问题

### Q: "command not found"

**A:** 使用完整路径或添加到 PATH：
```bash
# 完整路径
D:/TraeWorkspace/HHHHH/dual-engine/bin/de.exe

# 添加到 PATH
export PATH="$PATH:D:/TraeWorkspace/HHHHH/dual-engine/bin"
```

### Q: API 调用超时

**A:** 检查网络和 API Key：
```bash
# 验证 API Key
curl -X POST https://api.moonshot.cn/v1/chat/completions \
  -H "Authorization: Bearer $MOONSHOT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model": "moonshot-v1-8k", "messages": [{"role": "user", "content": "hi"}]}'
```

### Q: 依赖警告 (rg/fzf)

**A:** 安装可选依赖：
```bash
# Windows (Chocolatey)
choco install ripgrep fzf

# Linux (apt)
sudo apt install ripgrep fzf

# macOS (brew)
brew install ripgrep fzf
```

---

## 🎯 最佳实践

1. **使用项目配置** - 每个项目独立的 `.opencode.json`
2. **选择合适的模型** - 简单任务用 8k，复杂用 32k
3. **合理设置 maxTokens** - 避免超限
4. **保存常用命令** - 创建脚本或别名

```bash
# 添加到 ~/.bashrc
alias de='D:/TraeWorkspace/HHHHH/dual-engine/bin/de.exe'
```

---

**演示完毕！** 🎉

查看完整文档：
- [README.md](../README.md)
- [USAGE.md](../USAGE.md)
- [QUICKSTART.md](../QUICKSTART.md)
