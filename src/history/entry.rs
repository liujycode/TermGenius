//! 历史记录条目定义

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 命令状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandStatus {
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 未执行
    NotExecuted,
}

/// 历史记录条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// 唯一 ID
    pub id: u64,

    /// 时间戳（Unix 时间戳）
    pub timestamp: u64,

    /// 用户输入（自然语言）
    pub user_input: String,

    /// 生成的命令
    pub generated_command: String,

    /// 命令状态
    pub status: CommandStatus,

    /// 错误信息（如果失败）
    pub error_message: Option<String>,

    /// 执行时间（毫秒）
    pub execution_time: Option<u64>,

    /// 操作系统
    pub os: String,

    /// Shell 类型
    pub shell: String,
}

impl HistoryEntry {
    /// 创建新的历史记录
    pub fn new(
        id: u64,
        user_input: String,
        generated_command: String,
        os: String,
        shell: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            timestamp,
            user_input,
            generated_command,
            status: CommandStatus::NotExecuted,
            error_message: None,
            execution_time: None,
            os,
            shell,
        }
    }

    /// 标记为成功
    pub fn mark_success(&mut self, execution_time: u64) {
        self.status = CommandStatus::Success;
        self.execution_time = Some(execution_time);
    }

    /// 标记为失败
    pub fn mark_failed(&mut self, error_message: String, execution_time: u64) {
        self.status = CommandStatus::Failed;
        self.error_message = Some(error_message);
        self.execution_time = Some(execution_time);
    }

    /// 获取格式化的时间
    pub fn formatted_time(&self) -> String {
        use std::time::{Duration, UNIX_EPOCH};

        let datetime = UNIX_EPOCH + Duration::from_secs(self.timestamp);

        // 简单格式化（实际项目中应使用 chrono）
        format!("{:?}", datetime)
    }

    /// 获取状态图标
    pub fn status_icon(&self) -> &str {
        match self.status {
            CommandStatus::Success => "✅",
            CommandStatus::Failed => "❌",
            CommandStatus::NotExecuted => "⏸️",
        }
    }

    /// 获取状态文本
    pub fn status_text(&self) -> &str {
        match self.status {
            CommandStatus::Success => "成功",
            CommandStatus::Failed => "失败",
            CommandStatus::NotExecuted => "未执行",
        }
    }

    /// 显示为字符串
    pub fn display(&self) -> String {
        let mut result = format!(
            "[{}] {} {}\n  输入: {}\n  命令: {}",
            self.id,
            self.status_icon(),
            self.formatted_time(),
            self.user_input,
            self.generated_command
        );

        if let Some(error) = &self.error_message {
            result.push_str(&format!("\n  错误: ", error));
        }

        if let Some(time) = self.execution_time {
            result.push_str(&format!("\n  耗时: {} ms", time));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry() {
        let mut entry = HistoryEntry::new(
            1,
            "列出文件".to_string(),
            "ls -la".to_string(),
            "Linux".to_string(),
            "bash".to_string(),
        );

        assert_eq!(entry.status, CommandStatus::NotExecuted);

        entry.mark_success(100);
        assert_eq!(entry.status, CommandStatus::Success);
        assert_eq!(entry.execution_time, Some(100));

        entry.mark_failed("命令不存在".to_string(), 50);
        assert_eq!(entry.status, CommandStatus::Failed);
        assert_eq!(entry.error_message, Some("命令不存在".to_string()));
    }
}
