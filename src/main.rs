use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::error;

mod llm;
mod commands;

use commands::CommandGenerator;

#[derive(Parser)]
#[command(name = "termgenius")]
#[command(author = "liujycode <liujycode@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "本地离线 AI 终端助手", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 自然语言描述（直接生成命令）
    #[arg(value_name = "PROMPT")]
    prompt: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// 生成代码或脚本
    Code {
        /// 代码需求描述
        prompt: String,
    },
    /// 修复上一个失败的命令
    Fix {
        /// 是否使用上一个错误
        #[arg(long)]
        last_error: bool,
    },
    /// 查看命令历史
    History {
        /// 清理历史记录
        #[arg(long)]
        clear: bool,
    },
    /// 配置管理
    Config {
        /// 设置模型
        #[arg(long)]
        model: Option<String>,

        /// 启用/禁用历史记录
        #[arg(long)]
        history_enabled: Option<bool>,
    },
    /// 卸载 TermGenius
    Uninstall,
}

fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // 获取模型路径
    let model_path = get_model_path();

    match &cli.command {
        Some(Commands::Code { prompt }) => {
            handle_code_generation(&model_path, prompt);
        }
        Some(Commands::Fix { last_error }) => {
            handle_fix_command(&model_path, *last_error);
        }
        Some(Commands::History { clear }) => {
            if *clear {
                println!("清理历史记录");
                // TODO: 实现历史清理
            } else {
                println!("查看历史记录");
                // TODO: 实现历史查看
            }
        }
        Some(Commands::Config { model, history_enabled }) => {
            if let Some(m) = model {
                println!("设置模型: {}", m);
                // TODO: 实现模型切换
            }
            if let Some(enabled) = history_enabled {
                println!("历史记录: {}", if *enabled { "启用" } else { "禁用" });
                // TODO: 实现历史记录开关
            }
        }
        Some(Commands::Uninstall) => {
            println!("卸载 TermGenius");
            // TODO: 实现卸载逻辑
        }
        None => {
            if let Some(prompt) = &cli.prompt {
                handle_command_generation(&model_path, prompt);
            } else {
                println!("TermGenius v0.1.0");
                println!("使用 --help 查看帮助");
            }
        }
    }
}

/// 获取模型路径
fn get_model_path() -> PathBuf {
    // 优先从环境变量读取
    if let Ok(path) = std::env::var("TERMGENIUS_MODEL") {
        return PathBuf::from(path);
    }

    // 默认路径
    let home = dirs::home_dir().expect("无法获取用户主目录");
    home.join(".termgenius").join("models").join("qwen-1.8b-q4_0.gguf")
}

/// 处理命令生成
fn handle_command_generation(model_path: &PathBuf, prompt: &str) {
    println!("🤖 正在生成命令...");

    match CommandGenerator::new(model_path.clone()) {
        Ok(mut generator) => {
            match generator.generate_command(prompt) {
                Ok(command) => {
                    println!("\n✅ 生成的命令:");
                    println!("   {}", command);
                    println!("\n💡 提示: 请仔细检查命令后再执行");
                }
                Err(e) => {
                    error!("命令生成失败: {}", e);
                    println!("❌ 命令生成失败: {}", e);
                }
            }
        }
        Err(e) => {
            error!("初始化失败: {}", e);
            println!("❌ 初始化失败: {}", e);
            println!("\n💡 提示:");
            println!("   1. 请确保已下载模型文件");
            println!("   2. 模型路径: {}", model_path.display());
            println!("   3. 可通过环境变量 TERMGENIUS_MODEL 指定模型路径");
        }
    }
}

/// 处理代码生成
fn handle_code_generation(model_path: &PathBuf, prompt: &str) {
    println!("🤖 正在生成代码...");

    match CommandGenerator::new(model_path.clone()) {
        Ok(mut generator) => {
            match generator.generate_code(prompt) {
                Ok(code) => {
                    println!("\n✅ 生成的代码:");
                    println!("{}", code);
                }
                Err(e) => {
                    error!("代码生成失败: {}", e);
                    println!("❌ 代码生成失败: {}", e);
                }
            }
        }
        Err(e) => {
            error!("初始化失败: {}", e);
            println!("❌ 初始化失败: {}", e);
        }
    }
}

/// 处理命令修复
fn handle_fix_command(model_path: &PathBuf, _last_error: bool) {
    println!("🤖 正在修复命令...");
    println!("⚠️  此功能尚未实现");
    println!("\n💡 提示:");
    println!("   需要实现命令历史和错误捕获功能");
    // TODO: 实现错误修复
}
