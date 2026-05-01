//! 配置管理器

use crate::config::Config;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// 配置管理器
pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
}

impl ConfigManager {
    /// 加载配置
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        let config = if config_path.exists() {
            info!("加载配置文件: {}", config_path.display());
            Self::load_from_file(&config_path)?
        } else {
            info!("配置文件不存在，使用默认配置");
            let config = Config::default();

            // 创建默认配置文件
            if let Err(e) = Self::save_to_file(&config_path, &config) {
                warn!("无法保存默认配置: {}", e);
            }

            config
        };

        Ok(Self {
            config_path,
            config,
        })
    }

    /// 从文件加载配置
    fn load_from_file(path: &Path) -> Result<Config> {
        let content = fs::read_to_string(path)
            .context("读取配置文件失败")?;

        let config: Config = toml::from_str(&content)
            .context("解析配置文件失败")?;

        Ok(config)
    }

    /// 保存配置到文件
    fn save_to_file(path: &Path, config: &Config) -> Result<()> {
        // 确保目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("创建配置目录失败")?;
        }

        let content = toml::to_string_pretty(config)
            .context("序列化配置失败")?;

        fs::write(path, content)
            .context("写入配置文件失败")?;

        info!("配置已保存到: {}", path.display());
        Ok(())
    }

    /// 保存当前配置
    pub fn save(&self) -> Result<()> {
        Self::save_to_file(&self.config_path, &self.config)
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取可变配置
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// 获取配置文件路径
    fn get_config_path() -> Result<PathBuf> {
        // 优先从环境变量读取
        if let Ok(path) = std::env::var("TERMGENIUS_CONFIG") {
            return Ok(PathBuf::from(path));
        }

        // 默认路径
        let home = dirs::home_dir()
            .context("无法获取用户主目录")?;

        Ok(home.join(".termgenius").join("config.toml"))
    }

    /// 重置为默认配置
    pub fn reset(&mut self) -> Result<()> {
        info!("重置为默认配置");
        self.config = Config::default();
        self.save()
    }

    /// 更新模型路径
    pub fn set_model_path(&mut self, path: PathBuf) -> Result<()> {
        info!("更新模型路径: {}", path.display());
        self.config.model.path = path;
        self.save()
    }

    /// 更新历史记录开关
    pub fn set_history_enabled(&mut self, enabled: bool) -> Result<()> {
        info!("历史记录: {}", if enabled { "启用" } else { "禁用" });
        self.config.history.enabled = enabled;
        self.save()
    }

    /// 显示当前配置
    pub fn display(&self) -> String {
        format!(
            r#"TermGenius 配置

模型:
  路径: {}
  上下文大小: {}
  GPU 层数: {}
  线程数: {}
  温度: {}

历史记录:
  启用: {}
  最大记录数: {}

命令生成:
  自动执行: {}
  显示解释: {}
  安全警告: {}

UI:
  彩色输出: {}
  Emoji:
  进度条: {}

配置文件: {}"#,
            self.config.model.path.display(),
            self.config.model.context_size,
            self.config.model.gpu_layers,
            self.config.get_threads(),
            self.config.model.temperature,
            self.config.history.enabled,
            self.config.history.max_entries,
            self.config.generation.auto_execute,
            self.config.generation.show_explanation,
            self.config.generation.show_safety_warning,
            self.config.ui.color,
            self.config.ui.emoji,
            self.config.ui.progress_bar,
            self.config_path.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.model.context_size, 2048);
        assert_eq!(config.model.temperature, 0.7);
        assert!(config.history.enabled);
    }

    #[test]
    fn test_dangerous_command() {
        let config = Config::default();
        assert!(config.is_dangerous_command("rm -rf /"));
        assert!(config.is_dangerous_command("del /f /q C:\\"));
        assert!(!config.is_dangerous_command("ls -la"));
    }
}
