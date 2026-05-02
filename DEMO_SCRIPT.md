# TermGenius 演示视频脚本

**时长**: 3-5 分钟  
**目标**: 展示 TermGenius 的核心功能和使用场景

---

## 第一部分：介绍（30 秒）

### 画面
- 打开终端，显示项目 Logo 或标题

### 旁白/字幕
```
TermGenius - 智能终端助手
通过自然语言生成 Shell 命令和代码
让命令行操作更简单、更高效
```

### 操作
```bash
termgenius --version
# 输出: termgenius 0.1.0
```

---

## 第二部分：基础命令生成（1 分钟）

### 场景 1: 简单命令
**旁白**: "忘记了查找文件的命令？直接用自然语言描述"

```bash
termgenius "列出所有 Rust 文件"
# 输出: dir *.rs (Windows) 或 ls *.rs (Linux/macOS)
```

### 场景 2: 复杂命令
**旁白**: "复杂的命令也能轻松生成"

```bash
termgenius "查找大于 1MB 的文件"
# Windows 输出: forfiles /S /C "cmd /c if @fsize gtr 1048576 echo @path"
# Linux 输出: find . -type f -size +1M
```

### 场景 3: 系统命令
**旁白**: "系统管理命令也不在话下"

```bash
termgenius "显示当前运行的进程"
# Windows 输出: tasklist
# Linux 输出: ps aux
```

---

## 第三部分：代码生成（1 分钟）

### 场景 1: Python 脚本
**旁白**: "需要写脚本？TermGenius 帮你生成"

```bash
termgenius code "写一个 Python 脚本批量重命名文件"
```

**画面**: 显示生成的完整 Python 代码，包含：
- Shebang 行
- 导入语句
- 函数定义
- 错误处理
- 中文注释

### 场景 2: Shell 脚本
**旁白**: "Shell 脚本也能快速生成"

```bash
termgenius code "写一个 Shell 脚本备份目录"
```

**画面**: 显示生成的 Shell 脚本，包含：
- 错误检查（set -e）
- 函数定义
- 时间戳命名
- 完整注释

---

## 第四部分：终端集成（1 分钟）

### 场景 1: 快捷命令 tgc
**旁白**: "使用终端集成脚本，操作更便捷"

```bash
tgc "删除 7 天前的日志文件"
# 显示生成的命令，但不执行
```

### 场景 2: 快捷命令 tgx
**旁白**: "tgx 可以生成并执行命令"

```bash
tgx "创建一个名为 test 的目录"
# 显示生成的命令: mkdir test
# 提示: Execute this command? (y/n)
# 输入 y 后执行
```

### 场景 3: 历史记录
**旁白**: "所有命令都会保存到历史记录"

```bash
tgh
# 显示历史命令列表，包含时间戳和命令内容
```

**搜索历史**:
```bash
tgh --search "文件"
# 只显示包含"文件"的历史命令
```

---

## 第五部分：配置和引擎（30 秒）

### 查看配置
**旁白**: "TermGenius 支持多种 LLM 引擎"

```bash
tgconf
# 显示当前配置信息
```

**画面**: 显示配置文件内容
```toml
[model]
engine_type = "ollama"
api_url = "http://localhost:11434"
model_name = "qwen2.5:7b"
temperature = 0.7
max_tokens = 512
```

### 支持的引擎
**字幕**:
- ✅ Ollama API（推荐）
- ✅ llama.cpp server
- ✅ Mock 引擎（测试用）

---

## 第六部分：实际应用场景（1 分钟）

### 场景 1: 日常运维
**旁白**: "日常运维任务变得简单"

```bash
tgx "查看磁盘使用情况"
tgx "查找占用 8080 端口的进程"
tgx "压缩 logs 目录"
```

### 场景 2: 开发工作
**旁白**: "开发工作更高效"

```bash
tgc "查找所有 TODO 注释"
tgc "统计代码行数"
tgcode "写一个 Git pre-commit hook"
```

### 场景 3: 学习新命令
**旁白**: "学习新命令的好帮手"

```bash
termgenius "如何使用 rsync 同步文件"
# 生成带注释的 rsync 命令
```

---

## 第七部分：总结和展望（30 秒）

### 核心优势
**字幕**:
- ✨ 自然语言生成命令
- 🚀 支持多种 LLM 引擎
- 🖥️ 跨平台支持（Windows/Linux/macOS）
- 📝 完整的历史记录
- 🔧 终端集成脚本

### 未来计划
**字幕**:
- 🤖 更多 LLM 引擎支持
- 🧠 命令历史学习
- 🖥️ 桌面版（Tauri）
- 🔌 VS Code 插件

### 结束画面
**字幕**:
```
TermGenius v0.1.0
GitHub: https://github.com/liujycode/TermGenius
Star ⭐ 支持项目发展
```

---

## 录制建议

### 技术要求
1. **屏幕录制工具**:
   - Windows: OBS Studio / Camtasia
   - macOS: QuickTime / ScreenFlow
   - Linux: SimpleScreenRecorder / OBS Studio

2. **终端设置**:
   - 使用清晰的字体（Consolas / Fira Code）
   - 字体大小：16-18pt
   - 配色方案：高对比度（推荐 Dracula / Solarized Dark）
   - 窗口大小：1920x1080 或 1280x720

3. **录制参数**:
   - 分辨率：1920x1080 (1080p)
   - 帧率：30fps
   - 格式：MP4 (H.264)

### 录制技巧
1. **准备工作**:
   - 清空终端历史
   - 准备好测试文件和目录
   - 确保 Ollama 服务运行正常
   - 关闭不必要的通知

2. **录制过程**:
   - 每个命令执行后停顿 2-3 秒
   - 重要输出用鼠标高亮
   - 保持操作流畅，避免失误
   - 可以分段录制后剪辑

3. **后期处理**:
   - 添加字幕（中英文）
   - 添加背景音乐（轻音乐）
   - 添加转场效果
   - 添加 Logo 水印

### 发布渠道
- YouTube
- Bilibili
- GitHub README
- 项目官网

---

## 备用场景（可选）

### 命令修复功能
```bash
# 故意输入错误命令
ls -la
# 假设报错: command not found

# 使用修复功能
termgenius fix "ls -la" "command not found"
# 输出修复建议
```

### 多语言支持
```bash
termgenius "list all Python files"
# 英文输入也能正确识别
```

### 性能展示
**字幕**: "响应速度"
- 简单命令：~13 秒
- 复杂命令：~15 秒
- 代码生成：~43 秒

---

**脚本版本**: v1.0  
**创建日期**: 2026-05-02  
**适用版本**: TermGenius v0.1.0
