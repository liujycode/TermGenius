//! Prompt 构建器

use std::env;

/// Prompt 构建器
pub struct PromptBuilder {
    os: String,
    shell: String,
}

impl PromptBuilder {
    /// 创建新的 Prompt 构建器
    pub fn new() -> Self {
        let os = if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "macos") {
            "macOS"
        } else {
            "Linux"
        }.to_string();

        let shell = env::var("SHELL")
            .unwrap_or_else(|_| {
                if cfg!(target_os = "windows") {
                    "PowerShell".to_string()
                } else {
                    "bash".to_string()
                }
            });

        Self { os, shell }
    }

    /// 构建命令生成 prompt
    pub fn build_command_prompt(&self, user_input: &str) -> String {
        let examples = self.get_command_examples();
        let common_commands = self.get_common_commands();

        format!(
            r#"你是一个专业的 {} {} 终端命令助手。

【关键要求】
你的输出必须是一行可执行的命令，不能包含任何解释、说明、注释或额外文字。

错误示例（不要这样输出）：
- "dir /s /b *test*  # 这个命令会查找文件"  ❌ 包含注释
- "可以使用 dir 命令"  ❌ 包含解释
- "dir\n\n注意：这个命令..."  ❌ 包含说明

正确示例（必须这样输出）：
- "dir /s /b *test*"  ✅ 只有命令

系统环境：
- 操作系统: {}
- Shell: {}

命令规范：
1. 只使用 {} 原生命令（Windows 用 dir/forfiles/tasklist 等，Linux 用 ls/find/ps 等）
2. 不要混用不同操作系统的命令
3. 如果需要多个操作，用 && 连接
4. 路径分隔符：{} 使用 {}
5. 避免危险操作

常用命令：
{}

示例：
{}

用户需求：{}

命令："#,
            self.os,
            self.shell,
            self.os,
            self.shell,
            self.os,
            self.os,
            if self.os == "Windows" { "\\" } else { "/" },
            common_commands,
            examples,
            user_input
        )
    }

    /// 构建错误修复 prompt
    pub fn build_fix_prompt(&self, command: &str, error: &str) -> String {
        let error_patterns = self.get_error_patterns();

        format!(
            r#"你是一个专业的终端命令助手。用户执行了一个命令但失败了，请分析错误并提供修复后的命令。

系统信息：
- 操作系统: {}
- Shell: {}

失败的命令：
{}

错误信息：
{}

常见错误模式：
{}

要求：
1. 只输出修复后的命令，不要有任何解释
2. 如果无法修复，输出 "ERROR: 无法自动修复"
3. 确保修复后的命令在当前系统下可执行
4. 优先考虑权限、路径、参数等常见问题

修复后的命令："#,
            self.os, self.shell, command, error, error_patterns
        )
    }

    /// 构建代码生成 prompt
    pub fn build_code_prompt(&self, user_input: &str) -> String {
        let code_examples = self.get_code_examples();

        format!(
            r#"你是一个专业的编程助手。请根据用户需求生成高质量的代码。

系统环境：
- 操作系统: {}

【重要】输出要求：
1. 直接输出完整可运行的代码，不要有额外的解释文字
2. 代码必须包含必要的注释（中文）
3. 如果是脚本，必须添加 shebang 行
4. 代码应该遵循最佳实践
5. 包含错误处理
6. 代码风格清晰、易读

代码规范：
- Python: 使用 PEP 8 风格，添加类型提示
- Shell: 使用 set -e，添加错误检查
- JavaScript: 使用 ES6+ 语法
- 其他语言: 遵循该语言的标准规范

示例：
{}

用户需求：{}

代码（直接输出代码，不要有其他内容）：
"#,
            self.os,
            code_examples,
            user_input
        )
    }

    /// 获取代码示例
    fn get_code_examples(&self) -> String {
        r#"用户: "写一个 Python 脚本读取 CSV 文件"
代码:
#!/usr/bin/env python3
import csv

def read_csv(filename):
    """读取 CSV 文件并返回数据"""
    with open(filename, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        return list(reader)

if __name__ == "__main__":
    data = read_csv('data.csv')
    print(data)

用户: "写一个 Shell 脚本备份目录"
代码:
#!/bin/bash
set -e

backup_dir() {
    local src="$1"
    local dst="$2"
    tar -czf "${dst}/backup_$(date +%Y%m%d_%H%M%S).tar.gz" "$src"
    echo "备份完成"
}

backup_dir "/path/to/source" "/path/to/backup""#.to_string()
    }

    /// 获取命令示例
    fn get_command_examples(&self) -> String {
        match self.os.as_str() {
            "Windows" => {
                r#"用户: "列出当前目录的文件"
命令: dir

用户: "查找大于 1MB 的文件"
命令: forfiles /S /C "cmd /c if @fsize gtr 1048576 echo @path"

用户: "查找包含 test 的文件"
命令: dir /s /b *test*

用户: "删除 7 天前的日志文件"
命令: forfiles /p "C:\logs" /s /m *.log /d -7 /c "cmd /c del @path"

用户: "显示当前目录的磁盘使用情况"
命令: dir

用户: "查看进程列表"
命令: tasklist"#
            }
            "macOS" | "Linux" => {
                r#"用户: "列出当前目录的文件"
命令: ls -la

用户: "查找大于 1MB 的文件"
命令: find . -type f -size +1M

用户: "查找包含 test 的文件"
命令: find . -name "*test*"

用户: "删除 7 天前的日志文件"
命令: find /var/log -name "*.log" -mtime +7 -delete

用户: "显示当前目录的磁盘使用情况"
命令: du -sh *

用户: "查看进程列表"
命令: ps aux"#
            }
            _ => ""
        }.to_string()
    }

    /// 获取常用命令
    fn get_common_commands(&self) -> String {
        match self.os.as_str() {
            "Windows" => {
                r#"- 文件操作: dir, cd, copy, move, del, mkdir, rmdir
- 系统信息: systeminfo, tasklist, netstat
- 网络: ping, ipconfig, tracert
- 进程: taskkill, start
- 文本: type, find, findstr"#
            }
            "macOS" | "Linux" => {
                r#"- 文件操作: ls, cd, cp, mv, rm, mkdir, rmdir, touch
- 系统信息: uname, top, ps, df, du
- 网络: ping, ifconfig, netstat, curl, wget
- 进程: kill, killall, pkill
- 文本: cat, grep, sed, awk, head, tail
- 权限: chmod, chown, sudo"#
            }
            _ => ""
        }.to_string()
    }

    /// 获取错误模式
    fn get_error_patterns(&self) -> String {
        match self.os.as_str() {
            "Windows" => {
                r#"- "不是内部或外部命令" → 检查命令拼写或添加路径
- "拒绝访问" → 使用管理员权限运行
- "找不到文件" → 检查路径是否正确
- "语法错误" → 检查命令参数"#
            }
            "macOS" | "Linux" => {
                r#"- "command not found" → 检查命令拼写或安装软件包
- "Permission denied" → 使用 sudo 或修改权限
- "No such file or directory" → 检查路径是否正确
- "syntax error" → 检查命令参数和引号"#
            }
            _ => ""
        }.to_string()
    }

    /// 获取操作系统
    pub fn os(&self) -> &str {
        &self.os
    }

    /// 获取 Shell 类型
    pub fn shell(&self) -> &str {
        &self.shell
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder() {
        let builder = PromptBuilder::new();
        let prompt = builder.build_command_prompt("列出当前目录的文件");
        assert!(prompt.contains("列出当前目录的文件"));
        assert!(prompt.contains("操作系统"));
        assert!(prompt.contains("常用命令参考"));
    }

    #[test]
    fn test_fix_prompt() {
        let builder = PromptBuilder::new();
        let prompt = builder.build_fix_prompt("ls -la", "command not found");
        assert!(prompt.contains("ls -la"));
        assert!(prompt.contains("command not found"));
        assert!(prompt.contains("常见错误模式"));
    }
}

