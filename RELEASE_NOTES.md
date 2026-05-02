# TermGenius v0.1.0 Release Notes

🎉 **首个 MVP 版本发布！**

TermGenius 是一个基于 Rust 开发的智能终端助手，通过自然语言生成 Shell 命令和代码，提升命令行操作效率。

## ✨ 核心功能

### 1. 命令生成
将自然语言转换为可执行的 Shell 命令：
```bash
termgenius "列出所有 Rust 文件"
# 输出: dir *.rs (Windows) 或 ls *.rs (Linux/macOS)
```

### 2. 代码生成
生成完整的脚本代码：
```bash
termgenius code "写一个 Python 脚本批量重命名文件"
# 输出: 完整的 Python 脚本，包含错误处理和注释
```

### 3. 命令修复
分析失败的命令并提供修复建议：
```bash
termgenius fix "ls -la" "command not found"
# 输出: 修复后的命令
```

### 4. 历史记录
保存和搜索历史命令：
```bash
termgenius history          # 查看所有历史
termgenius history --search "文件"  # 搜索包含"文件"的命令
```

## 🔌 多引擎支持

- **Ollama API**（推荐）- 本地运行，隐私安全
- **llama.cpp server** - 兼容 llama.cpp HTTP 服务
- **Mock 引擎** - 测试和演示用

## 🖥️ 终端集成

提供 Bash/Zsh/PowerShell 集成脚本，支持快捷命令：

- `tgc` - 生成命令（不执行）
- `tgx` - 生成并执行命令（需确认）
- `tgcode` - 生成代码
- `tgfix` - 修复失败的命令
- `tgh` - 查看/搜索历史
- `tgconf` - 查看配置

## 📦 安装

### 方法 1: 从 Release 下载

1. 下载对应平台的二进制文件：
   - Windows: `termgenius-windows-x86_64.exe`
   - Linux: `termgenius-linux-x86_64`
   - macOS (Intel): `termgenius-macos-x86_64`
   - macOS (Apple Silicon): `termgenius-macos-aarch64`

2. 重命名并添加到 PATH：
   ```bash
   # Windows
   rename termgenius-windows-x86_64.exe termgenius.exe
   
   # Linux/macOS
   mv termgenius-linux-x86_64 termgenius
   chmod +x termgenius
   sudo mv termgenius /usr/local/bin/
   ```

### 方法 2: 从源码编译

```bash
git clone https://github.com/liujycode/TermGenius.git
cd TermGenius
cargo build --release
```

## 🚀 快速开始

### 1. 安装 Ollama（推荐）

```bash
# 访问 https://ollama.ai 下载安装

# 拉取模型
ollama pull qwen2.5:7b
```

### 2. 配置 TermGenius

首次运行会自动创建配置文件 `~/.termgenius/config.toml`：

```toml
[model]
engine_type = "ollama"
api_url = "http://localhost:11434"
model_name = "qwen2.5:7b"
```

### 3. 开始使用

```bash
# 生成命令
termgenius "查找大于 1MB 的文件"

# 生成代码
termgenius code "写一个 Shell 脚本备份目录"

# 查看历史
termgenius history
```

## 📊 性能指标

- 简单命令生成：~13 秒
- 复杂命令生成：~15 秒
- 代码生成：~43 秒
- 可执行文件大小：1.3 MB

## 🧪 测试覆盖

- 37 个单元测试
- 97.4% 通过率
- 覆盖核心功能模块

## 📝 文档

- [README.md](README.md) - 项目说明
- [USAGE.md](USAGE.md) - 详细使用指南
- [SHELL_INTEGRATION.md](SHELL_INTEGRATION.md) - 终端集成指南
- [CHANGELOG.md](CHANGELOG.md) - 版本变更记录

## 🐛 已知问题

- 响应速度依赖于 LLM 引擎性能
- 复杂命令的生成质量依赖于模型能力
- Windows 环境下需要 MSVC 工具链

## 🔮 未来计划

- 支持更多 LLM 引擎（OpenAI API、Claude API）
- 命令历史学习功能
- 桌面版（Tauri）
- VS Code 插件

## 🙏 致谢

感谢所有测试和反馈的用户！

## 📧 联系方式

- **作者**: liujycode
- **Email**: liujiye36@gmail.com
- **GitHub**: https://github.com/liujycode/TermGenius

---

**完整变更日志**: [CHANGELOG.md](CHANGELOG.md)
