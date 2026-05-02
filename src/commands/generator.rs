//! 命令生成器

use crate::llm::{LLMEngine, MockLLMEngine, HttpLLMEngine, ApiType, LLMConfig, Result as LLMResult};
use crate::commands::PromptBuilder;
use crate::config::Config;
use tracing::{info, error};

/// 命令生成器
pub struct CommandGenerator {
    engine: LLMEngine,
    prompt_builder: PromptBuilder,
}

impl CommandGenerator {
    /// 从配置创建命令生成器
    pub fn from_config(config: &Config) -> LLMResult<Self> {
        info!("初始化命令生成器...");
        info!("  引擎类型: {}", config.model.engine_type);

        let llm_config = LLMConfig::new(config.model.path.clone())
            .with_temperature(config.model.temperature)
            .with_max_tokens(config.model.max_tokens);

        let engine = match config.model.engine_type.as_str() {
            "ollama" => {
                info!("  使用 Ollama API");
                let http_engine = HttpLLMEngine::new(
                    llm_config,
                    config.model.api_url.clone(),
                    config.model.model_name.clone(),
                    ApiType::Ollama,
                )?;
                LLMEngine::Http(http_engine)
            }
            "llama_cpp" => {
                info!("  使用 llama.cpp server API");
                let http_engine = HttpLLMEngine::new(
                    llm_config,
                    config.model.api_url.clone(),
                    config.model.model_name.clone(),
                    ApiType::LlamaCpp,
                )?;
                LLMEngine::Http(http_engine)
            }
            "mock" | _ => {
                info!("  使用模拟引擎");
                let mock_engine = MockLLMEngine::new(llm_config)?;
                LLMEngine::Mock(mock_engine)
            }
        };

        let prompt_builder = PromptBuilder::new();

        Ok(Self {
            engine,
            prompt_builder,
        })
    }

    /// 从自然语言生成命令
    pub fn generate_command(&mut self, user_input: &str) -> LLMResult<String> {
        info!("生成命令: {}", user_input);

        let prompt = self.prompt_builder.build_command_prompt(user_input);
        let output = self.engine.generate(&prompt)?;

        // 清理输出（去除多余空白和换行）
        let command = output.trim().to_string();

        info!("生成的命令: {}", command);
        Ok(command)
    }

    /// 修复失败的命令
    pub fn fix_command(&mut self, command: &str, error: &str) -> LLMResult<String> {
        info!("修复命令: {}", command);

        let prompt = self.prompt_builder.build_fix_prompt(command, error);
        let output = self.engine.generate(&prompt)?;

        let fixed_command = output.trim().to_string();

        if fixed_command.starts_with("ERROR:") {
            error!("无法自动修复命令");
            return Err(crate::llm::LLMError::InferenceError(
                "无法自动修复该命令".to_string()
            ));
        }

        info!("修复后的命令: {}", fixed_command);
        Ok(fixed_command)
    }

    /// 生成代码
    pub fn generate_code(&mut self, user_input: &str) -> LLMResult<String> {
        info!("生成代码: {}", user_input);

        let prompt = self.prompt_builder.build_code_prompt(user_input);

        // 代码生成需要更多 tokens
        let _original_max_tokens = self.engine.config().max_tokens;
        // TODO: 支持动态修改 max_tokens

        let output = self.engine.generate(&prompt)?;

        info!("代码生成完成，长度: {}", output.len());
        Ok(output)
    }

    /// 获取模型信息
    pub fn model_info(&self) -> String {
        self.engine.model_info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_command_generator_creation() {
        let config = Config::default();
        let generator = CommandGenerator::from_config(&config);

        // Mock 引擎应该总是成功创建
        assert!(generator.is_ok());
    }

    #[test]
    fn test_command_generation_with_mock() {
        let config = Config::default();
        let mut generator = CommandGenerator::from_config(&config).unwrap();

        let command = generator.generate_command("列出当前目录的所有文件").unwrap();
        assert!(!command.is_empty());
        // Mock 引擎应该返回 "ls -la"
        assert_eq!(command, "ls -la");
    }

    #[test]
    fn test_code_generation_with_mock() {
        let config = Config::default();
        let mut generator = CommandGenerator::from_config(&config).unwrap();

        let code = generator.generate_code("写一个 Python 脚本批量重命名文件").unwrap();
        assert!(!code.is_empty());
        assert!(code.contains("python"));
    }
}
