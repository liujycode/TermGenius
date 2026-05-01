#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
TermGenius 模型下载脚本

支持：
- 多镜像源（HuggingFace / ModelScope / HF-Mirror）
- 断点续传
- 进度显示
- 自动重试
"""

import os
import sys
import argparse
import hashlib
from pathlib import Path
from urllib.request import urlopen, Request
from urllib.error import URLError, HTTPError
import time

# 修复 Windows 控制台编码问题
if sys.platform == "win32":
    import io
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')

# 模型配置
MODELS = {
    "qwen-1.8b": {
        "filename": "qwen-1_8b-chat-q4_0.gguf",
        "size": "1.1 GB",
        "sources": {
            "hf": "https://huggingface.co/Qwen/Qwen-1_8B-Chat-GGUF/resolve/main/qwen-1_8b-chat-q4_0.gguf",
            "hf-mirror": "https://hf-mirror.com/Qwen/Qwen-1_8B-Chat-GGUF/resolve/main/qwen-1_8b-chat-q4_0.gguf",
            "modelscope": "https://www.modelscope.cn/models/qwen/Qwen-1_8B-Chat-GGUF/resolve/master/qwen-1_8b-chat-q4_0.gguf",
        }
    },
    "qwen-1.8b-instruct": {
        "filename": "qwen1_5-1_8b-chat-q4_0.gguf",
        "size": "1.1 GB",
        "sources": {
            "hf": "https://huggingface.co/Qwen/Qwen1.5-1.8B-Chat-GGUF/resolve/main/qwen1_5-1_8b-chat-q4_0.gguf",
            "hf-mirror": "https://hf-mirror.com/Qwen/Qwen1.5-1.8B-Chat-GGUF/resolve/main/qwen1_5-1_8b-chat-q4_0.gguf",
            "modelscope": "https://www.modelscope.cn/models/qwen/Qwen1.5-1.8B-Chat-GGUF/resolve/master/qwen1_5-1_8b-chat-q4_0.gguf",
        }
    },
    "deepseek-1.3b": {
        "filename": "deepseek-coder-1.3b-instruct-q4_0.gguf",
        "size": "800 MB",
        "sources": {
            "hf": "https://huggingface.co/TheBloke/deepseek-coder-1.3b-instruct-GGUF/resolve/main/deepseek-coder-1.3b-instruct.Q4_0.gguf",
            "hf-mirror": "https://hf-mirror.com/TheBloke/deepseek-coder-1.3b-instruct-GGUF/resolve/main/deepseek-coder-1.3b-instruct.Q4_0.gguf",
        }
    }
}

class DownloadProgress:
    """下载进度显示"""

    def __init__(self, total_size):
        self.total_size = total_size
        self.downloaded = 0
        self.start_time = time.time()
        self.last_update = 0

    def update(self, chunk_size):
        self.downloaded += chunk_size
        current_time = time.time()

        # 每 0.5 秒更新一次显示
        if current_time - self.last_update < 0.5:
            return

        self.last_update = current_time

        # 计算进度
        if self.total_size > 0:
            percent = (self.downloaded / self.total_size) * 100
        else:
            percent = 0

        # 计算速度
        elapsed = current_time - self.start_time
        if elapsed > 0:
            speed = self.downloaded / elapsed / 1024 / 1024  # MB/s
        else:
            speed = 0

        # 计算剩余时间
        if speed > 0 and self.total_size > 0:
            remaining = (self.total_size - self.downloaded) / (speed * 1024 * 1024)
            eta = f"{int(remaining // 60)}m {int(remaining % 60)}s"
        else:
            eta = "未知"

        # 显示进度条
        bar_length = 40
        filled = int(bar_length * self.downloaded / self.total_size) if self.total_size > 0 else 0
        bar = "█" * filled + "░" * (bar_length - filled)

        # 格式化大小
        downloaded_mb = self.downloaded / 1024 / 1024
        total_mb = self.total_size / 1024 / 1024 if self.total_size > 0 else 0

        # 输出进度
        sys.stdout.write(f"\r[{bar}] {percent:.1f}% | {downloaded_mb:.1f}/{total_mb:.1f} MB | {speed:.2f} MB/s | ETA: {eta}")
        sys.stdout.flush()

    def finish(self):
        elapsed = time.time() - self.start_time
        avg_speed = self.downloaded / elapsed / 1024 / 1024 if elapsed > 0 else 0
        print(f"\n✅ 下载完成！总用时: {int(elapsed // 60)}m {int(elapsed % 60)}s，平均速度: {avg_speed:.2f} MB/s")


def download_file(url, output_path, resume=True):
    """
    下载文件，支持断点续传

    Args:
        url: 下载链接
        output_path: 输出路径
        resume: 是否支持断点续传
    """
    # 检查已下载的大小
    downloaded_size = 0
    if resume and output_path.exists():
        downloaded_size = output_path.stat().st_size
        print(f"📂 发现已下载 {downloaded_size / 1024 / 1024:.1f} MB，继续下载...")

    # 构建请求
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
    }

    if downloaded_size > 0:
        headers["Range"] = f"bytes={downloaded_size}-"

    request = Request(url, headers=headers)

    try:
        # 发起请求
        response = urlopen(request, timeout=30)

        # 获取文件总大小
        if "Content-Length" in response.headers:
            total_size = int(response.headers["Content-Length"])
            if downloaded_size > 0:
                total_size += downloaded_size
        else:
            total_size = 0

        print(f"📥 开始下载: {output_path.name}")
        print(f"📊 文件大小: {total_size / 1024 / 1024:.1f} MB")

        # 创建进度显示
        progress = DownloadProgress(total_size)
        progress.downloaded = downloaded_size

        # 下载文件
        mode = "ab" if downloaded_size > 0 else "wb"
        with open(output_path, mode) as f:
            while True:
                chunk = response.read(8192)
                if not chunk:
                    break
                f.write(chunk)
                progress.update(len(chunk))

        progress.finish()
        return True

    except HTTPError as e:
        if e.code == 416:  # Range Not Satisfiable
            print("⚠️  文件已完整下载")
            return True
        else:
            print(f"❌ HTTP 错误: {e.code} - {e.reason}")
            return False
    except URLError as e:
        print(f"❌ 网络错误: {e.reason}")
        return False
    except Exception as e:
        print(f"❌ 下载失败: {e}")
        return False


def download_model(model_name, source="auto", output_dir=None, max_retries=3):
    """
    下载模型

    Args:
        model_name: 模型名称
        source: 下载源 (auto/hf/hf-mirror/modelscope)
        output_dir: 输出目录
        max_retries: 最大重试次数
    """
    if model_name not in MODELS:
        print(f"❌ 未知的模型: {model_name}")
        print(f"可用模型: {', '.join(MODELS.keys())}")
        return False

    model_info = MODELS[model_name]
    filename = model_info["filename"]

    # 确定输出目录
    if output_dir is None:
        home = Path.home()
        output_dir = home / ".termgenius" / "models"
    else:
        output_dir = Path(output_dir)

    output_dir.mkdir(parents=True, exist_ok=True)
    output_path = output_dir / filename

    print(f"🤖 TermGenius 模型下载器")
    print(f"📦 模型: {model_name}")
    print(f"📁 输出: {output_path}")
    print(f"💾 大小: {model_info['size']}")
    print()

    # 确定下载源
    sources = model_info["sources"]
    if source == "auto":
        # 自动选择：优先国内镜像
        source_list = ["hf-mirror", "modelscope", "hf"]
    else:
        if source not in sources:
            print(f"❌ 不支持的下载源: {source}")
            print(f"可用源: {', '.join(sources.keys())}")
            return False
        source_list = [source]

    # 尝试下载
    for attempt in range(max_retries):
        for src in source_list:
            if src not in sources:
                continue

            url = sources[src]
            print(f"🌐 尝试下载源: {src} (尝试 {attempt + 1}/{max_retries})")

            success = download_file(url, output_path, resume=True)

            if success:
                print(f"\n✅ 模型下载成功！")
                print(f"📁 路径: {output_path}")
                print(f"\n💡 使用方法:")
                print(f"   export TERMGENIUS_MODEL={output_path}")
                print(f"   tg \"列出文件\"")
                return True

            print(f"⚠️  从 {src} 下载失败，尝试下一个源...")
            time.sleep(2)

    print(f"\n❌ 下载失败，已尝试 {max_retries} 次")
    return False


def main():
    parser = argparse.ArgumentParser(
        description="TermGenius 模型下载器",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  # 下载 Qwen 1.8B 模型（自动选择最快源）
  python download_model.py qwen-1.8b

  # 指定下载源
  python download_model.py qwen-1.8b --source hf-mirror

  # 指定输出目录
  python download_model.py qwen-1.8b --output ./models

可用模型:
  - qwen-1.8b: Qwen 1.8B Chat (1.1 GB)
  - deepseek-1.3b: DeepSeek Coder 1.3B (800 MB)

可用下载源:
  - auto: 自动选择（推荐）
  - hf: HuggingFace 官方
  - hf-mirror: HuggingFace 镜像（国内快）
  - modelscope: ModelScope（国内快）
        """
    )

    parser.add_argument("model", choices=list(MODELS.keys()), help="模型名称")
    parser.add_argument("--source", default="auto", help="下载源 (auto/hf/hf-mirror/modelscope)")
    parser.add_argument("--output", help="输出目录（默认: ~/.termgenius/models）")
    parser.add_argument("--retries", type=int, default=3, help="最大重试次数")

    args = parser.parse_args()

    success = download_model(
        args.model,
        source=args.source,
        output_dir=args.output,
        max_retries=args.retries
    )

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
