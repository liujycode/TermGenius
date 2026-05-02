//! 配置模块单元测试

#[cfg(test)]
mod tests {
    use crate::config::schema::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert_eq!(config.model.engine_type, "mock");
        assert_eq!(config.model.api_url, "http://localhost:11434");
        assert_eq!(config.model.context_size, 2048);
        assert_eq!(config.model.temperature, 0.7);
        assert_eq!(config.model.top_p, 0.9);
        assert_eq!(config.model.max_tokens, 512);

        assert!(config.history.enabled);
        assert_eq!(config.history.max_entries, 1000);
        assert!(config.history.save_failed);

        assert!(!config.generation.auto_execute);
        assert!(config.generation.show_explanation);
        assert!(config.generation.show_safety_warning);

        assert!(config.ui.color);
        assert!(config.ui.emoji);
        assert!(config.ui.progress_bar);
    }

    #[test]
    fn test_model_config_defaults() {
        let config = Config::default();

        assert_eq!(config.model.gpu_layers, 0);
        assert_eq!(config.model.threads, 0);
        assert_eq!(config.model.top_k, 40);
    }

    #[test]
    fn test_dangerous_command_detection() {
        let config = Config::default();

        // 危险命令
        assert!(config.is_dangerous_command("rm -rf /"));
        assert!(config.is_dangerous_command("format C:"));
        assert!(config.is_dangerous_command("del /f important.txt"));
        assert!(config.is_dangerous_command("mkfs.ext4 /dev/sda"));
        assert!(config.is_dangerous_command("dd if=/dev/zero of=/dev/sda"));
        assert!(config.is_dangerous_command("echo test > /dev/sda"));

        // 安全命令
        assert!(!config.is_dangerous_command("ls -la"));
        assert!(!config.is_dangerous_command("cat file.txt"));
        assert!(!config.is_dangerous_command("mkdir test"));
        assert!(!config.is_dangerous_command("echo hello"));
    }

    #[test]
    fn test_get_threads() {
        let mut config = Config::default();

        // threads = 0 应该返回 CPU 核心数
        config.model.threads = 0;
        let threads = config.get_threads();
        assert!(threads > 0);

        // threads > 0 应该返回设置的值
        config.model.threads = 4;
        assert_eq!(config.get_threads(), 4);
    }

    #[test]
    fn test_history_config() {
        let config = Config::default();

        assert!(config.history.enabled);
        assert!(config.history.path.is_some());
        assert_eq!(config.history.max_entries, 1000);
        assert!(config.history.save_failed);
    }

    #[test]
    fn test_generation_config() {
        let config = Config::default();

        assert!(!config.generation.auto_execute);
        assert!(config.generation.show_explanation);
        assert!(config.generation.show_safety_warning);
        assert!(!config.generation.dangerous_keywords.is_empty());
    }

    #[test]
    fn test_ui_config() {
        let config = Config::default();

        assert!(config.ui.color);
        assert!(config.ui.emoji);
        assert!(config.ui.progress_bar);
    }

    #[test]
    fn test_dangerous_keywords_not_empty() {
        let config = Config::default();

        assert!(config.generation.dangerous_keywords.len() >= 6);
        assert!(config.generation.dangerous_keywords.contains(&"rm -rf".to_string()));
        assert!(config.generation.dangerous_keywords.contains(&"format".to_string()));
    }

    #[test]
    fn test_model_config_ollama() {
        let mut config = Config::default();
        config.model.engine_type = "ollama".to_string();
        config.model.model_name = "qwen2.5:7b".to_string();

        assert_eq!(config.model.engine_type, "ollama");
        assert_eq!(config.model.model_name, "qwen2.5:7b");
    }

    #[test]
    fn test_model_config_llama_cpp() {
        let mut config = Config::default();
        config.model.engine_type = "llama_cpp".to_string();
        config.model.api_url = "http://localhost:8080".to_string();

        assert_eq!(config.model.engine_type, "llama_cpp");
        assert_eq!(config.model.api_url, "http://localhost:8080");
    }
}
