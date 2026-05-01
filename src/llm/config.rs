//! LLM 配置管理

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// 模型文件路径
    pub model_path: PathBuf,

    /// 上下文长度
    pub context_size: usize,

    /// 批处理大小
    pub batch_size: usize,

    /// GPU 层数（0 表示仅 CPU）
    pub gpu_layers: i32,

    /// 线程数
    pub threads: usize,

    /// 温度参数（控制随机性）
    pub temperature: f32,

    /// Top-P 采样
    pub top_p: f32,

    /// Top-K 采样
    pub top_k: i32,

    /// 最大生成 token 数
    pub max_tokens: usize,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/qwen-1.8b-q4_0.gguf"),
            context_size: 2048,
            batch_size: 512,
            gpu_layers: 0,
            threads: num_cpus::get(),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            max_tokens: 512,
        }
    }
}

impl LLMConfig {
    /// 创建新的配置
    pub fn new(model_path: PathBuf) -> Self {
        Self {
            model_path,
            ..Default::default()
        }
    }

    /// 设置 GPU 层数
    pub fn with_gpu_layers(mut self, layers: i32) -> Self {
        self.gpu_layers = layers;
        self
    }

    /// 设置线程数
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    /// 设置温度
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    /// 设置最大 token 数
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}
