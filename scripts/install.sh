#!/bin/bash
# TermGenius 安装脚本 - Linux/macOS

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 配置
REPO="liujycode/TermGenius"
INSTALL_DIR="$HOME/.termgenius"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/termgenius"

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  TermGenius 安装程序${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# 检测操作系统
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
    else
        echo -e "${RED}错误: 不支持的操作系统${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓${NC} 检测到操作系统: $OS"
}

# 检测架构
detect_arch() {
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            echo -e "${RED}错误: 不支持的架构 $ARCH${NC}"
            exit 1
            ;;
    esac
    echo -e "${GREEN}✓${NC} 检测到架构: $ARCH"
}

# 创建目录
create_dirs() {
    mkdir -p "$INSTALL_DIR"
    mkdir -p "$BIN_DIR"
    mkdir -p "$CONFIG_DIR"
    echo -e "${GREEN}✓${NC} 创建安装目录"
}

# 下载二进制文件
download_binary() {
    echo -e "${YELLOW}→${NC} 下载 TermGenius..."

    # TODO: 替换为实际的 release URL
    DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/termgenius-$OS-$ARCH"

    if command -v curl &> /dev/null; then
        curl -fsSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/termgenius" || {
            echo -e "${RED}错误: 下载失败${NC}"
            echo -e "${YELLOW}提示: 项目尚未发布二进制文件，请从源码编译${NC}"
            exit 1
        }
    elif command -v wget &> /dev/null; then
        wget -q "$DOWNLOAD_URL" -O "$INSTALL_DIR/termgenius" || {
            echo -e "${RED}错误: 下载失败${NC}"
            exit 1
        }
    else
        echo -e "${RED}错误: 需要 curl 或 wget${NC}"
        exit 1
    fi

    chmod +x "$INSTALL_DIR/termgenius"
    echo -e "${GREEN}✓${NC} 下载完成"
}

# 创建符号链接
create_symlink() {
    ln -sf "$INSTALL_DIR/termgenius" "$BIN_DIR/tg"
    echo -e "${GREEN}✓${NC} 创建符号链接"
}

# 配置 Shell
configure_shell() {
    SHELL_RC=""
    if [[ -n "$BASH_VERSION" ]]; then
        SHELL_RC="$HOME/.bashrc"
    elif [[ -n "$ZSH_VERSION" ]]; then
        SHELL_RC="$HOME/.zshrc"
    fi

    if [[ -n "$SHELL_RC" ]]; then
        # 添加 PATH
        if ! grep -q "$BIN_DIR" "$SHELL_RC"; then
            echo "" >> "$SHELL_RC"
            echo "# TermGenius" >> "$SHELL_RC"
            echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_RC"
            echo -e "${GREEN}✓${NC} 配置 Shell 环境"
        fi
    fi
}

# 下载模型
download_model() {
    echo -e "${YELLOW}→${NC} 下载 AI 模型（约 1.5GB）..."
    echo -e "${YELLOW}提示: 首次运行时会自动下载，也可以现在下载${NC}"

    read -p "是否现在下载模型？(y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # TODO: 实现模型下载
        echo -e "${YELLOW}→${NC} 模型下载功能开发中..."
    else
        echo -e "${YELLOW}→${NC} 跳过模型下载，首次运行时会自动下载"
    fi
}

# 验证安装
verify_installation() {
    if command -v tg &> /dev/null; then
        echo -e "${GREEN}✓${NC} 安装成功！"
        echo ""
        echo -e "${GREEN}使用方法:${NC}"
        echo -e "  tg \"列出所有大于100MB的文件\""
        echo -e "  tg code \"写一个Python脚本\""
        echo -e "  tg --help"
        echo ""
        echo -e "${YELLOW}提示: 请重启终端或运行 'source $SHELL_RC' 使配置生效${NC}"
    else
        echo -e "${RED}错误: 安装失败${NC}"
        exit 1
    fi
}

# 主流程
main() {
    detect_os
    detect_arch
    create_dirs
    download_binary
    create_symlink
    configure_shell
    download_model
    verify_installation
}

main
