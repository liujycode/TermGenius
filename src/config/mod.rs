//! 配置管理模块
//!
//! 负责读取、保存和管理 TermGenius 的配置

pub mod manager;
pub mod schema;

pub use manager::ConfigManager;
pub use schema::Config;
