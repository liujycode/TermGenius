//! LLM 引擎核心实现

use crate::llm::{LLMConfig, LLMError, Result};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::token::data_array::LlamaTokenDataArray;
use std::path::Path;
use tracing::{info, warn, debug};

/// LLM 引擎
pub struct LLMEngine {
    config: LLMConfig,
    backend: LlamaBackend,
    model: LlamaModel,
    context: LlamaContext,
}

impl LLMEngine {
    /// 创建新的 LLM 引擎
    pub fn new(config: LLMConfig) -> Result<Self> {
        info!("初始化 LLM 引擎...");

        // 检查模型文件是否存在
        if !config.model_path.exists() {
            return Err(LLMError::ModelNotFound(
                config.model_path.display().to_string()
            ));
        }

        // 初始化后端
        info!("初始化 llama.cpp 后端...");
        let backend = LlamaBackend::init()
            .map_err(|e| LLMError::ModelLoadError(e.to_string()))?;

        // 加载模型
        info!("加载模型: {}", config.model_path.display());
        let model_params = LlamaModelParams::default()
            .with_n_gpu_layers(config.gpu_layers as u32);

        let model = LlamaModel::load_from_file(&backend, &config.model_path, &model_params)
            .map_err(|e| LLMError::ModelLoadError(e.to_string()))?;

        // 创建上下文
        info!("创建推理上下文...");
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(Some(config.context_size as u32))
            .with_n_batch(config.batch_size as u32)
            .with_n_threads(config.threads as u32)
            .with_n_threads_batch(config.threads as u32);

        let context = model.new_context(&backend, ctx_params)
            .map_err(|e| LLMError::ModelLoadError(e.to_string()))?;

        info!("LLM 引擎初始化完成");

        Ok(Self {
            config,
            backend,
            model,
            context,
        })
    }

    /// 生成文本
    pub fn generate(&mut self, prompt: &str) -> Result<String> {
        debug!("开始生成，提示词: {}", prompt);

        // Tokenize 输入
        let tokens = self.model
            .str_to_token(prompt, llama_cpp_2::model::AddBos::Always)
            .map_err(|e| LLMError::InferenceError(format!("Tokenization 失败: {}", e)))?;

        debug!("Token 数量: {}", tokens.len());

        // 创建批处理
        let mut batch = LlamaBatch::new(self.config.batch_size, 1);

        // 添加 tokens 到批处理
        for (i, &token) in tokens.iter().enumerate() {
            batch.add(token, i as i32, &[0], false)
                .map_err(|e| LLMError::InferenceError(format!("添加 token 失败: {}", e)))?;
        }

        // 解码输入
        self.context.decode(&mut batch)
            .map_err(|e| LLMError::InferenceError(format!("解码失败: {}", e)))?;

        // 生成输出
        let mut output_tokens = Vec::new();
        let mut n_cur = tokens.len();
        let n_len = self.config.max_tokens;

        while n_cur <= n_len {
            // 获取候选 tokens
            let candidates = self.context.candidates_ith(batch.n_tokens() - 1);

            // 应用采样参数
            let mut candidates_p = LlamaTokenDataArray::from_iter(candidates, false);

            // Temperature 采样
            candidates_p.sample_temp(&mut self.context, self.config.temperature);

            // Top-K 采样
            candidates_p.sample_top_k(&mut self.context, self.config.top_k, 1);

            // Top-P 采样
            candidates_p.sample_top_p(&mut self.context, self.config.top_p, 1);

            // 选择 token
            let new_token_id = candidates_p.sample_token(&mut self.context);

            // 检查是否结束
            if self.model.is_eog_token(new_token_id) {
                break;
            }

            output_tokens.push(new_token_id);

            // 准备下一轮
            batch.clear();
            batch.add(new_token_id, n_cur as i32, &[0], true)
                .map_err(|e| LLMError::InferenceError(format!("添加 token 失败: {}", e)))?;

            self.context.decode(&mut batch)
                .map_err(|e| LLMError::InferenceError(format!("解码失败: {}", e)))?;

            n_cur += 1;
        }

        // 将 tokens 转换为文本
        let output = self.model.token_to_str(output_tokens[0])
            .map_err(|e| LLMError::InferenceError(format!("Token 转文本失败: {}", e)))?;

        debug!("生成完成，输出长度: {}", output.len());

        Ok(output)
    }

    /// 获取配置
    pub fn config(&self) -> &LLMConfig {
        &self.config
    }

    /// 获取模型信息
    pub fn model_info(&self) -> String {
        format!(
            "模型: {}\n上下文大小: {}\nGPU 层数: {}\n线程数: {}",
            self.config.model_path.display(),
            self.config.context_size,
            self.config.gpu_layers,
            self.config.threads
        )
    }
}
