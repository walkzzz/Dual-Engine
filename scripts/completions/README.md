# Dual-Engine Shell 自动补全

## 安装补全

### Bash

```bash
# 生成补全脚本
de completion bash > ~/.local/share/bash-completion/completions/de

# 或添加到 .bashrc
echo 'source <(de completion bash)' >> ~/.bashrc
```

### Zsh

```bash
# 生成补全脚本
de completion zsh > "${fpath[1]}/_de"

# 或添加到 .zshrc
echo 'source <(de completion zsh)' >> ~/.zshrc
```

### PowerShell

```powershell
# 生成补全脚本
de completion powershell > $PROFILE

# 或当前会话
de completion powershell | Out-String | Invoke-Expression
```

### Fish

```fish
# 生成补全脚本
de completion fish > ~/.config/fish/completions/de.fish
```

## 补全功能

### 命令补全

```bash
de <TAB>
# run    运行对话
# switch 切换引擎
# status 查看状态
# setup  配置向导
# completion 生成补全脚本
# help   显示帮助
```

### 选项补全

```bash
de run --<TAB>
# --prompt        -p  提示内容
# --timeout-secs      超时时间 (秒)

de switch <TAB>
# opencode  OpenCode 引擎
# claude    Claude 引擎
```

## 故障排除

### 补全不工作

```bash
# 检查补全文件
ls -la ~/.local/share/bash-completion/completions/de

# 重新加载补全
source ~/.bashrc  # 或重新打开终端

# 测试补全
complete -p de
```

### 手动加载

```bash
# Bash
source <(de completion bash)

# Zsh
source <(de completion zsh)
```