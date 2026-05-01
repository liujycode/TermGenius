//! 错误捕获模块
//!
//! 负责捕获命令执行错误并提供修复建议

pub mod executor;
pub mod analyzer;

pub use executor::CommandExecutor;
pub use analyzer::ErrorAnalyzer;
