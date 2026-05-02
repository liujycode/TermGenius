//! LLM 引擎模块
//!
//! 负责本地 LLM 模型的加载、推理和管理

// 使用模拟引擎进行测试
pub mod mock_engine;
pub mod config;
pub mod error;

#[cfg(test)]
mod tests;

// 导出模拟引擎作为 LLMEngine
pub use mock_engine::MockLLMEngine as LLMEngine;
pub use config::LLMConfig;
pub use error::{LLMError, Result};
