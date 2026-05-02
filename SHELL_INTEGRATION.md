# TermGenius 终端集成指南

本文档介绍如何将 TermGenius 集成到你的终端环境中，以获得更好的使用体验。

## 功能特性

终端集成提供以下增强功能：

- **快捷命令**：简化常用操作
- **命令补全**：Tab 键自动补全子命令和参数
- **交互式执行**：生成命令后可选择是否执行
- **历史搜索**：快速搜索历史记录

## Bash/Zsh 集成

### 安装步骤

1. **复制集成脚本**

```bash
# 创建目录
mkdir -p ~/.termgenius

# 复制脚本
cp scripts/shell-integration.sh ~/.termgenius/
```

2. **添加到 Shell 配置**

**Bash 用户**：编辑 `~/.bashrc`

```bash
# 在文件末尾添加
source ~/.termgenius/shell-integration.sh
```

**Zsh 用户**：编辑 `~/.zshrc`

```bash
# 在文件末尾添加
source ~/.termgenius/shell-integration.sh
```

3. **重新加载配置**

```bash
# Bash
source ~/.bashrc

# Zsh
source ~/.zshrc
```

### 可用命令

| 命令 | 说明 | 示例 |
|------|------|------|
| `tg` | TermGenius 别名 | `tg "列出文件"` |
| `tgc` | 生成命令（不执行） | `tgc 列出文件` |
| `tgx` | 生成并执行命令（需确认） | `tgx 列出文件` |
| `tgcode` | 生成代码 | `tgcode Python读取CSV` |
| `tgfix` | 修复失败的命令 | `tgfix` |
| `tgh` | 查看/搜索历史 | `tgh git` |
| `tgconf` | 查看配置 | `tgconf` |

### 命令补全

按 `Tab` 键可以自动补全：

```bash
tg <Tab>          # 显示所有子命令
tg history <Tab>  # 显示 history 的参数
tg config <Tab>   # 显示 config 的参数
```

### 使用示例

```bash
# 快速生成命令
$ tgc 查找大于100MB的文件
🤖 正在生成命令...
✅ 生成的命令:
   find . -type f -size +100M

# 生成并执行（需确认）
$ tgx 列出当前目录文件
🤖 正在生成命令...

📋 生成的命令:
   ls -la

确认执行? (y/N) y
⚡ 执行中...
total 48
drwxr-xr-x  12 user  staff   384 May  2 11:30 .
...

# 搜索历史
$ tgh git
🔍 搜索结果 (找到 3 条):
...
```

## PowerShell 集成

### 安装步骤

1. **复制集成脚本**

```powershell
# 创建目录
New-Item -ItemType Directory -Force -Path $HOME\.termgenius

# 复制脚本
Copy-Item scripts\shell-integration.ps1 $HOME\.termgenius\
```

2. **添加到 PowerShell 配置**

```powershell
# 查看配置文件位置
$PROFILE

# 编辑配置文件（如果不存在会创建）
notepad $PROFILE

# 在文件末尾添加
. $HOME\.termgenius\shell-integration.ps1
```

3. **设置执行策略**（如果需要）

```powershell
# 允许运行本地脚本
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

4. **重新加载配置**

```powershell
# 重启 PowerShell 或执行
. $PROFILE
```

### 可用命令

与 Bash/Zsh 相同的命令集：

```powershell
tg "列出文件"
tgc 列出文件
tgx 列出文件
tgcode Python读取CSV
tgfix
tgh git
tgconf
```

### 命令补全

PowerShell 的 Tab 补全会自动工作：

```powershell
tg <Tab>          # 循环显示子命令
tg history --<Tab> # 显示 history 的参数
```

## 高级配置

### 自动错误捕获（可选）

可以配置终端在命令失败时自动提示使用 `tgfix`。

**Bash/Zsh**：

在 `shell-integration.sh` 中取消注释以下代码：

```bash
trap 'tg_auto_fix' ERR
tg_auto_fix() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        echo ""
        echo "💡 提示: 命令执行失败，使用 'tgfix' 获取修复建议"
    fi
}
```

**PowerShell**：

在 `shell-integration.ps1` 中取消注释相关代码。

**注意**：此功能可能会干扰某些脚本的正常运行，请谨慎使用。

### 自定义快捷命令

你可以在集成脚本中添加自己的快捷命令。

**示例**：添加一个快捷命令用于生成 Git 命令

```bash
# Bash/Zsh
tggit() {
    tgc "git $*"
}

# PowerShell
function tggit {
    param([Parameter(ValueFromRemainingArguments=$true)][string[]]$Args)
    tgc "git $($Args -join ' ')"
}
```

使用：

```bash
tggit 提交所有更改
# 等同于: tgc "git 提交所有更改"
```

## 故障排除

### Bash/Zsh: 脚本无法加载

**问题**：
```
bash: ~/.termgenius/shell-integration.sh: Permission denied
```

**解决**：
```bash
chmod +x ~/.termgenius/shell-integration.sh
```

### PowerShell: 脚本被阻止

**问题**：
```
无法加载文件，因为在此系统上禁止运行脚本
```

**解决**：
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### 命令补全不工作

**Bash**：确保安装了 `bash-completion`

```bash
# Ubuntu/Debian
sudo apt install bash-completion

# macOS
brew install bash-completion
```

**Zsh**：确保启用了补全系统

```bash
# 在 ~/.zshrc 中添加
autoload -Uz compinit
compinit
```

### tgx 无法提取命令

**问题**：生成的命令格式可能与脚本预期不符

**解决**：
1. 检查 TermGenius 输出格式
2. 调整脚本中的正则表达式
3. 或直接使用 `tgc` 手动复制命令

## 卸载

### Bash/Zsh

1. 从 `~/.bashrc` 或 `~/.zshrc` 中删除以下行：
```bash
source ~/.termgenius/shell-integration.sh
```

2. 删除脚本文件：
```bash
rm ~/.termgenius/shell-integration.sh
```

3. 重新加载配置：
```bash
source ~/.bashrc  # 或 source ~/.zshrc
```

### PowerShell

1. 从 `$PROFILE` 中删除以下行：
```powershell
. $HOME\.termgenius\shell-integration.ps1
```

2. 删除脚本文件：
```powershell
Remove-Item $HOME\.termgenius\shell-integration.ps1
```

3. 重启 PowerShell

## 最佳实践

1. **先测试再启用自动错误捕获**：自动错误捕获可能影响某些脚本
2. **定期更新集成脚本**：随着 TermGenius 更新，集成脚本也可能需要更新
3. **自定义快捷命令**：根据自己的使用习惯添加快捷命令
4. **使用 tgx 时要小心**：始终检查生成的命令再执行

## 反馈

如果你有改进建议或发现问题，请：

- 提交 Issue: https://github.com/liujycode/TermGenius/issues
- 或发送邮件: liujycode@foxmail.com

---

更多信息请查看：
- [使用指南](USAGE.md)
- [README](README.md)
