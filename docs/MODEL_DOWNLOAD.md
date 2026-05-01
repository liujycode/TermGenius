# 模型下载说明

## 快速开始

### Windows
```bash
# 方法 1: 直接运行（前台）
python scripts/download_model.py qwen-1.8b-instruct

# 方法 2: 后台运行
scripts/download_background.bat
```

### Linux/macOS
```bash
# 方法 1: 直接运行（前台）
python3 scripts/download_model.py qwen-1.8b-instruct

# 方法 2: 后台运行
./scripts/download_background.sh
```

## 可用模型

| 模型 | 大小 | 说明 |
|------|------|------|
| qwen-1.8b-instruct | 1.1 GB | Qwen 1.5 1.8B Chat（推荐） |
| qwen-1.8b | 1.1 GB | Qwen 1.0 1.8B Chat（旧版） |
| deepseek-1.3b | 800 MB | DeepSeek Coder 1.3B |

## 下载源

| 源 | 说明 | 速度 |
|----|------|------|
| auto | 自动选择（推荐） | - |
| hf-mirror | HuggingFace 镜像 | 国内快 ⚡ |
| modelscope | ModelScope | 国内快 ⚡ |
| hf | HuggingFace 官方 | 国外快 |

## 高级用法

### 指定下载源
```bash
python scripts/download_model.py qwen-1.8b-instruct --source hf-mirror
```

### 指定输出目录
```bash
python scripts/download_model.py qwen-1.8b-instruct --output ./models
```

### 查看下载进度（后台运行时）
```bash
# Windows
type scripts\download.log

# Linux/macOS
tail -f scripts/download.log
```

### 停止下载
```bash
# Windows
taskkill /F /IM python.exe

# Linux/macOS
pkill -f download_model.py
```

## 断点续传

脚本支持断点续传，如果下载中断，重新运行相同命令即可继续下载。

## 使用下载的模型

下载完成后，模型会保存到 `~/.termgenius/models/` 目录。

### 方法 1: 环境变量（推荐）
```bash
# Windows (PowerShell)
$env:TERMGENIUS_MODEL = "$HOME\.termgenius\models\qwen1_5-1_8b-chat-q4_0.gguf"

# Linux/macOS
export TERMGENIUS_MODEL=~/.termgenius/models/qwen1_5-1_8b-chat-q4_0.gguf
```

### 方法 2: 修改配置文件
编辑 `~/.termgenius/config.toml`:
```toml
model_path = "~/.termgenius/models/qwen1_5-1_8b-chat-q4_0.gguf"
```

## 常见问题

### 下载速度慢
尝试切换下载源：
```bash
python scripts/download_model.py qwen-1.8b-instruct --source modelscope
```

### 下载失败
1. 检查网络连接
2. 尝试其他下载源
3. 增加重试次数：`--retries 5`

### 磁盘空间不足
- Qwen 1.8B 需要约 1.2 GB 空间
- DeepSeek 1.3B 需要约 900 MB 空间

## 技术细节

- **协议**: HTTPS
- **分块大小**: 8 KB
- **超时时间**: 30 秒
- **自动重试**: 3 次（可配置）
- **断点续传**: 支持（HTTP Range）
