# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- 📹 演示视频
- 🚀 多平台二进制文件构建
- 🤖 支持更多 LLM 引擎（OpenAI API、Claude API）
- 🧠 命令历史学习功能
- 🖥️ 桌面版（Tauri）
- 🔌 VS Code 插件

## [0.1.0] - 2026-05-02

### Added
- 🎉 首个 MVP 版本发布
- ✨ 自然语言生成 Shell 命令功能
- ✨ 代码生成功能（Python/Shell 等脚本）
- ✨ 命令修复功能（分析失败命令并提供修复建议）
- ✨ 历史记录功能（保存和搜索历史命令）
- 🔌 多 LLM 引擎支持：
  - Ollama API（推荐）
  - llama.cpp server
  - Mock 引擎（测试用）
- 🖥️ 终端集成脚本：
  - Bash/Zsh 集成（`scripts/shell-integration.sh`）
  - PowerShell 集成（`scripts/shell-integration.ps1`）
  - 快捷命令：`tgc`、`tgx`、`tgcode`、`tgfix`、`tgh`、`tgconf`
- 📝 完整的项目文档：
  - README.md - 项目说明
  - USAGE.md - 详细使用指南
  - SHELL_INTEGRATION.md - 终端集成指南
  - PROJECT_RECORD.md - 开发记录
- 🧪 完整的单元测试套件（37 个测试，97.4% 通过率）
- 🎯 改进的 Prompt 工程：
  - 强化输出格式约束
  - 添加错误示例 vs 正确示例对比
  - 明确操作系统环境约束
  - 自动清理输出中的解释文字

### Changed
- 🔄 从 llama-cpp-2 本地模型切换到 HTTP API 方式
- 🔄 优化命令生成质量（不再混用 Windows/Linux 命令）
- 🔄 改进 Windows 命令示例（添加 forfiles、tasklist 等）

### Fixed
- 🐛 修复 Windows 编译环境问题（切换到 MSVC 工具链）
- 🐛 修复类型不匹配问题（max_tokens 类型转换）
- 🐛 修复命令生成输出包含解释文字的问题
- 🐛 修复复杂命令混用不同操作系统命令的问题

### Technical Details
- **语言**: Rust 1.95+
- **CLI 框架**: clap 4.5
- **HTTP 客户端**: reqwest 0.12
- **配置格式**: TOML
- **历史记录格式**: JSON
- **日志**: tracing + tracing-subscriber

### Performance
- 简单命令生成：~13 秒
- 复杂命令生成：~15 秒
- 代码生成：~43 秒
- 可执行文件大小：1.3 MB（Release 模式）

### Known Issues
- 响应速度依赖于 LLM 引擎性能
- 复杂命令的生成质量依赖于模型能力
- Windows 环境下需要 MSVC 工具链

### Documentation
- 详细的测试报告：`OLLAMA_TEST_REPORT.md`
- 工作总结：`WORK_SUMMARY_20260502.md`
- 开发记录：`PROJECT_RECORD.md`

---

[0.1.0]: https://github.com/liujycode/TermGenius/releases/tag/v0.1.0
