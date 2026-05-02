use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::error;

mod llm;
mod commands;
mod config;
mod history;
mod error_capture;

use commands::CommandGenerator;
use config::ConfigManager;
use history::HistoryManager;
use error_capture::ErrorAnalyzer;

#[derive(Parser)]
#[command(name = "termgenius")]
#[command(author = "liujycode <liujiye36@gmail.com>")]
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

        /// 搜索历史记录
        #[arg(long)]
        search: Option<String>,
    },
    /// 配置管理
    Config {
        /// 设置模型
        #[arg(long)]
        model: Option<String>,

        /// 启用/禁用历史记录
        #[arg(long)]
        history_enabled: Option<bool>,

        /// 显示当前配置
        #[arg(long)]
        show: bool,

        /// 重置为默认配置
        #[arg(long)]
        reset: bool,
    },
    /// 卸载 TermGenius
    Uninstall,
}

fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // 初始化配置管理器
    let mut config_manager = match ConfigManager::new() {
        Ok(cm) => cm,
        Err(e) => {
            eprintln!("❌ 配置初始化失败: {}", e);
            std::process::exit(1);
        }
    };

    match &cli.command {
        Some(Commands::Code { prompt }) => {
            handle_code_generation(&config_manager, prompt);
        }
        Some(Commands::Fix { last_error }) => {
            handle_fix_command(&config_manager, *last_error);
        }
        Some(Commands::History { clear, search }) => {
            handle_history_command(&config_manager, *clear, search.as_deref());
        }
        Some(Commands::Config { model, history_enabled, show, reset }) => {
            handle_config_command(&mut config_manager, model.as_deref(), *history_enabled, *show, *reset);
        }
        Some(Commands::Uninstall) => {
            handle_uninstall();
        }
        None => {
            if let Some(prompt) = &cli.prompt {
                handle_command_generation(&config_manager, prompt);
            } else {
                println!("TermGenius v0.1.0 - 本地离线 AI 终端助手");
                println!();
                println!("使用方法:");
                println!("  tg \"自然语言描述\"     生成命令");
                println!("  tg code \"代码需求\"    生成代码");
                println!("  tg fix                修复上一个失败的命令");
                println!("  tg history            查看命令历史");
                println!("  tg config --show      查看配置");
                println!();
                println!("使用 --help 查看完整帮助");
            }
        }
    }
}

/// 处理命令生成
fn handle_command_generation(config_manager: &ConfigManager, prompt: &str) {
    println!("🤖 正在生成命令...");

    match CommandGenerator::from_config(config_manager.config()) {
        Ok(mut generator) => {
            match generator.generate_command(prompt) {
                Ok(command) => {
                    println!("\n✅ 生成的命令:");
                    println!("   \x1b[32m{}\x1b[0m", command);

                    // 保存到历史记录
                    if config_manager.config().history.enabled {
                        if let Ok(mut history) = HistoryManager::new() {
                            let _ = history.add_entry(
                                prompt.to_string(),
                                command.clone(),
                                std::env::consts::OS.to_string(),
                                get_current_shell(),
                            );
                        }
                    }

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
            println!("   1. 检查配置文件中的引擎类型设置");
            println!("   2. 如果使用 Ollama，确保服务已启动");
            println!("   3. 可通过 tg config --show 查看当前配置");
        }
    }
}

/// 处理代码生成
fn handle_code_generation(config_manager: &ConfigManager, prompt: &str) {
    println!("🤖 正在生成代码...");

    match CommandGenerator::from_config(config_manager.config()) {
        Ok(mut generator) => {
            match generator.generate_code(prompt) {
                Ok(code) => {
                    println!("\n✅ 生成的代码:");
                    println!("{}", code);

                    // 保存到历史记录
                    if config_manager.config().history.enabled {
                        if let Ok(mut history) = HistoryManager::new() {
                            let _ = history.add_entry(
                                format!("code: {}", prompt),
                                code,
                                std::env::consts::OS.to_string(),
                                get_current_shell(),
                            );
                        }
                    }
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
fn handle_fix_command(config_manager: &ConfigManager, _last_error: bool) {
    println!("🤖 正在修复命令...");

    // 获取历史记录
    let mut history = match HistoryManager::new() {
        Ok(h) => h,
        Err(e) => {
            println!("❌ 无法加载历史记录: {}", e);
            return;
        }
    };

    // 获取最后一个失败的命令
    let last_failed = history.get_failed_commands(1);

    if last_failed.is_empty() {
        println!("⚠️  没有找到失败的命令");
        return;
    }

    let failed_entry = &last_failed[0];
    let command = &failed_entry.generated_command;
    let error = failed_entry.error_message.as_deref().unwrap_or("未知错误");

    println!("\n失败的命令: \x1b[31m{}\x1b[0m", command);
    println!("错误信息: {}", error);

    // 分析错误
    let analyzer = ErrorAnalyzer::new(std::env::consts::OS.to_string());
    let (error_type, suggestion) = analyzer.analyze(error);

    println!("\n错误类型: {:?}", error_type);
    println!("修复建议:");
    println!("   \x1b[32m{}\x1b[0m", suggestion);

    // 使用 LLM 生成修复命令
    match CommandGenerator::from_config(config_manager.config()) {
        Ok(mut generator) => {
            match generator.fix_command(command, error) {
                Ok(fixed_command) => {
                    println!("\n✅ 修复后的命令:");
                    println!("   \x1b[32m{}\x1b[0m", fixed_command);

                    // 保存到历史记录
                    if config_manager.config().history.enabled {
                        let _ = history.add_entry(
                            format!("fix: {}", command),
                            fixed_command,
                            std::env::consts::OS.to_string(),
                            get_current_shell(),
                        );
                    }
                }
                Err(e) => {
                    println!("❌ 命令修复失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 初始化失败: {}", e);
        }
    }
}

/// 处理历史记录命令
fn handle_history_command(config_manager: &ConfigManager, clear: bool, search: Option<&str>) {
    let mut history = match HistoryManager::new() {
        Ok(h) => h,
        Err(e) => {
            println!("❌ 无法加载历史记录: {}", e);
            return;
        }
    };

    if clear {
        match history.clear() {
            Ok(_) => println!("✅ 历史记录已清空"),
            Err(e) => println!("❌ 清空失败: {}", e),
        }
        return;
    }

    if let Some(query) = search {
        let results = history.search(query);
        if results.is_empty() {
            println!("⚠️  没有找到匹配的记录");
        } else {
            println!("🔍 搜索结果 (找到 {} 条):", results.len());
            for entry in results {
                println!("\n{}", entry.display());
            }
        }
        return;
    }

    // 显示最近的历史记录
    let max_entries = config_manager.config().history.max_entries;
    let entries = history.get_recent(max_entries);

    if entries.is_empty() {
        println!("⚠️  暂无历史记录");
    } else {
        println!("📜 最近的命令历史 (共 {} 条):", entries.len());
        for entry in entries {
            println!("\n{}", entry.display());
        }

        // 显示统计信息
        let stats = history.get_statistics();
        println!("\n📊 统计信息:");
        println!("   总记录数: {}", stats.total);
        println!("   成功: {} | 失败: {} | 未执行: {}",
                 stats.success, stats.failed, stats.not_executed);
    }
}

/// 处理配置命令
fn handle_config_command(
    config_manager: &mut ConfigManager,
    model: Option<&str>,
    history_enabled: Option<bool>,
    show: bool,
    reset: bool,
) {
    if reset {
        match config_manager.reset() {
            Ok(_) => println!("✅ 配置已重置为默认值"),
            Err(e) => println!("❌ 重置失败: {}", e),
        }
        return;
    }

    if show {
        println!("{}", config_manager.display());
        return;
    }

    let mut changed = false;

    if let Some(model_path) = model {
        match config_manager.set_model_path(PathBuf::from(model_path)) {
            Ok(_) => {
                println!("✅ 模型路径已更新: {}", model_path);
                changed = true;
            }
            Err(e) => println!("❌ 更新失败: {}", e),
        }
    }

    if let Some(enabled) = history_enabled {
        match config_manager.set_history_enabled(enabled) {
            Ok(_) => {
                println!("✅ 历史记录已{}", if enabled { "启用" } else { "禁用" });
                changed = true;
            }
            Err(e) => println!("❌ 更新失败: {}", e),
        }
    }

    if !changed {
        println!("⚠️  没有指定要修改的配置项");
        println!("使用 tg config --help 查看可用选项");
    }
}

/// 处理卸载
fn handle_uninstall() {
    println!("🗑️  卸载 TermGenius");
    println!();
    println!("请手动执行以下步骤:");
    println!("1. 删除配置目录:");

    if cfg!(windows) {
        println!("   rmdir /s /q %USERPROFILE%\\.termgenius");
    } else {
        println!("   rm -rf ~/.termgenius");
    }

    println!("2. 删除可执行文件:");
    if cfg!(windows) {
        println!("   del %USERPROFILE%\\.cargo\\bin\\termgenius.exe");
    } else {
        println!("   rm ~/.cargo/bin/termgenius");
    }

    println!("3. 从 shell 配置文件中移除相关配置");
    println!();
    println!("感谢使用 TermGenius！");
}

/// 获取当前 Shell
fn get_current_shell() -> String {
    std::env::var("SHELL")
        .unwrap_or_else(|_| {
            if cfg!(windows) {
                "powershell".to_string()
            } else {
                "bash".to_string()
            }
        })
        .split('/')
        .last()
        .unwrap_or("unknown")
        .to_string()
}
