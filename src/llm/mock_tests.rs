//! Mock LLM 引擎单元测试

#[cfg(test)]
mod tests {
    use super::super::mock_engine::MockLLMEngine;
    use super::super::config::LLMConfig;
    use std::path::PathBuf;

    #[test]
    fn test_mock_engine_creation() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let engine = MockLLMEngine::new(config);

        assert!(engine.is_ok());
    }

    #[test]
    fn test_mock_engine_list_files() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let mut engine = MockLLMEngine::new(config).unwrap();

        let result = engine.generate("列出所有文件");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ls -la");
    }

    #[test]
    fn test_mock_engine_find_large_files() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let mut engine = MockLLMEngine::new(config).unwrap();

        let result = engine.generate("查找大于 100MB 的文件");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "find . -type f -size +100M");
    }

    #[test]
    fn test_mock_engine_delete_logs() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let mut engine = MockLLMEngine::new(config).unwrap();

        let result = engine.generate("删除 7 天前的日志文件");
        assert!(result.is_ok());
        let command = result.unwrap();
        assert!(command.contains("find"));
        assert!(command.contains("log"));
        assert!(command.contains("delete"));
    }

    #[test]
    fn test_mock_engine_python_script() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let mut engine = MockLLMEngine::new(config).unwrap();

        let result = engine.generate("写一个 Python 脚本批量重命名文件");
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("#!/usr/bin/env python3"));
        assert!(code.contains("def rename_files"));
        assert!(code.contains("os.rename"));
    }

    #[test]
    fn test_mock_engine_default_response() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let mut engine = MockLLMEngine::new(config).unwrap();

        let result = engine.generate("这是一个未知的命令");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("模拟响应"));
        assert!(response.contains("这是一个未知的命令"));
    }

    #[test]
    fn test_mock_engine_config() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"))
            .with_temperature(0.5)
            .with_max_tokens(1024);
        let engine = MockLLMEngine::new(config).unwrap();

        let engine_config = engine.config();
        assert_eq!(engine_config.temperature, 0.5);
        assert_eq!(engine_config.max_tokens, 1024);
    }

    #[test]
    fn test_mock_engine_model_info() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let engine = MockLLMEngine::new(config).unwrap();

        let info = engine.model_info();
        assert!(info.contains("Mock LLM Engine"));
        assert!(info.contains("test.gguf"));
    }

    #[test]
    fn test_mock_engine_multiple_generations() {
        let config = LLMConfig::new(PathBuf::from("test.gguf"));
        let mut engine = MockLLMEngine::new(config).unwrap();

        // 第一次生成
        let result1 = engine.generate("列出所有文件");
        assert!(result1.is_ok());

        // 第二次生成
        let result2 = engine.generate("查找大于 100MB 的文件");
        assert!(result2.is_ok());

        // 第三次生成
        let result3 = engine.generate("写一个 Python 脚本");
        assert!(result3.is_ok());
    }
}
