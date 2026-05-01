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
        format!(
            r#"你是一个专业的终端命令助手。请根据用户的自然语言描述，生成对应的 Shell 命令。

系统信息：
- 操作系统: {}
- Shell: {}

要求：
1. 只输出命令本身，不要有任何解释
2. 如果需要多个命令，用 && 或 ; 连接
3. 确保命令在当前系统和 Shell 下可执行
4. 优先使用常见、安全的命令

用户需求：{}

命令："#,
            self.os, self.shell, user_input
        )
    }

    /// 构建错误修复 prompt
    pub fn build_fix_prompt(&self, command: &str, error: &str) -> String {
        format!(
            r#"你是一个专业的终端命令助手。用户执行了一个命令但失败了，请分析错误并提供修复后的命令。

系统信息：
- 操作系统: {}
- Shell: {}

失败的命令：
{}

错误信息：
{}

要求：
1. 只输出修复后的命令，不要有任何解释
2. 如果无法修复，输出 "ERROR: 无法自动修复"
3. 确保修复后的命令在当前系统下可执行

修复后的命令："#,
            self.os, self.shell, command, error
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

用户需求：{}

代码：
"#,
            self.os, user_input
        )
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
    }
}
