#!/bin/bash
# TermGenius 模型后台下载脚本 (Linux/macOS)

set -e

echo "========================================"
echo "  TermGenius 模型后台下载"
echo "========================================"
echo

# 检查 Python
if ! command -v python3 &> /dev/null; then
    echo "[错误] 未找到 Python3，请先安装"
    exit 1
fi

# 获取脚本目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_FILE="$SCRIPT_DIR/download.log"

echo "[信息] 开始后台下载模型..."
echo "[信息] 日志文件: $LOG_FILE"
echo

# 后台运行下载脚本
nohup python3 "$SCRIPT_DIR/download_model.py" qwen-1.8b > "$LOG_FILE" 2>&1 &
PID=$!

echo "[完成] 下载已在后台启动 (PID: $PID)"
echo
echo "查看下载进度:"
echo "  tail -f $LOG_FILE"
echo
echo "停止下载:"
echo "  kill $PID"
echo
