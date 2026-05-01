//! 命令生成器

use crate::llm::{LLMEngine, LLMConfig, Result as LLMResult};
use crate::commands::PromptBuilder;
use std::path::PathBuf;
use tracing::{info, error};

/// 命令生成器
pub struct CommandGenerator {
    engine: LLMEngine,
    prompt_builder: PromptBuilder,
}

impl CommandGenerator {
    /// 创建新的命令生成器
    pub fn new(model_path: PathBuf) -> LLMResult<Self> {
        info!("初始化命令生成器...");

        let config = LLMConfig::new(model_path)
            .with_temperature(0.3)  // 降低温度，提高确定性
            .with_max_tokens(256);  // 命令通常较短

        let engine = LLMEngine::new(config)?;
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
        let original_max_tokens = self.engine.config().max_tokens;
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
    use std::path::PathBuf;

    #[test]
    #[ignore] // 需要实际模型文件
    fn test_command_generator() {
        let model_path = PathBuf::from("models/test.gguf");
        let mut generator = CommandGenerator::new(model_path).unwrap();

        let command = generator.generate_command("列出当前目录").unwrap();
        assert!(!command.is_empty());
    }
}
