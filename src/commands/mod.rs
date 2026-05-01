//! 命令生成模块
//!
//! 负责将自然语言转换为 Shell 命令

pub mod generator;
pub mod prompt;

pub use generator::CommandGenerator;
pub use prompt::PromptBuilder;
