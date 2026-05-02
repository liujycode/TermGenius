//! HTTP API LLM 引擎
//! 支持 Ollama 和 llama.cpp server

use super::config::LLMConfig;
use super::error::LLMError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};

/// HTTP API 类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiType {
    /// Ollama API
    Ollama,
    /// llama.cpp server API
    LlamaCpp,
}

/// Ollama API 请求
#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: i32,
}

/// Ollama API 响应
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

/// llama.cpp server API 请求
#[derive(Debug, Serialize)]
struct LlamaCppRequest {
    prompt: String,
    temperature: f32,
    n_predict: i32,
    stop: Vec<String>,
}

/// llama.cpp server API 响应
#[derive(Debug, Deserialize)]
struct LlamaCppResponse {
    content: String,
}

/// HTTP API LLM 引擎
pub struct HttpLLMEngine {
    config: LLMConfig,
    client: Client,
    api_type: ApiType,
    api_url: String,
    model_name: String,
}

impl HttpLLMEngine {
    /// 创建新的 HTTP API 引擎
    ///
    /// # 参数
    /// - `config`: LLM 配置
    /// - `api_url`: API 地址（如 "http://localhost:11434" for Ollama）
    /// - `model_name`: 模型名称（如 "qwen2.5:1.5b" for Ollama）
    /// - `api_type`: API 类型
    pub fn new(
        config: LLMConfig,
        api_url: String,
        model_name: String,
        api_type: ApiType,
    ) -> Result<Self, LLMError> {
        info!("初始化 HTTP API LLM 引擎");
        info!("  API 类型: {:?}", api_type);
        info!("  API 地址: {}", api_url);
        info!("  模型名称: {}", model_name);

        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| LLMError::GenerationError(format!("创建 HTTP 客户端失败: {}", e)))?;

        Ok(Self {
            config,
            client,
            api_type,
            api_url,
            model_name,
        })
    }

    /// 生成文本
    pub fn generate(&mut self, prompt: &str) -> Result<String, LLMError> {
        debug!("生成文本，prompt 长度: {}", prompt.len());

        match self.api_type {
            ApiType::Ollama => self.generate_ollama(prompt),
            ApiType::LlamaCpp => self.generate_llama_cpp(prompt),
        }
    }

    /// 使用 Ollama API 生成
    fn generate_ollama(&self, prompt: &str) -> Result<String, LLMError> {
        let url = format!("{}/api/generate", self.api_url);

        let request = OllamaRequest {
            model: self.model_name.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: OllamaOptions {
                temperature: self.config.temperature,
                num_predict: self.config.max_tokens as i32,
            },
        };

        debug!("发送 Ollama API 请求: {}", url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .map_err(|e| LLMError::GenerationError(format!("API 请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(LLMError::GenerationError(format!(
                "API 返回错误状态: {}",
                response.status()
            )));
        }

        let ollama_response: OllamaResponse = response
            .json()
            .map_err(|e| LLMError::GenerationError(format!("解析响应失败: {}", e)))?;

        Ok(ollama_response.response)
    }

    /// 使用 llama.cpp server API 生成
    fn generate_llama_cpp(&self, prompt: &str) -> Result<String, LLMError> {
        let url = format!("{}/completion", self.api_url);

        let request = LlamaCppRequest {
            prompt: prompt.to_string(),
            temperature: self.config.temperature,
            n_predict: self.config.max_tokens as i32,
            stop: vec!["\n\n".to_string()],
        };

        debug!("发送 llama.cpp API 请求: {}", url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .map_err(|e| LLMError::GenerationError(format!("API 请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(LLMError::GenerationError(format!(
                "API 返回错误状态: {}",
                response.status()
            )));
        }

        let llama_response: LlamaCppResponse = response
            .json()
            .map_err(|e| LLMError::GenerationError(format!("解析响应失败: {}", e)))?;

        Ok(llama_response.content)
    }

    /// 获取配置
    pub fn config(&self) -> &LLMConfig {
        &self.config
    }

    /// 获取模型信息
    pub fn model_info(&self) -> String {
        format!(
            "HTTP API LLM Engine\nAPI 类型: {:?}\nAPI 地址: {}\n模型名称: {}",
            self.api_type, self.api_url, self.model_name
        )
    }

    /// 测试连接
    pub fn test_connection(&self) -> Result<(), LLMError> {
        info!("测试 API 连接...");

        match self.api_type {
            ApiType::Ollama => {
                let url = format!("{}/api/tags", self.api_url);
                let response = self.client
                    .get(&url)
                    .send()
                    .map_err(|e| LLMError::GenerationError(format!("连接失败: {}", e)))?;

                if !response.status().is_success() {
                    return Err(LLMError::GenerationError(format!(
                        "API 返回错误状态: {}",
                        response.status()
                    )));
                }

                info!("✅ Ollama API 连接成功");
                Ok(())
            }
            ApiType::LlamaCpp => {
                let url = format!("{}/health", self.api_url);
                let response = self.client
                    .get(&url)
                    .send()
                    .map_err(|e| LLMError::GenerationError(format!("连接失败: {}", e)))?;

                if !response.status().is_success() {
                    return Err(LLMError::GenerationError(format!(
                        "API 返回错误状态: {}",
                        response.status()
                    )));
                }

                info!("✅ llama.cpp server 连接成功");
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_engine_creation() {
        let config = LLMConfig::default();
        let engine = HttpLLMEngine::new(
            config,
            "http://localhost:11434".to_string(),
            "qwen2.5:1.5b".to_string(),
            ApiType::Ollama,
        );
        assert!(engine.is_ok());
    }
}
