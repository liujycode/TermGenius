//! 命令执行器

use anyhow::{Context, Result};
use std::process::{Command, Output};
use std::time::{Duration, Instant};
use tracing::{debug, info};

/// 命令执行结果
#[derive(Debug)]
pub struct ExecutionResult {
    /// 是否成功
    pub success: bool,
    /// 标准输出
    pub stdout: String,
    /// 标准错误
    pub stderr: String,
    /// 退出码
    pub exit_code: Option<i32>,
    /// 执行时间（毫秒）
    pub execution_time: u64,
}

/// 命令执行器
pub struct CommandExecutor {
    shell: String,
}

impl CommandExecutor {
    /// 创建新的命令执行器
    pub fn new() -> Self {
        let shell = if cfg!(target_os = "windows") {
            "powershell".to_string()
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string())
        };

        Self { shell }
    }

    /// 执行命令
    pub fn execute(&self, command: &str) -> Result<ExecutionResult> {
        info!("执行命令: {}", command);
        let start = Instant::now();

        let output = if cfg!(target_os = "windows") {
            self.execute_windows(command)?
        } else {
            self.execute_unix(command)?
        };

        let execution_time = start.elapsed().as_millis() as u64;

        let result = ExecutionResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            execution_time,
        };

        debug!("执行结果: success={}, exit_code={:?}, time={}ms",
               result.success, result.exit_code, result.execution_time);

        Ok(result)
    }

    /// 在 Windows 上执行命令
    fn execute_windows(&self, command: &str) -> Result<Output> {
        Command::new("powershell")
            .args(&["-NoProfile", "-Command", command])
            .output()
            .context("执行命令失败")
    }

    /// 在 Unix 系统上执行命令
    fn execute_unix(&self, command: &str) -> Result<Output> {
        Command::new(&self.shell)
            .args(&["-c", command])
            .output()
            .context("执行命令失败")
    }

    /// 执行命令并捕获错误
    pub fn execute_with_error_capture(&self, command: &str) -> Result<ExecutionResult> {
        let result = self.execute(command)?;

        if !result.success {
            info!("命令执行失败: exit_code={:?}", result.exit_code);
            debug!("stderr: {}", result.stderr);
        }

        Ok(result)
    }

    /// 测试命令是否可执行（不实际执行）
    pub fn test_command(&self, command: &str) -> bool {
        // 提取命令名称（第一个单词）
        let cmd_name = command.split_whitespace().next().unwrap_or("");

        if cmd_name.is_empty() {
            return false;
        }

        // 检查命令是否存在
        if cfg!(target_os = "windows") {
            Command::new("where")
                .arg(cmd_name)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        } else {
            Command::new("which")
                .arg(cmd_name)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
    }
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionResult {
    /// 获取输出（优先 stdout，如果为空则返回 stderr）
    pub fn get_output(&self) -> &str {
        if !self.stdout.is_empty() {
            &self.stdout
        } else {
            &self.stderr
        }
    }

    /// 是否有错误输出
    pub fn has_error(&self) -> bool {
        !self.stderr.is_empty()
    }

    /// 显示结果
    pub fn display(&self) -> String {
        let mut result = String::new();

        if self.success {
            result.push_str("✅ 命令执行成功\n\n");
        } else {
            result.push_str("❌ 命令执行失败\n\n");
        }

        if !self.stdout.is_empty() {
            result.push_str("输出:\n");
            result.push_str(&self.stdout);
            result.push('\n');
        }

        if !self.stderr.is_empty() {
            result.push_str("错误:\n");
            result.push_str(&self.stderr);
            result.push('\n');
        }

        result.push_str(&format!("\n耗时:  ms", self.execution_time));

        if let Some(code) = self.exit_code {
            result.push_str(&format!("\n退出码: {}", code));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_success() {
        let executor = CommandExecutor::new();
        let result = executor.execute("echo hello").unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("hello"));
    }

    #[test]
    fn test_execute_failure() {
        let executor = CommandExecutor::new();
        let result = executor.execute("nonexistent_command_xyz").unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_command_exists() {
        let executor = CommandExecutor::new();
        assert!(executor.test_command("echo"));
        assert!(!executor.test_command("nonexistent_command_xyz"));
    }
}
