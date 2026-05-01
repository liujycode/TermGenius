//! 历史记录模块
//!
//! 负责记录和管理命令历史

pub mod manager;
pub mod entry;

pub use manager::HistoryManager;
pub use entry::{HistoryEntry, CommandStatus};
