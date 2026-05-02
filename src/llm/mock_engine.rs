// Mock LLM Engine for testing without llama-cpp dependency
use super::config::LLMConfig;
use super::error::LLMError;

pub struct MockLLMEngine {
    config: LLMConfig,
}

impl MockLLMEngine {
    pub fn new(config: LLMConfig) -> Result<Self, LLMError> {
        println!("⚠️  使用模拟 LLM 引擎（用于测试）");
        println!("   模型路径: {}", config.model_path.display());
        Ok(Self { config })
    }

    pub fn generate(&mut self, prompt: &str) -> Result<String, LLMError> {
        // 模拟命令生成
        if prompt.contains("列出") && prompt.contains("文件") {
            return Ok("ls -la".to_string());
        }

        if prompt.contains("查找") && prompt.contains("大于") {
            return Ok("find . -type f -size +100M".to_string());
        }

        if prompt.contains("删除") && prompt.contains("日志") {
            return Ok("find /var/log -name \"*.log\" -mtime +7 -delete".to_string());
        }

        if prompt.contains("Python") && prompt.contains("脚本") {
            return Ok(r#"#!/usr/bin/env python3
# 批量重命名文件脚本

import os
import sys

def rename_files(directory, pattern, replacement):
    """批量重命名文件"""
    for filename in os.listdir(directory):
        if pattern in filename:
            new_name = filename.replace(pattern, replacement)
            old_path = os.path.join(directory, filename)
            new_path = os.path.join(directory, new_name)
            os.rename(old_path, new_path)
            print(f"重命名: {filename} -> {new_name}")

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("用法: python rename.py <目录> <匹配模式> <替换文本>")
        sys.exit(1)

    directory = sys.argv[1]
    pattern = sys.argv[2]
    replacement = sys.argv[3]

    rename_files(directory, pattern, replacement)
"#.to_string());
        }

        // 默认响应
        Ok(format!("# 模拟响应\n# 输入: {}\n# 这是一个模拟的 LLM 引擎，用于测试\necho \"Hello from mock LLM\"", prompt))
    }

    pub fn config(&self) -> &LLMConfig {
        &self.config
    }

    pub fn model_info(&self) -> String {
        format!("Mock LLM Engine (模拟引擎)\n模型路径: {}", self.config.model_path.display())
    }
}
