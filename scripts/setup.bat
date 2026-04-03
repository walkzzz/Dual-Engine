@echo off
REM Dual-Engine Setup Wizard for Windows
REM Dual-Engine 配置向导 (Windows 版)

setlocal enabledelayedexpansion

echo 🚀 Dual-Engine 配置向导
echo ========================
echo.

echo 请选择 AI 提供商：
echo   1) MoonShot (Kimi) - 推荐中文用户
echo   2) DashScope (通义千问)
echo   3) Groq - 快速推理
echo   4) 跳过配置
echo.
set /p choice="请输入选项 (1-4): "

if "%choice%"=="1" (
    set PROVIDER=moonshot
    set ENV_VAR=MOONSHOT_API_KEY
    echo.
    echo ✅ 已选择：MoonShot (Kimi)
    echo.
    echo 📝 请获取 API Key:
    echo    访问：https://platform.moonshot.cn/console/api-keys
    echo.
) else if "%choice%"=="2" (
    set PROVIDER=dashscope
    set ENV_VAR=DASHSCOPE_API_KEY
    echo.
    echo ✅ 已选择：DashScope (通义千问)
    echo.
    echo 📝 请获取 API Key:
    echo    访问：https://dashscope.console.aliyun.com/apiKey
    echo.
) else if "%choice%"=="3" (
    set PROVIDER=groq
    set ENV_VAR=GROQ_API_KEY
    echo.
    echo ✅ 已选择：Groq
    echo.
    echo 📝 请获取 API Key:
    echo    访问：https://console.groq.com/keys
    echo.
) else if "%choice%"=="4" (
    echo.
    echo ⏭️  跳过配置
    echo 稍后可以运行 'de setup' 进行配置
    exit /b 0
) else (
    echo ❌ 无效选项
    exit /b 1
)

set /p api_key="请输入你的 API Key: "

echo.
echo 🔧 配置 API Key...

REM 创建配置文件
set CONFIG_FILE=%USERPROFILE%\.opencode.json
(
echo {
echo   "providers": {
echo     "%PROVIDER%": {
echo       "apiKey": "%api_key%",
echo       "disabled": false
echo     }
echo   },
echo   "agents": {
echo     "coder": {
echo       "model": "%PROVIDER%.default",
echo       "maxTokens": 4096
echo     }
echo   }
echo }
) > "%CONFIG_FILE%"

echo ✅ 配置文件已创建：%CONFIG_FILE%

REM 设置环境变量
echo.
setx %ENV_VAR% "%api_key%" >nul 2>&1
echo ✅ 已设置环境变量：%ENV_VAR%
echo.
echo ⚠️  需要重新打开终端才能生效

REM 完成
echo.
echo ========================
echo ✅ 配置完成！
echo.
echo 📚 使用指南:
echo   de run -p '问题'           - 运行对话
echo   de switch ^<engine^>         - 切换引擎
echo   de status                  - 查看状态
echo   det                        - TUI 模式
echo.
echo 🎉 祝你使用愉快！

pause
