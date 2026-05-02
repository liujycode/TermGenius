//! 配置文件结构定义

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// TermGenius 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 模型配置
    pub model: ModelConfig,

    /// 历史记录配置
    pub history: HistoryConfig,

    /// 命令生成配置
    pub generation: GenerationConfig,

    /// UI 配置
    pub ui: UiConfig,
}

/// 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// 引擎类型：mock, ollama, llama_cpp
    #[serde(default = "default_engine_type")]
    pub engine_type: String,

    /// API 地址（用于 HTTP API 引擎）
    #[serde(default = "default_api_url")]
    pub api_url: String,

    /// 模型名称（用于 HTTP API 引擎）
    #[serde(default = "default_model_name")]
    pub model_name: String,

    /// 模型文件路径（用于本地引擎）
    pub path: PathBuf,

    /// 上下文大小
    #[serde(default = "default_context_size")]
    pub context_size: usize,

    /// GPU 层数（0 表示仅 CPU）
    #[serde(default)]
    pub gpu_layers: i32,

    /// 线程数（0 表示自动检测）
    #[serde(default)]
    pub threads: usize,

    /// 温度参数
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Top-P 采样
    #[serde(default = "default_top_p")]
    pub top_p: f32,

    /// Top-K 采样
    #[serde(default = "default_top_k")]
    pub top_k: i32,

    /// 最大生成 token 数
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
}

/// 历史记录配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    /// 是否启用历史记录
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// 历史记录文件路径
    pub path: Option<PathBuf>,

    /// 最大历史记录数
    #[serde(default = "default_max_history")]
    pub max_entries: usize,

    /// 是否保存失败的命令
    #[serde(default = "default_true")]
    pub save_failed: bool,
}

/// 命令生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// 是否自动执行生成的命令
    #[serde(default)]
    pub auto_execute: bool,

    /// 是否显示命令解释
    #[serde(default = "default_true")]
    pub show_explanation: bool,

    /// 是否显示安全警告
    #[serde(default = "default_true")]
    pub show_safety_warning: bool,

    /// 危险命令关键词列表
    #[serde(default = "default_dangerous_keywords")]
    pub dangerous_keywords: Vec<String>,
}

/// UI 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// 是否使用彩色输出
    #[serde(default = "default_true")]
    pub color: bool,

    /// 是否使用 emoji
    #[serde(default = "default_true")]
    pub emoji: bool,

    /// 是否显示进度条
    #[serde(default = "default_true")]
    pub progress_bar: bool,
}

// 默认值函数
fn default_engine_type() -> String { "mock".to_string() }
fn default_api_url() -> String { "http://localhost:11434".to_string() }
fn default_model_name() -> String { "qwen2.5:1.5b".to_string() }
fn default_context_size() -> usize { 2048 }
fn default_temperature() -> f32 { 0.7 }
fn default_top_p() -> f32 { 0.9 }
fn default_top_k() -> i32 { 40 }
fn default_max_tokens() -> usize { 512 }
fn default_max_history() -> usize { 1000 }
fn default_true() -> bool { true }

fn default_dangerous_keywords() -> Vec<String> {
    vec![
        "rm -rf".to_string(),
        "format".to_string(),
        "del /f".to_string(),
        "mkfs".to_string(),
        "dd if=".to_string(),
        "> /dev/".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().expect("无法获取用户主目录");
        let config_dir = home.join(".termgenius");

        Self {
            model: ModelConfig {
                engine_type: default_engine_type(),
                api_url: default_api_url(),
                model_name: default_model_name(),
                path: config_dir.join("models").join("qwen1_5-1_8b-chat-q4_0.gguf"),
                context_size: default_context_size(),
                gpu_layers: 0,
                threads: 0, // 自动检测
                temperature: default_temperature(),
                top_p: default_top_p(),
                top_k: default_top_k(),
                max_tokens: default_max_tokens(),
            },
            history: HistoryConfig {
                enabled: true,
                path: Some(config_dir.join("history.json")),
                max_entries: default_max_history(),
                save_failed: true,
            },
            generation: GenerationConfig {
                auto_execute: false,
                show_explanation: true,
                show_safety_warning: true,
                dangerous_keywords: default_dangerous_keywords(),
            },
            ui: UiConfig {
                color: true,
                emoji: true,
                progress_bar: true,
            },
        }
    }
}

impl Config {
    /// 检查命令是否危险
    pub fn is_dangerous_command(&self, command: &str) -> bool {
        self.generation.dangerous_keywords.iter()
            .any(|keyword| command.contains(keyword))
    }

    /// 获取实际使用的线程数
    pub fn get_threads(&self) -> usize {
        if self.model.threads == 0 {
            num_cpus::get()
        } else {
            self.model.threads
        }
    }
}
