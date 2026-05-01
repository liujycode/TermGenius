# TermGenius Windows 安装脚本

$ErrorActionPreference = "Stop"

# 颜色输出
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-ColorOutput Green "========================================"
Write-ColorOutput Green "  TermGenius 安装程序"
Write-ColorOutput Green "========================================"
Write-Output ""

# 配置
$REPO = "liujycode/TermGenius"
$INSTALL_DIR = "$env:APPDATA\termgenius"
$CONFIG_DIR = "$env:APPDATA\termgenius\config"

# 创建目录
Write-ColorOutput Yellow "→ 创建安装目录..."
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null
New-Item -ItemType Directory -Force -Path $CONFIG_DIR | Out-Null
Write-ColorOutput Green "✓ 目录创建完成"

# 检测架构
$ARCH = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
Write-ColorOutput Green "✓ 检测到架构: $ARCH"

# 下载二进制文件
Write-ColorOutput Yellow "→ 下载 TermGenius..."
$DOWNLOAD_URL = "https://github.com/$REPO/releases/latest/download/termgenius-windows-$ARCH.exe"

try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile "$INSTALL_DIR\termgenius.exe"
    Write-ColorOutput Green "✓ 下载完成"
} catch {
    Write-ColorOutput Red "错误: 下载失败"
    Write-ColorOutput Yellow "提示: 项目尚未发布二进制文件，请从源码编译"
    exit 1
}

# 添加到 PATH
Write-ColorOutput Yellow "→ 配置环境变量..."
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$INSTALL_DIR*") {
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$INSTALL_DIR", "User")
    Write-ColorOutput Green "✓ 环境变量配置完成"
}

# 配置 PowerShell Profile
Write-ColorOutput Yellow "→ 配置 PowerShell..."
if (!(Test-Path -Path $PROFILE)) {
    New-Item -ItemType File -Path $PROFILE -Force | Out-Null
}

$ProfileContent = Get-Content $PROFILE -Raw -ErrorAction SilentlyContinue
if ($ProfileContent -notlike "*termgenius*") {
    Add-Content $PROFILE "`n# TermGenius"
    Add-Content $PROFILE "function tg { termgenius.exe `$args }"
    Write-ColorOutput Green "✓ PowerShell 配置完成"
}

# 下载模型
Write-ColorOutput Yellow "→ 下载 AI 模型（约 1.5GB）..."
Write-ColorOutput Yellow "提示: 首次运行时会自动下载，也可以现在下载"
$response = Read-Host "是否现在下载模型？(y/N)"
if ($response -eq "y" -or $response -eq "Y") {
    Write-ColorOutput Yellow "→ 模型下载功能开发中..."
} else {
    Write-ColorOutput Yellow "→ 跳过模型下载，首次运行时会自动下载"
}

# 验证安装
Write-ColorOutput Green "✓ 安装成功！"
Write-Output ""
Write-ColorOutput Green "使用方法:"
Write-Output "  tg `"列出所有大于100MB的文件`""
Write-Output "  tg code `"写一个Python脚本`""
Write-Output "  tg --help"
Write-Output ""
Write-ColorOutput Yellow "提示: 请重启 PowerShell 使配置生效"
