use clap::{Parser, Subcommand};

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

    match &cli.command {
        Some(Commands::Code { prompt }) => {
            println!("生成代码: {}", prompt);
            // TODO: 实现代码生成
        }
        Some(Commands::Fix { last_error }) => {
            println!("修复命令，使用上一个错误: {}", last_error);
            // TODO: 实现错误修复
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
                println!("生成命令: {}", prompt);
                // TODO: 实现命令生成
            } else {
                println!("TermGenius v0.1.0");
                println!("使用 --help 查看帮助");
            }
        }
    }
}
