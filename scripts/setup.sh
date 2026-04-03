#!/bin/bash
# Dual-Engine 配置向导
# Dual-Engine Setup Wizard

set -e

echo "🚀 Dual-Engine 配置向导"
echo "========================"
echo ""

# 检测操作系统
OS="$(uname -s)"
case "$OS" in
    Darwin)
        SHELL_RC="$HOME/.zshrc"
        OS_NAME="macOS"
        ;;
    Linux)
        SHELL_RC="$HOME/.bashrc"
        OS_NAME="Linux"
        ;;
    *)
        echo "❌ 不支持的操作系统：$OS"
        exit 1
        ;;
esac

echo "检测到操作系统：$OS_NAME"
echo ""

# 选择 AI 提供商
echo "请选择 AI 提供商："
echo "  1) MoonShot (Kimi) - 推荐中文用户"
echo "  2) DashScope (通义千问)"
echo "  3) Groq - 快速推理"
echo "  4) 跳过配置"
echo ""
read -p "请输入选项 (1-4): " choice

case $choice in
    1)
        PROVIDER="moonshot"
        ENV_VAR="MOONSHOT_API_KEY"
        echo ""
        echo "✅ 已选择：MoonShot (Kimi)"
        echo ""
        echo "📝 请获取 API Key:"
        echo "   访问：https://platform.moonshot.cn/console/api-keys"
        echo ""
        read -p "请输入你的 API Key: " api_key
        ;;
    2)
        PROVIDER="dashscope"
        ENV_VAR="DASHSCOPE_API_KEY"
        echo ""
        echo "✅ 已选择：DashScope (通义千问)"
        echo ""
        echo "📝 请获取 API Key:"
        echo "   访问：https://dashscope.console.aliyun.com/apiKey"
        echo ""
        read -p "请输入你的 API Key: " api_key
        ;;
    3)
        PROVIDER="groq"
        ENV_VAR="GROQ_API_KEY"
        echo ""
        echo "✅ 已选择：Groq"
        echo ""
        echo "📝 请获取 API Key:"
        echo "   访问：https://console.groq.com/keys"
        echo ""
        read -p "请输入你的 API Key: " api_key
        ;;
    4)
        echo ""
        echo "⏭️  跳过配置"
        echo "稍后可以运行 'de setup' 进行配置"
        exit 0
        ;;
    *)
        echo "❌ 无效选项"
        exit 1
        ;;
esac

# 配置 API Key
echo ""
echo "🔧 配置 API Key..."

# 创建配置文件
CONFIG_FILE="$HOME/.opencode.json"
cat > "$CONFIG_FILE" << EOF
{
  "providers": {
    "$PROVIDER": {
      "apiKey": "$api_key",
      "disabled": false
    }
  },
  "agents": {
    "coder": {
      "model": "${PROVIDER}.default",
      "maxTokens": 4096
    }
  }
}
EOF

echo "✅ 配置文件已创建：$CONFIG_FILE"

# 添加到环境变量（可选）
echo ""
read -p "是否添加到环境变量？(推荐，y/n): " add_env

if [ "$add_env" = "y" ]; then
    if ! grep -q "$ENV_VAR" "$SHELL_RC" 2>/dev/null; then
        echo "export $ENV_VAR=\"$api_key\"" >> "$SHELL_RC"
        echo "✅ 已添加到 $SHELL_RC"
        echo ""
        echo "⚠️  需要重新加载配置文件:"
        echo "   source $SHELL_RC"
    else
        echo "ℹ️  环境变量已存在"
    fi
fi

# 测试配置
echo ""
echo "🧪 测试配置..."
echo ""
echo "运行测试命令:"
echo "  de run -p 'hello'"
echo ""

# 完成
echo "============================"
echo "✅ 配置完成！"
echo ""
echo "📚 使用指南:"
echo "  de run -p '问题'           - 运行对话"
echo "  de switch <engine>         - 切换引擎"
echo "  de status                  - 查看状态"
echo "  det                        - TUI 模式"
echo ""
echo "🎉 祝你使用愉快！"
