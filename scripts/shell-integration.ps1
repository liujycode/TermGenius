# TermGenius Shell Integration for PowerShell
#
# 安装方法：
# 1. 将此文件保存到 $HOME\.termgenius\shell-integration.ps1
# 2. 在 PowerShell 配置文件中添加：
#    . $HOME\.termgenius\shell-integration.ps1
#
# 查找 PowerShell 配置文件位置：
#    $PROFILE

# TermGenius 别名
Set-Alias tg termgenius

# 快捷函数：生成并显示命令（不执行）
function tgc {
    param(
        [Parameter(Mandatory=$true, ValueFromRemainingArguments=$true)]
        [string[]]$Description
    )

    $prompt = $Description -join " "
    termgenius $prompt
}

# 快捷函数：生成并执行命令（需要确认）
function tgx {
    param(
        [Parameter(Mandatory=$true, ValueFromRemainingArguments=$true)]
        [string[]]$Description
    )

    $prompt = $Description -join " "

    Write-Host "🤖 正在生成命令..." -ForegroundColor Cyan

    # 生成命令
    $output = termgenius $prompt 2>$null | Out-String

    # 提取命令（查找 "生成的命令:" 后的内容）
    $lines = $output -split "`n"
    $cmdLine = $null
    $foundMarker = $false

    foreach ($line in $lines) {
        if ($line -match "生成的命令") {
            $foundMarker = $true
            continue
        }
        if ($foundMarker -and $line.Trim() -ne "") {
            # 移除 ANSI 颜色代码
            $cmdLine = $line -replace '\x1b\[[0-9;]*m', '' -replace '^\s+', ''
            break
        }
    }

    if ([string]::IsNullOrWhiteSpace($cmdLine)) {
        Write-Host "❌ 命令生成失败" -ForegroundColor Red
        return
    }

    Write-Host ""
    Write-Host "📋 生成的命令:" -ForegroundColor White
    Write-Host "   $cmdLine" -ForegroundColor Green
    Write-Host ""

    # 确认执行
    $confirm = Read-Host "确认执行? (y/N)"

    if ($confirm -eq 'y' -or $confirm -eq 'Y') {
        Write-Host "⚡ 执行中..." -ForegroundColor Yellow
        Invoke-Expression $cmdLine
    } else {
        Write-Host "❌ 已取消" -ForegroundColor Red
    }
}

# 快捷函数：生成代码
function tgcode {
    param(
        [Parameter(Mandatory=$true, ValueFromRemainingArguments=$true)]
        [string[]]$Description
    )

    $prompt = $Description -join " "
    termgenius code $prompt
}

# 快捷函数：修复上一个失败的命令
function tgfix {
    termgenius fix
}

# 快捷函数：搜索历史
function tgh {
    param(
        [Parameter(ValueFromRemainingArguments=$true)]
        [string[]]$Query
    )

    if ($Query) {
        $searchQuery = $Query -join " "
        termgenius history --search $searchQuery
    } else {
        termgenius history
    }
}

# 快捷函数：查看配置
function tgconf {
    termgenius config --show
}

# 命令补全
Register-ArgumentCompleter -CommandName tg,termgenius -ScriptBlock {
    param($commandName, $wordToComplete, $commandAst, $fakeBoundParameters)

    $subcommands = @(
        [System.Management.Automation.CompletionResult]::new('code', 'code', 'ParameterValue', '生成代码或脚本')
        [System.Management.Automation.CompletionResult]::new('fix', 'fix', 'ParameterValue', '修复上一个失败的命令')
        [System.Management.Automation.CompletionResult]::new('history', 'history', 'ParameterValue', '查看命令历史')
        [System.Management.Automation.CompletionResult]::new('config', 'config', 'ParameterValue', '配置管理')
        [System.Management.Automation.CompletionResult]::new('uninstall', 'uninstall', 'ParameterValue', '卸载指南')
        [System.Management.Automation.CompletionResult]::new('--help', '--help', 'ParameterValue', '显示帮助信息')
        [System.Management.Automation.CompletionResult]::new('--version', '--version', 'ParameterValue', '显示版本信息')
    )

    # 如果是子命令的参数
    $tokens = $commandAst.CommandElements
    if ($tokens.Count -gt 1) {
        $subcommand = $tokens[1].Value

        switch ($subcommand) {
            'history' {
                return @(
                    [System.Management.Automation.CompletionResult]::new('--search', '--search', 'ParameterValue', '搜索历史记录')
                    [System.Management.Automation.CompletionResult]::new('--clear', '--clear', 'ParameterValue', '清空历史记录')
                )
            }
            'config' {
                return @(
                    [System.Management.Automation.CompletionResult]::new('--show', '--show', 'ParameterValue', '显示当前配置')
                    [System.Management.Automation.CompletionResult]::new('--reset', '--reset', 'ParameterValue', '重置为默认配置')
                    [System.Management.Automation.CompletionResult]::new('--model', '--model', 'ParameterValue', '设置模型路径')
                    [System.Management.Automation.CompletionResult]::new('--history-enabled', '--history-enabled', 'ParameterValue', '启用/禁用历史记录')
                )
            }
        }
    }

    # 返回子命令列表
    $subcommands | Where-Object { $_.CompletionText -like "$wordToComplete*" }
}

# 自动捕获命令错误（可选，需要谨慎使用）
# 取消注释以启用
# $global:LastExitCodeBackup = 0
# function prompt {
#     $global:LastExitCodeBackup = $LASTEXITCODE
#     if ($LASTEXITCODE -ne 0 -and $LASTEXITCODE -ne $null) {
#         Write-Host ""
#         Write-Host "💡 提示: 命令执行失败，使用 'tgfix' 获取修复建议" -ForegroundColor Yellow
#     }
#     # 调用原始 prompt
#     & $function:prompt
# }

# 显示欢迎信息
Write-Host "✨ TermGenius Shell Integration 已加载" -ForegroundColor Green
Write-Host "   快捷命令: tgc, tgx, tgcode, tgfix, tgh, tgconf" -ForegroundColor Cyan
Write-Host "   使用 'tg --help' 查看完整帮助" -ForegroundColor Gray
