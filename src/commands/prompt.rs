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
            r#"你是一个专业的终端命令助手。请根据用户的自然语言描述，生成对应的 Shell 命令。

系统信息：
- 操作系统: {}
- Shell: {}

要求：
1. 只输出命令本身，不要有任何解释或额外文字
2. 如果需要多个命令，用 && 或 ; 连接
3. 确保命令在当前系统和 Shell 下可执行
4. 优先使用常见、安全的命令
5. 避免危险操作（如 rm -rf /、format 等）

常用命令参考：
{}

示例：
{}

用户需求：{}

命令："#,
            self.os, self.shell, common_commands, examples, user_input
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
        format!(
            r#"你是一个专业的编程助手。请根据用户的需求，生成对应的代码或脚本。

系统信息：
- 操作系统: {}

要求：
1. 输出完整可运行的代码
2. 包含必要的注释
3. 遵循最佳实践
4. 如果是脚本，添加 shebang 行
5. 代码应该简洁、高效、易读

用户需求：{}

代码：
"#,
            self.os, user_input
        )
    }

    /// 获取命令示例
    fn get_command_examples(&self) -> String {
        match self.os.as_str() {
            "Windows" => {
                r#"用户: "列出当前目录的文件"
命令: dir

用户: "查找包含 test 的文件"
命令: dir /s /b *test*

用户: "删除临时文件"
命令: del /q %TEMP%\*.tmp"#
            }
            "macOS" | "Linux" => {
                r#"用户: "列出当前目录的文件"
命令: ls -la

用户: "查找包含 test 的文件"
命令: find . -name "*test*"

用户: "删除临时文件"
命令: rm -f /tmp/*.tmp"#
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

