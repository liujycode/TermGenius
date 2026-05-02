# TermGenius - 本地离线 AI 终端助手

<div align="center">

**让终端操作更智能 | 支持多种 LLM 引擎 | 灵活部署**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.95+-orange.svg)](https://www.rust-lang.org/)

[English](README.md) | [简体中文](README_zh.md)

</div>

## 🚀 核心特性

- 🧠 **AI 驱动**：自然语言转命令，告别记忆复杂参数
- 🔧 **智能修复**：命令报错自动诊断并提供修复方案
- 💻 **代码生成**：终端内直接生成 Python/Shell/其他脚本
- 🔒 **隐私优先**：支持本地部署，数据不上传
- 🎯 **多引擎支持**：
  - Ollama API（推荐）
  - llama.cpp server
  - Mock 引擎（测试）
- ⚡ **高性能**：Rust 编写，启动快速，资源占用低
- 🌍 **全平台**：支持 Linux/macOS/Windows + bash/zsh/PowerShell

## 📦 快速开始

### 安装

#### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/liujycode/TermGenius.git
cd TermGenius

# 编译（需要 Rust 1.95+）
cargo build --release

# 安装到系统
cargo install --path .
```

### 配置 LLM 引擎

#### 使用 Ollama（推荐）

```bash
# 1. 安装 Ollama
# 访问 https://ollama.ai 下载安装

# 2. 下载模型
ollama pull qwen2.5:1.5b

# 3. 启动 Ollama 服务（通常会自动启动）
ollama serve
```

编辑配置文件 `~/.termgenius/config.toml`：

```toml
[model]
engine_type = "ollama"
api_url = "http://localhost:11434"
model_name = "qwen2.5:1.5b"
```

### 第一个命令

```bash
# 自然语言生成命令
tg "列出所有大于100MB的文件"
# 输出: find . -type f -size +100M

# 生成代码
tg code "写一个Python脚本批量重命名文件"

# 查看配置
tg config --show
```

## 🎯 使用场景

### 命令生成
```bash
tg "查找7天前修改的日志文件"
# → find /var/log -name "*.log" -mtime +7

tg "查看占用端口8080的进程"
# → lsof -i :8080  (Linux/macOS)
# → netstat -ano | findstr :8080  (Windows)
```

### 命令修复
```bash
# 执行失败的命令会被记录
$ rm /root/test.txt
# Permission denied

# 使用 fix 命令修复
tg fix
# → 分析错误类型
# → 提供修复建议: sudo rm /root/test.txt
```

### 代码生成
```bash
tg code "写一个Python脚本批量重命名文件"
# → 生成完整的 Python 脚本 + 使用说明

tg code "Shell脚本备份数据库"
# → 生成 Shell 脚本
```

### 历史记录
```bash
# 查看历史
tg history

# 搜索历史
tg history --search "git"

# 清空历史
tg history --clear
```

## 📖 命令参考

| 命令 | 说明 | 示例 |
|------|------|------|
| `tg "描述"` | 生成 Shell 命令 | `tg "删除 7 天前的日志文件"` |
| `tg code "需求"` | 生成代码或脚本 | `tg code "Python 读取 CSV 文件"` |
| `tg fix` | 修复上一个失败的命令 | `tg fix` |
| `tg history` | 查看命令历史 | `tg history` |
| `tg history --search <关键词>` | 搜索历史记录 | `tg history --search git` |
| `tg history --clear` | 清空历史记录 | `tg history --clear` |
| `tg config --show` | 显示当前配置 | `tg config --show` |
| `tg uninstall` | 卸载指南 | `tg uninstall` |

## 🛠️ 配置文件

配置文件位于 `~/.termgenius/config.toml`：

```toml
[model]
# 引擎类型: mock, ollama, llama_cpp
engine_type = "ollama"
# API 地址（用于 HTTP API 引擎）
api_url = "http://localhost:11434"
# 模型名称（用于 HTTP API 引擎）
model_name = "qwen2.5:1.5b"
# 温度参数（0.0-1.0，越高越随机）
temperature = 0.7
# 最大生成 token 数
max_tokens = 512

[history]
# 是否启用历史记录
enabled = true
# 最大历史记录数
max_entries = 1000

[generation]
# 是否自动执行生成的命令（危险！）
auto_execute = false
# 是否显示命令解释
show_explanation = true
# 是否显示安全警告
show_safety_warning = true

[ui]
# 是否使用彩色输出
color = true
# 是否使用 emoji
emoji = true
```

## 🔧 高级用法

### 使用 llama.cpp server

```bash
# 1. 启动 llama.cpp server
./llama-server -m models/qwen-1.5b-q4.gguf --port 8080

# 2. 配置 TermGenius
# 编辑 ~/.termgenius/config.toml
[model]
engine_type = "llama_cpp"
api_url = "http://localhost:8080"
```

### 终端集成

#### Bash/Zsh

在 `~/.bashrc` 或 `~/.zshrc` 中添加：

```bash
# TermGenius 别名
alias tg='termgenius'
```

#### PowerShell

在 PowerShell 配置文件中添加：

```powershell
# TermGenius 别名
Set-Alias tg termgenius
```

## 🌟 技术栈

- **语言**: Rust 1.95+
- **LLM 引擎**: 
  - Ollama API
  - llama.cpp server
  - Mock 引擎（测试）
- **HTTP 客户端**: reqwest
- **终端 UI**: Ratatui + Crossterm
- **配置管理**: TOML
- **历史记录**: JSON

## 🗺️ Roadmap

- [x] MVP 版本（命令生成 + 错误修复）
- [x] 多引擎支持（Ollama + llama.cpp）
- [x] 历史记录管理
- [x] 配置管理
- [ ] 终端集成脚本
- [ ] 自动补全支持
- [ ] 桌面版（Tauri）
- [ ] VS Code 插件

## 🤝 贡献

欢迎贡献代码、报告 Bug、提出建议！

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源。

## 🙏 致谢

- [Ollama](https://ollama.ai) - 简单易用的本地 LLM 运行工具
- [llama.cpp](https://github.com/ggerganov/llama.cpp) - 高性能 LLM 推理引擎
- [Qwen](https://github.com/QwenLM/Qwen) - 优秀的开源大语言模型
- [Ratatui](https://github.com/ratatui-org/ratatui) - 终端 UI 框架

## 📮 联系方式

- GitHub Issues: [提交问题](https://github.com/liujycode/TermGenius/issues)
- Email: liujycode@gmail.com

---

<div align="center">

**如果觉得有用，请给个 ⭐️ Star！**

Made with ❤️ by [liujycode](https://github.com/liujycode)

</div>
