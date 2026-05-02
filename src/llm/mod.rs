//! LLM 引擎模块
//!
//! 支持多种 LLM 引擎：
//! - MockLLMEngine: 模拟引擎（用于测试）
//! - HttpLLMEngine: HTTP API 引擎（支持 Ollama 和 llama.cpp server）

pub mod mock_engine;
pub mod http_engine;
pub mod config;
pub mod error;

#[cfg(test)]
mod tests;

// 导出所有引擎
pub use mock_engine::MockLLMEngine;
pub use http_engine::{HttpLLMEngine, ApiType};
pub use config::LLMConfig;
pub use error::{LLMError, Result};

/// LLM 引擎枚举
pub enum LLMEngine {
    Mock(MockLLMEngine),
    Http(HttpLLMEngine),
}

impl LLMEngine {
    /// 生成文本
    pub fn generate(&mut self, prompt: &str) -> Result<String> {
        match self {
            LLMEngine::Mock(engine) => engine.generate(prompt),
            LLMEngine::Http(engine) => engine.generate(prompt),
        }
    }

    /// 获取配置
    pub fn config(&self) -> &LLMConfig {
        match self {
            LLMEngine::Mock(engine) => engine.config(),
            LLMEngine::Http(engine) => engine.config(),
        }
    }

    /// 获取模型信息
    pub fn model_info(&self) -> String {
        match self {
            LLMEngine::Mock(engine) => engine.model_info(),
            LLMEngine::Http(engine) => engine.model_info(),
        }
    }
}
