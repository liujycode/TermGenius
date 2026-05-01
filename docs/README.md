# TermGenius 项目文档

## 快速导航

- [安装指南](installation.md)
- [使用教程](usage.md)
- [常见问题](faq.md)
- [架构设计](architecture.md)
- [开发指南](../CONTRIBUTING.md)

## 项目概述

TermGenius 是一个 100% 本地离线的 AI 终端助手，旨在让终端操作更智能、更高效。

### 核心功能

1. **自然语言转命令**：用自然语言描述需求，AI 自动生成对应的 Shell 命令
2. **智能错误修复**：命令执行失败时，自动分析错误并提供修复方案
3. **终端内编程辅助**：直接在终端生成 Python/Go/Shell 脚本
4. **命令历史学习**：学习你的使用习惯，提供个性化推荐

### 技术特点

- **本地运行**：所有数据处理在本地完成，无需联网
- **轻量高效**：单二进制文件，约 10MB，启动速度快
- **跨平台**：支持 Linux/macOS/Windows 三大平台
- **零配置**：一键安装，自动下载模型，开箱即用

## 文档结构

```
docs/
├── README.md           # 本文件
├── installation.md     # 安装指南
├── usage.md            # 使用教程
├── faq.md              # 常见问题
├── architecture.md     # 架构设计
└── assets/             # 图片资源
```

## 贡献文档

如果你发现文档有错误或需要改进，欢迎：

1. 提交 Issue 报告问题
2. 提交 PR 改进文档
3. 在 Discussions 中提出建议

## 许可证

文档采用 [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/) 许可证。
