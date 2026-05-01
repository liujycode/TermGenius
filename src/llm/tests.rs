//! 单元测试

#[cfg(test)]
mod tests {
    use crate::llm::{LLMConfig, LLMEngine};
    use std::path::PathBuf;

    #[test]
    fn test_llm_config_default() {
        let config = LLMConfig::default();
        assert_eq!(config.context_size, 2048);
        assert_eq!(config.batch_size, 512);
        assert_eq!(config.temperature, 0.7);
    }

    #[test]
    fn test_llm_config_builder() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"))
            .with_temperature(0.5)
            .with_max_tokens(1024)
            .with_gpu_layers(10);

        assert_eq!(config.temperature, 0.5);
        assert_eq!(config.max_tokens, 1024);
        assert_eq!(config.gpu_layers, 10);
    }

    #[test]
    #[ignore] // 需要实际模型文件
    fn test_llm_engine_creation() {
        let config = LLMConfig::new(PathBuf::from("models/test.gguf"));
        let result = LLMEngine::new(config);

        // 如果模型文件不存在，应该返回错误
        if result.is_err() {
            println!("预期的错误: 模型文件不存在");
        }
    }
}
