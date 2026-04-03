#!/bin/bash
# Dual-Engine 完整演示脚本
# 使用方法：./demo.sh

set -e

echo "========================================"
echo "  Dual-Engine 完整功能演示"
echo "========================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 设置 API Key
export MOONSHOT_API_KEY="sk-2AHag06yfVKXiLzz8CTs95gGkyBuH8PJA6mC6INQkYBTBzsb"

echo -e "${BLUE}步骤 1: 验证环境${NC}"
echo "----------------------------------------"

# 检查 rg
if command -v rg &> /dev/null; then
    echo -e "${GREEN}✓${NC} ripgrep 已安装：$(rg --version | head -1)"
else
    echo -e "${YELLOW}!${NC} ripgrep 未安装 (可选)"
fi

# 检查 fzf
if command -v fzf &> /dev/null; then
    echo -e "${GREEN}✓${NC} fzf 已安装：$(fzf --version)"
else
    echo -e "${YELLOW}!${NC} fzf 未安装 (可选，某些功能受限)"
fi

echo ""
echo -e "${BLUE}步骤 2: 查看帮助${NC}"
echo "----------------------------------------"
./bin/de.exe --help

echo ""
echo -e "${BLUE}步骤 3: 查看当前状态${NC}"
echo "----------------------------------------"
./bin/de.exe status

echo ""
echo -e "${BLUE}步骤 4: 引擎切换演示${NC}"
echo "----------------------------------------"
echo "切换到 Claude..."
./bin/de.exe switch claude
echo ""
echo "当前引擎:"
./bin/de.exe status | head -1

echo ""
echo "切换回 OpenCode..."
./bin/de.exe switch opencode
echo ""
echo "当前引擎:"
./bin/de.exe status | head -1

echo ""
echo -e "${BLUE}步骤 5: 运行对话任务${NC}"
echo "----------------------------------------"

# 任务 1: 简单问候
echo -e "${YELLOW}任务 1: 简单问候${NC}"
echo "命令：de run -p \"你好\""
echo "结果:"
echo "---"
timeout 30 ./bin/de.exe run -p "你好" 2>&1 | grep -v "INFO\|WARN" || echo "[超时或错误]"
echo "---"
echo ""

# 任务 2: 代码生成
echo -e "${YELLOW}任务 2: 代码生成 (Rust 阶乘)${NC}"
echo "命令：de run -p \"写一个 Rust 函数计算阶乘\""
echo "结果:"
echo "---"
timeout 60 ./bin/de.exe run -p "写一个 Rust 函数计算阶乘" 2>&1 | grep -v "INFO\|WARN" || echo "[超时或错误]"
echo "---"
echo ""

# 任务 3: 代码解释
echo -e "${YELLOW}任务 3: 代码解释${NC}"
echo "命令：de run -p \"解释什么是快速排序\""
echo "结果:"
echo "---"
timeout 60 ./bin/de.exe run -p "解释什么是快速排序" 2>&1 | grep -v "INFO\|WARN" || echo "[超时或错误]"
echo "---"
echo ""

echo -e "${BLUE}步骤 6: 演示完成${NC}"
echo "----------------------------------------"
echo ""
echo -e "${GREEN}✓${NC} 所有演示完成！"
echo ""
echo "提示:"
echo "  - 查看完整文档：cat README.md"
echo "  - 使用演示文档：cat DEMO.md"
echo "  - 查看优化总结：cat docs/OPTIMIZATION_SUMMARY.md"
echo ""