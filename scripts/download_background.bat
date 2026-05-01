@echo off
REM TermGenius 模型后台下载脚本 (Windows)

echo ========================================
echo   TermGenius 模型后台下载
echo ========================================
echo.

REM 检查 Python
python --version >nul 2>&1
if errorlevel 1 (
    echo [错误] 未找到 Python，请先安装 Python 3.7+
    echo 下载地址: https://www.python.org/downloads/
    pause
    exit /b 1
)

REM 获取脚本目录
set SCRIPT_DIR=%~dp0
set LOG_FILE=%SCRIPT_DIR%download.log

echo [信息] 开始后台下载模型...
echo [信息] 日志文件: %LOG_FILE%
echo.

REM 后台运行下载脚本
start /B python "%SCRIPT_DIR%download_model.py" qwen-1.8b > "%LOG_FILE%" 2>&1

echo [完成] 下载已在后台启动
echo.
echo 查看下载进度:
echo   type %LOG_FILE%
echo.
echo 或者打开新的命令行窗口查看实时日志:
echo   Get-Content %LOG_FILE% -Wait
echo.
pause
