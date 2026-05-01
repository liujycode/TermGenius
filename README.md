# TermGenius - 本地离线 AI 终端助手

<div align="center">

![TermGenius Logo](docs/assets/logo.png)

**让终端操作更智能 | 100% 本地运行 | 零配置开箱即用**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![GitHub Stars](https://img.shields.io/github/stars/liujycode/TermGenius?style=social)](https://github.com/liujycode/TermGenius)

[English](README.md) | [简体中文](README_zh.md)

</div>

## 🚀 核心特性

- 🧠 **AI 驱动**：自然语言转命令，告别记忆复杂参数
- 🔧 **智能修复**：命令报错自动诊断并提供修复方案
- 💻 **编程辅助**：终端内直接生成 Python/Go/Shell 脚本
- 🔒 **隐私优先**：100% 本地运行，代码和数据不上传
- ⚡ **零配置**：一键安装，自动下载模型，开箱即用
- 🌍 **全平台**：支持 Linux/macOS/Windows + bash/zsh/fish/PowerShell

## 📦 快速开始

### 一键安装

**Linux / macOS**:
```bash
curl -fsSL https://raw.githubusercontent.com/liujycode/TermGenius/main/scripts/install.sh | bash
```

**Windows (PowerShell)**:
```powershell
irm https://raw.githubusercontent.com/liujycode/TermGenius/main/scripts/install.ps1 | iex
```

### 验证安装

```bash
tg --version
```

### 第一个命令

```bash
# 自然语言生成命令
tg "列出所有大于100MB的文件"

# 输出：
# [绿色] find . -type f -size +100M
# [灰色] 在当前目录递归查找所有大于100MB的文件
# [Enter 执行] [Ctrl+E 编辑] [Esc 取消]
```

## 🎯 使用场景

### 命令生成
```bash
tg "查找7天前修改的日志文件并删除"
# → find /var/log -name "*.log" -mtime +7 -delete
```

### 错误修复
```bash
$ rm /root/test.txt
# Permission denied

# TermGenius 自动弹出：
# [红色] 错误：权限不足
# [绿色] 建议：sudo rm /root/test.txt
# [Enter 执行修复]
```

### 编程辅助
```bash
tg code "写一个Python脚本批量重命名文件"
# → 生成完整的 Python 脚本 + 使用说明
```

## 📖 文档

- [安装指南](docs/installation.md)
- [使用教程](docs/usage.md)
- [常见问题](docs/faq.md)
- [架构设计](docs/architecture.md)
- [贡献指南](CONTRIBUTING.md)

## 🛠️ 技术栈

- **语言**: Rust
- **LLM 引擎**: llama.cpp
- **模型**: Qwen 1.8B / DeepSeek-Coder 1.3B
- **终端 UI**: Ratatui
- **跨平台**: 单二进制，无依赖

## 🌟 为什么选择 TermGenius？

| 特性 | TermGenius | GitHub Copilot CLI | Warp Terminal | tldr |
|------|-----------|-------------------|---------------|------|
| 本地离线 | ✅ | ❌ | ❌ | ✅ |
| AI 能力 | ✅ | ✅ | ✅ | ❌ |
| 编程辅助 | ✅ | ⚠️ | ❌ | ❌ |
| 零配置 | ✅ | ✅ | ✅ | ✅ |
| 全平台 | ✅ | ✅ | ⚠️ | ✅ |
| 价格 | 免费 | $10/月 | 部分免费 | 免费 |

## 🗺️ Roadmap

- [x] MVP 版本（命令生成 + 错误修复）
- [ ] 编程辅助（Python/Go/Shell）
- [ ] 命令历史学习
- [ ] 桌面版（Tauri）
- [ ] VS Code 插件
- [ ] 企业版（批量部署 + 权限管理）

## 🤝 贡献

欢迎贡献代码、报告 Bug、提出建议！

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

详见 [贡献指南](CONTRIBUTING.md)

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源。

## 🙏 致谢

- [llama.cpp](https://github.com/ggerganov/llama.cpp) - 高性能 LLM 推理引擎
- [Qwen](https://github.com/QwenLM/Qwen) - 优秀的开源模型
- [Ratatui](https://github.com/ratatui-org/ratatui) - 终端 UI 框架

## 📮 联系我们

- GitHub Issues: [提交问题](https://github.com/liujycode/TermGenius/issues)
- Email: liujycode@gmail.com

---

<div align="center">

**如果觉得有用，请给个 ⭐️ Star！**

Made with ❤️ by [liujycode](https://github.com/liujycode)

</div>
