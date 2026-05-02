#!/bin/bash
# TermGenius Shell Integration for Bash/Zsh
#
# 安装方法：
# 1. 将此文件保存到 ~/.termgenius/shell-integration.sh
# 2. 在 ~/.bashrc 或 ~/.zshrc 中添加：
#    source ~/.termgenius/shell-integration.sh

# TermGenius 别名
alias tg='termgenius'

# 快捷函数：生成并显示命令（不执行）
tgc() {
    if [ -z "$*" ]; then
        echo "用法: tgc <自然语言描述>"
        echo "示例: tgc 列出所有文件"
        return 1
    fi

    termgenius "$*"
}

# 快捷函数：生成并执行命令（需要确认）
tgx() {
    if [ -z "$*" ]; then
        echo "用法: tgx <自然语言描述>"
        echo "示例: tgx 列出所有文件"
        return 1
    fi

    echo "🤖 正在生成命令..."

    # 生成命令并提取
    local output=$(termgenius "$*" 2>/dev/null)
    local cmd=$(echo "$output" | grep -A1 "生成的命令" | tail -1 | sed 's/^[[:space:]]*//' | sed 's/\x1b\[[0-9;]*m//g')

    if [ -z "$cmd" ]; then
        echo "❌ 命令生成失败"
        return 1
    fi

    echo ""
    echo "📋 生成的命令:"
    echo "   \033[32m$cmd\033[0m"
    echo ""

    # 确认执行
    read -p "确认执行? (y/N) " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "⚡ 执行中..."
        eval "$cmd"
    else
        echo "❌ 已取消"
    fi
}

# 快捷函数：生成代码
tgcode() {
    if [ -z "$*" ]; then
        echo "用法: tgcode <代码需求描述>"
        echo "示例: tgcode 写一个Python脚本批量重命名文件"
        return 1
    fi

    termgenius code "$*"
}

# 快捷函数：修复上一个失败的命令
tgfix() {
    termgenius fix
}

# 快捷函数：搜索历史
tgh() {
    if [ -z "$*" ]; then
        termgenius history
    else
        termgenius history --search "$*"
    fi
}

# 快捷函数：查看配置
tgconf() {
    termgenius config --show
}

# 自动捕获命令错误（可选，需要谨慎使用）
# 取消注释以启用
# trap 'tg_auto_fix' ERR
# tg_auto_fix() {
#     local exit_code=$?
#     if [ $exit_code -ne 0 ]; then
#         echo ""
#         echo "💡 提示: 命令执行失败，使用 'tgfix' 获取修复建议"
#     fi
# }

# 命令补全（Bash）
if [ -n "$BASH_VERSION" ]; then
    _tg_completion() {
        local cur prev opts
        COMPREPLY=()
        cur="${COMP_WORDS[COMP_CWORD]}"
        prev="${COMP_WORDS[COMP_CWORD-1]}"
        opts="code fix history config uninstall --help --version"

        if [[ ${cur} == -* ]] ; then
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
        fi

        case "${prev}" in
            history)
                COMPREPLY=( $(compgen -W "--search --clear" -- ${cur}) )
                return 0
                ;;
            config)
                COMPREPLY=( $(compgen -W "--show --reset --model --history-enabled" -- ${cur}) )
                return 0
                ;;
        esac
    }
    complete -F _tg_completion tg termgenius
fi

# 命令补全（Zsh）
if [ -n "$ZSH_VERSION" ]; then
    _tg_completion() {
        local -a subcmds
        subcmds=(
            'code:生成代码或脚本'
            'fix:修复上一个失败的命令'
            'history:查看命令历史'
            'config:配置管理'
            'uninstall:卸载指南'
        )

        _arguments \
            '1: :->cmds' \
            '*:: :->args' && return 0

        case $state in
            cmds)
                _describe -t commands 'termgenius commands' subcmds
                ;;
            args)
                case $words[1] in
                    history)
                        _arguments \
                            '--search[搜索历史记录]:query:' \
                            '--clear[清空历史记录]'
                        ;;
                    config)
                        _arguments \
                            '--show[显示当前配置]' \
                            '--reset[重置为默认配置]' \
                            '--model[设置模型路径]:path:_files' \
                            '--history-enabled[启用/禁用历史记录]:bool:(true false)'
                        ;;
                esac
                ;;
        esac
    }
    compdef _tg_completion tg termgenius
fi

# 显示欢迎信息
echo "✨ TermGenius Shell Integration 已加载"
echo "   快捷命令: tgc, tgx, tgcode, tgfix, tgh, tgconf"
echo "   使用 'tg --help' 查看完整帮助"
