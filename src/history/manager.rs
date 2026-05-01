//! 历史记录管理器

use crate::history::{HistoryEntry, CommandStatus};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// 历史记录管理器
pub struct HistoryManager {
    history_path: PathBuf,
    entries: Vec<HistoryEntry>,
    max_entries: usize,
    next_id: u64,
}

impl HistoryManager {
    /// 加载历史记录
    pub fn load(history_path: PathBuf, max_entries: usize) -> Result<Self> {
        let entries = if history_path.exists() {
            info!("加载历史记录: {}", history_path.display());
            Self::load_from_file(&history_path)?
        } else {
            info!("历史记录文件不存在，创建新的");
            Vec::new()
        };

        let next_id = entries.iter()
            .map(|e| e.id)
            .max()
            .unwrap_or(0) + 1;

        Ok(Self {
            history_path,
            entries,
            max_entries,
            next_id,
        })
    }

    /// 从文件加载历史记录
    fn load_from_file(path: &Path) -> Result<Vec<HistoryEntry>> {
        let content = fs::read_to_string(path)
            .context("读取历史记录文件失败")?;

        let entries: Vec<HistoryEntry> = serde_json::from_str(&content)
            .context("解析历史记录失败")?;

        Ok(entries)
    }

    /// 保存历史记录到文件
    fn save_to_file(&self) -> Result<()> {
        // 确保目录存在
        if let Some(parent) = self.history_path.parent() {
            fs::create_dir_all(parent)
                .context("创建历史记录目录失败")?;
        }

        let content = serde_json::to_string_pretty(&self.entries)
            .context("序列化历史记录失败")?;

        fs::write(&self.history_path, content)
            .context("写入历史记录文件失败")?;

        Ok(())
    }

    /// 添加新记录
    pub fn add_entry(
        &mut self,
        user_input: String,
        generated_command: String,
        os: String,
        shell: String,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let entry = HistoryEntry::new(id, user_input, generated_command, os, shell);
        self.entries.push(entry);

        // 限制历史记录数量
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }

        // 保存到文件
        if let Err(e) = self.save_to_file() {
            warn!("保存历史记录失败: {}", e);
        }

        id
    }

    /// 更新记录状态（成功）
    pub fn mark_success(&mut self, id: u64, execution_time: u64) -> Result<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.mark_success(execution_time);
            self.save_to_file()?;
        }
        Ok(())
    }

    /// 更新记录状态（失败）
    pub fn mark_failed(&mut self, id: u64, error_message: String, execution_time: u64) -> Result<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.mark_failed(error_message, execution_time);
            self.save_to_file()?;
        }
        Ok(())
    }

    /// 获取最近的记录
    pub fn get_recent(&self, count: usize) -> Vec<&HistoryEntry> {
        self.entries.iter()
            .rev()
            .take(count)
            .collect()
    }

    /// 获取最后一条失败的记录
    pub fn get_last_failed(&self) -> Option<&HistoryEntry> {
        self.entries.iter()
            .rev()
            .find(|e| e.status == CommandStatus::Failed)
    }

    /// 搜索历史记录
    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        self.entries.iter()
            .filter(|e| {
                e.user_input.contains(query) || e.generated_command.contains(query)
            })
            .collect()
    }

    /// 清空历史记录
    pub fn clear(&mut self) -> Result<()> {
        info!("清空历史记录");
        self.entries.clear();
        self.next_id = 1;
        self.save_to_file()
    }

    /// 获取统计信息
    pub fn stats(&self) -> HistoryStats {
        let total = self.entries.len();
        let success = self.entries.iter()
            .filter(|e| e.status == CommandStatus::Success)
            .count();
        let failed = self.entries.iter()
            .filter(|e| e.status == CommandStatus::Failed)
            .count();
        let not_executed = self.entries.iter()
            .filter(|e| e.status == CommandStatus::NotExecuted)
            .count();

        HistoryStats {
            total,
            success,
            failed,
            not_executed,
        }
    }

    /// 显示历史记录
    pub fn display(&self, count: usize) -> String {
        let recent = self.get_recent(count);

        if recent.is_empty() {
            return "暂无历史记录".to_string();
        }

        let mut result = format!("最近 {} 条历史记录:\n\n", recent.len());

        for entry in recent {
            result.push_str(&entry.display());
            result.push_str("\n\n");
        }

        let stats = self.stats();
        result.push_str(&format!(
            "统计: 总计 {} 条，成功 {} 条，失败 {} 条，未执行 {} 条",
            stats.total, stats.success, stats.failed, stats.not_executed
        ));

        result
    }
}

/// 历史记录统计
#[derive(Debug)]
pub struct HistoryStats {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub not_executed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_history_manager() {
        let temp_path = env::temp_dir().join("termgenius_test_history.json");

        let mut manager = HistoryManager::load(temp_path.clone(), 100).unwrap();

        let id = manager.add_entry(
            "列出文件".to_string(),
            "ls -la".to_string(),
            "Linux".to_string(),
            "bash".to_string(),
        );

        assert_eq!(manager.entries.len(), 1);

        manager.mark_success(id, 100).unwrap();

        let entry = manager.entries.iter().find(|e| e.id == id).unwrap();
        assert_eq!(entry.status, CommandStatus::Success);

        // 清理
        let _ = fs::remove_file(temp_path);
    }
}
