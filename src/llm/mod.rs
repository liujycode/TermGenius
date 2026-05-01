//! LLM 引擎模块
//!
//! 负责本地 LLM 模型的加载、推理和管理

pub mod engine;
pub mod config;
pub mod error;

#[cfg(test)]
mod tests;

pub use engine::LLMEngine;
pub use config::LLMConfig;
pub use error::{LLMError, Result};
