# TermGenius 使用指南

本文档详细介绍 TermGenius 的使用方法和最佳实践。

## 目录

- [基本使用](#基本使用)
- [配置管理](#配置管理)
- [历史记录](#历史记录)
- [引擎配置](#引擎配置)
- [最佳实践](#最佳实践)
- [故障排除](#故障排除)

## 基本使用

### 命令生成

TermGenius 的核心功能是将自然语言转换为 Shell 命令。

```bash
# 基本语法
tg "自然语言描述"

# 示例
tg "列出当前目录的所有文件"
# 输出: ls -la

tg "查找大于 100MB 的文件"
# 输出: find . -type f -size +100M

tg "查看占用端口 8080 的进程"
# 输出: lsof -i :8080  (Linux/macOS)
# 输出: netstat -ano | findstr :8080  (Windows)
```

**提示**：
- 描述越具体，生成的命令越准确
- 可以使用中文或英文
- 生成的命令会根据当前操作系统自动调整

### 代码生成

生成完整的脚本或代码片段。

```bash
# 基本语法
tg code "代码需求描述"

# 示例
tg code "写一个 Python 脚本批量重命名文件"
tg code "Shell 脚本备份数据库"
tg code "Python 读取 CSV 文件并统计"
```

生成的代码会包含：
- 完整的代码实现
- 必要的注释
- 使用说明

### 命令修复

当命令执行失败时，TermGenius 可以分析错误并提供修复建议。

```bash
# 执行一个会失败的命令
$ rm /root/test.txt
# Permission denied

# 使用 fix 命令
tg fix

# 输出：
# 失败的命令: rm /root/test.txt
# 错误信息: Permission denied
# 错误类型: PermissionDenied
# 修复建议: sudo rm /root/test.txt
```

**工作原理**：
1. TermGenius 会记录失败的命令
2. `tg fix` 会获取最后一个失败的命令
3. 分析错误类型（权限、文件不存在、语法错误等）
4. 提供修复建议

## 配置管理

### 查看配置

```bash
tg config --show
```

输出示例：
```
TermGenius 配置

引擎:
  类型: ollama
  API 地址: http://localhost:11434
  模型名称: qwen2.5:1.5b

模型:
  路径: ~/.termgenius/models/qwen1_5-1_8b-chat-q4_0.gguf
  上下文大小: 2048
  温度: 0.7

历史记录:
  启用: true
  最大记录数: 1000

...
```

### 修改配置

配置文件位于 `~/.termgenius/config.toml`，可以直接编辑：

```bash
# Linux/macOS
vim ~/.termgenius/config.toml

# Windows
notepad %USERPROFILE%\.termgenius\config.toml
```

### 重置配置

```bash
tg config --reset
```

这会将配置恢复为默认值。

## 历史记录

### 查看历史

```bash
# 查看最近的历史记录
tg history

# 输出示例：
# 📜 最近的命令历史 (共 5 条):
#
# [1] ⏸️ 2026-05-02 11:30:15
#   输入: 列出所有文件
#   命令: ls -la
#
# [2] ✅ 2026-05-02 11:25:10
#   输入: 查找大文件
#   命令: find . -type f -size +100M
#   耗时: 150 ms
```

### 搜索历史

```bash
# 搜索包含关键词的历史记录
tg history --search "git"
tg history --search "文件"
```

### 清空历史

```bash
tg history --clear
```

**注意**：此操作不可恢复！

### 历史记录状态

历史记录有三种状态：
- ⏸️ **未执行**：命令已生成但未执行
- ✅ **成功**：命令执行成功
- ❌ **失败**：命令执行失败（包含错误信息）

## 引擎配置

TermGenius 支持多种 LLM 引擎，可以根据需求选择。

### Ollama（推荐）

**优点**：
- 安装简单
- 模型管理方便
- 性能优秀
- 社区活跃

**配置步骤**：

1. 安装 Ollama
```bash
# 访问 https://ollama.ai 下载安装
```

2. 下载模型
```bash
# 推荐模型
ollama pull qwen2.5:1.5b      # 小巧快速
ollama pull qwen2.5:3b        # 平衡性能
ollama pull deepseek-coder:1.3b  # 代码生成
```

3. 配置 TermGenius
```toml
[model]
engine_type = "ollama"
api_url = "http://localhost:11434"
model_name = "qwen2.5:1.5b"
```

4. 测试
```bash
tg "测试命令"
```

### llama.cpp server

**优点**：
- 更底层的控制
- 支持更多模型格式
- 可自定义参数

**配置步骤**：

1. 下载 llama.cpp
```bash
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp
make
```

2. 下载模型（GGUF 格式）
```bash
# 从 Hugging Face 下载
# 例如: qwen1_5-1_8b-chat-q4_0.gguf
```

3. 启动 server
```bash
./llama-server -m models/qwen-1.5b-q4.gguf --port 8080
```

4. 配置 TermGenius
```toml
[model]
engine_type = "llama_cpp"
api_url = "http://localhost:8080"
```

### Mock 引擎（测试）

**用途**：
- 测试 TermGenius 功能
- 开发调试
- 无需真实 LLM

**配置**：
```toml
[model]
engine_type = "mock"
```

**限制**：
- 只能生成预设的示例命令
- 不能处理复杂需求

## 最佳实践

### 1. 描述要具体

❌ 不好的描述：
```bash
tg "删除文件"
```

✅ 好的描述：
```bash
tg "删除 /tmp 目录下 7 天前的 .log 文件"
```

### 2. 检查后再执行

TermGenius 生成的命令可能不完全符合预期，**务必检查后再执行**，特别是：
- 删除操作
- 修改系统配置
- 批量操作

### 3. 使用历史记录

经常使用的命令可以通过历史记录快速查找：
```bash
tg history --search "备份"
```

### 4. 调整温度参数

温度参数影响生成的随机性：
- `0.1-0.3`：更确定，适合命令生成
- `0.5-0.7`：平衡，适合代码生成
- `0.8-1.0`：更创造性，适合文本生成

```toml
[model]
temperature = 0.3  # 命令生成推荐值
```

### 5. 选择合适的模型

| 模型 | 大小 | 速度 | 质量 | 适用场景 |
|------|------|------|------|----------|
| qwen2.5:1.5b | 1.5GB | 快 | 中 | 简单命令 |
| qwen2.5:3b | 3GB | 中 | 高 | 复杂命令 |
| deepseek-coder:1.3b | 1.3GB | 快 | 高 | 代码生成 |

## 故障排除

### 问题 1：无法连接到 Ollama

**症状**：
```
❌ 初始化失败: 连接失败: Connection refused
```

**解决方案**：
1. 检查 Ollama 是否运行
```bash
# Linux/macOS
ps aux | grep ollama

# Windows
tasklist | findstr ollama
```

2. 启动 Ollama
```bash
ollama serve
```

3. 检查端口
```bash
# 默认端口 11434
curl http://localhost:11434/api/tags
```

### 问题 2：模型未找到

**症状**：
```
❌ 命令生成失败: model not found
```

**解决方案**：
1. 列出已安装的模型
```bash
ollama list
```

2. 下载模型
```bash
ollama pull qwen2.5:1.5b
```

3. 检查配置文件中的模型名称是否正确

### 问题 3：生成速度慢

**可能原因**：
- 模型太大
- CPU 性能不足
- 没有使用 GPU

**解决方案**：
1. 使用更小的模型
```bash
ollama pull qwen2.5:1.5b  # 而不是 7b
```

2. 调整并发参数（Ollama）
```bash
OLLAMA_NUM_PARALLEL=1 ollama serve
```

3. 使用 GPU（如果有）
```bash
# Ollama 会自动检测并使用 GPU
```

### 问题 4：生成的命令不准确

**解决方案**：
1. 使用更具体的描述
2. 尝试不同的模型
3. 调整温度参数（降低温度）
```toml
[model]
temperature = 0.2
```

### 问题 5：配置文件损坏

**症状**：
```
❌ 配置初始化失败: 解析配置文件失败
```

**解决方案**：
1. 重置配置
```bash
tg config --reset
```

2. 或手动删除配置文件
```bash
# Linux/macOS
rm ~/.termgenius/config.toml

# Windows
del %USERPROFILE%\.termgenius\config.toml
```

## 获取帮助

如果遇到问题：

1. 查看帮助信息
```bash
tg --help
tg <command> --help
```

2. 查看日志
```bash
# 日志会输出到终端
# 可以使用 RUST_LOG 环境变量调整日志级别
RUST_LOG=debug tg "测试"
```

3. 提交 Issue
- GitHub: https://github.com/liujycode/TermGenius/issues
- 包含：
  - 操作系统和版本
  - TermGenius 版本
  - 完整的错误信息
  - 复现步骤

---

更多信息请访问：
- [GitHub 仓库](https://github.com/liujycode/TermGenius)
- [常见问题](FAQ.md)
