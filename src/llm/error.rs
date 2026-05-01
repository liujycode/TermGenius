//! LLM 引擎错误类型定义

use thiserror::Error;

pub type Result<T> = std::result::Result<T, LLMError>;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("模型文件未找到: {0}")]
    ModelNotFound(String),

    #[error("模型加载失败: {0}")]
    ModelLoadError(String),

    #[error("推理失败: {0}")]
    InferenceError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("其他错误: {0}")]
    Other(String),
}
