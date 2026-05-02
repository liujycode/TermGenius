# TermGenius 本地离线 AI 终端助手 - 完善版项目方案

# 一、项目概述

## 1.1 项目定位与边界

TermGenius 是100%本地离线 AI 终端助手，聚焦程序员终端操作与 AI 编程辅助，核心解决"记不住复杂命令、报错难排查、云端泄露代码、配置繁琐"四大痛点，实现自然语言转命令、命令自动纠错、终端内 AI 编程、零配置开箱即用，兼容全平台终端。**边界说明**：暂不支持大型模型训练、复杂可视化报表，聚焦终端实操核心场景，优先保障轻量化与稳定性。

核心关键词：本地离线、AI 编程辅助、全平台、零配置、轻量高效

## 1.2 核心价值与目标受众

- 核心价值：刚需高频（提效50%）、差异化（本地防泄露）、零配置易用、可扩展（联动桌面/插件），兼顾个人开发者效率与企业数据安全。

- 目标受众：核心（后端、运维、DevOps、终端相关前端）；次要（学生、入门程序员）；企业端（有代码隐私保护、私有化部署需求的研发团队）。

## 1.3 GitHub 目标与核心指标

- 短期（1-2个月）：MVP 上线，500+ Star，安装成功率≥95%，核心功能准确率≥90%，冲击 GitHub Trending。

- 中期（3-6个月）：完善功能，2000+ Star，上架 Homebrew/Chocolatey/AUR 主流渠道，用户留存率≥60%，搭建初步社区。

- 长期（6-12个月）：行业标杆，5000+ Star，支持多语言编程辅助，推出桌面版，企业用户≥50家，形成完善社区生态。

# 二、市场分析与竞争优势

## 2.1 市场痛点与空白

- 痛点：传统工具（tldr、zsh-autosuggestions）无 AI 能力；云端 AI 终端（Cursor Terminal）依赖网络、有泄露风险、收费；本地工具功能单一、配置繁琐，无编程辅助能力。

- 空白：缺乏"本地离线+AI 能力""终端+编程一体化""零配置轻量跨平台"的终端助手，现有竞品无法同时满足安全、高效、易用三大核心需求。

## 2.2 核心竞争优势

- 安全优势：100%本地离线运行，代码/操作日志不上传，支持企业私有化部署，完全符合企业代码安全规范。

- 体验优势：零配置一键安装（无依赖），全终端无缝兼容，本地推理响应≤1秒，不干扰终端原有操作习惯。

- 功能优势：终端操作与 AI 编程一体化，覆盖命令生成、纠错、脚本开发全场景，贴合程序员日常实操需求，无冗余功能。

- 技术优势：单二进制轻量（约10MB），跨平台适配性强，后期扩展成本低，可快速迭代新功能。

## 2.3 竞品对比表

| 产品 | 本地离线 | AI能力 | 编程辅助 | 零配置 | 跨平台 | 价格 | 核心劣势 |
|------|---------|--------|---------|--------|--------|------|---------|
| **TermGenius** | ✅ | ✅ | ✅ | ✅ | ✅ | 免费 | 新项目，生态待建 |
| GitHub Copilot CLI | ❌ 云端 | ✅ | ⚠️ 仅命令 | ✅ | ✅ | $10/月 | 需联网，有泄露风险 |
| Warp Terminal | ❌ 云端 | ✅ | ❌ | ✅ | ⚠️ 仅macOS | 部分免费 | 不支持Windows/Linux |
| tldr | ✅ | ❌ | ❌ | ✅ | ✅ | 免费 | 无AI，仅静态文档 |
| thefuck | ✅ | ❌ | ❌ | ⚠️ 需配置 | ✅ | 免费 | 规则固定，无AI推理 |
| Fig (已被收购) | ❌ 云端 | ⚠️ 弱 | ❌ | ✅ | ⚠️ 仅macOS | 免费 | 已停止维护 |

**差异化总结**：TermGenius 是唯一同时满足"本地离线+强AI能力+编程辅助+零配置+全平台"的终端助手，填补市场空白。

# 三、产品功能规划

**功能优先级说明**：P0（必做，MVP 核心）、P1（迭代重点）、P2（生态扩展，可选），确保开发资源聚焦核心需求。

## 3.1 MVP 版本（7-10天落地，P0级功能）

### 3.1.1 自然语言与 Shell 命令双向转换（P0）

- 离线运行，生成命令附带简洁参数解释
- 支持一键执行/手动修改
- 精准适配 Windows PowerShell 与 Linux/macOS bash 语法差异，避免跨系统命令失效
- 支持命令反向解释（输入命令，AI 解释作用）

### 3.1.2 命令报错自动修复（P0）

- 精准捕获终端错误日志（通过 $? 和 stderr）
- 快速识别报错类型（拼写、参数、权限、路径等）
- 生成可直接执行的修复命令
- 附带极简错误原因提示，降低用户排查成本

### 3.1.3 本地 LLM 引擎集成（P0）

- 首次启动自动下载适配当前系统的轻量模型（约1-2GB，可选精简版≤1GB）
- 支持模型缓存，后续启动秒级加载
- 可手动选择模型版本，适配不同性能设备
- **模型管理策略**：
  - 提供国内镜像源（ModelScope）+ 官方源（Hugging Face）双通道
  - 支持离线安装包（预打包模型）
  - 断点续传 + SHA256 校验和验证
  - 模型版本管理：支持回退到旧版本
  - 自动检测网络环境，优先选择最快源

### 3.1.4 全平台终端兼容（P0）

- 无缝适配 Linux（Ubuntu 18.04+、CentOS 7+）、macOS（10.15+）、Windows（10+）三大系统
- 支持 bash、zsh、fish、PowerShell 主流终端
- 无需修改终端配置，安装后自动关联唤醒（唤醒指令可自定义）
- **终端集成技术方案**：
  - Linux/macOS: shell alias + wrapper 脚本（在 ~/.bashrc / ~/.zshrc 中注入）
  - Windows: PowerShell Profile 注入（$PROFILE 自动加载）
  - 错误捕获：通过 $? 和 $PIPESTATUS 检测命令退出码，捕获 stderr

### 3.1.5 零配置一键安装（P0）

- 提供跨平台统一安装脚本
- 自动完成安装、模型下载、终端关联
- 支持安装校验（确保功能正常）
- 一键卸载（清理无残留，含模型文件）

## 3.2 迭代版本（1-2个月，P1级功能）

### 3.2.1 终端内 AI 编程辅助（P1）

- 优先支持 Python、Go、Shell 等终端高频编程语言
- 脚本生成可直接指定运行环境（如 Python 3.8+）
- 代码纠错/补全贴合终端实操场景（如批量运维脚本、简单工具脚本）
- 避免冗余代码，提升执行效率

### 3.2.2 命令历史学习与个性化推荐（P1）

- 本地加密存储操作历史（仅本地留存）
- 仅学习高频有效命令（过滤无效输入）
- 推荐优先级按"使用频率+场景相关性"排序
- 支持手动清理历史、关闭推荐功能
- 兼顾个性化与隐私保护

### 3.2.3 命令批量执行与脚本管理（P1）

- 支持批量命令组合编排（可视化拖拽可选，降低门槛）
- 脚本可保存为本地文件（适配对应系统格式：.sh/.ps1）
- 支持分类管理、一键执行与导出
- 适配终端批量运维、重复操作场景

### 3.2.4 基础终端 UI 优化（P1）

- 彩色区分命令（绿色）、解释（灰色）、错误提示（红色）
- 支持浅色/深色自动切换（适配终端系统主题）
- 操作指引简洁不干扰正常终端使用
- 可自定义提示样式

## 3.3 生态版本（3-6个月，P2级功能）

### 3.3.1 桌面版（Tauri 开发，P2）

- 图形化界面简洁轻量化（不占用过多系统资源）
- 支持自然语言输入、脚本管理、操作历史查询
- 与终端实时联动，可一键同步操作（如桌面生成脚本，终端直接执行）
- 适配不熟悉终端指令的轻度用户

### 3.3.2 VS Code 插件联动（P2）

- 实现"代码编辑-终端执行"无缝衔接
- 在 VS Code 中生成的脚本可直接同步至终端运行
- 终端错误反馈同步至插件（无需切换窗口）
- 提升编程与终端操作协同效率

### 3.3.3 多模型支持（P2）

- 兼容 Qwen、DeepSeek-Coder、Llama 3 等轻量模型
- 支持手动切换
- 可根据需求选择"速度优先"（精简模型）或"精度优先"（标准模型）模式
- 适配不同性能设备（如轻薄本/服务器）

### 3.3.4 企业版私有化部署（P2）

- 支持局域网内批量部署
- 提供用户权限分级管理（管理员/普通用户）
- 操作日志审计（便于合规检查）
- 自定义模型部署（企业私有模型）
- 适配企业代码安全规范与批量管理需求

### 3.3.5 社区插件生态（P2）

- 开放简单易懂的插件接口（降低开发门槛）
- 支持用户开发命令模板、语言扩展等轻量插件
- 提供插件审核与共享渠道（GitHub 社区）
- 丰富功能生态，降低维护成本

## 3.4 核心交互流程

### 3.4.1 命令生成流程

```
1. 用户输入：tg "列出所有大于100MB的文件"
2. AI 解析 → 生成命令：find . -type f -size +100M
3. 显示预览 + 解释：
   [绿色] find . -type f -size +100M
   [灰色] 在当前目录递归查找所有大于100MB的文件
4. 用户确认：
   [Enter 执行] [Ctrl+E 编辑] [Esc 取消]
```

### 3.4.2 错误修复流程

```
1. 命令执行失败 → 自动捕获错误输出
2. 后台分析 → 识别错误类型（权限/语法/路径）
3. 弹出提示：
   [红色] 错误：权限不足，无法访问 /root 目录
   [绿色] 建议：sudo find . -type f -size +100M
4. 用户选择：
   [Enter 执行修复] [Ctrl+E 手动调整] [Esc 取消]
```

### 3.4.3 编程辅助流程

```
1. 用户输入：tg code "写一个Python脚本批量重命名文件"
2. AI 生成代码 + 注释
3. 显示预览：
   [绿色] 代码块（带语法高亮）
   [灰色] 功能说明 + 使用方法
4. 用户操作：
   [Enter 保存到文件] [Ctrl+C 复制] [Ctrl+R 运行] [Esc 取消]
```

### 3.4.4 唤醒方式

- **命令前缀**：`tg "自然语言描述"` 或 `tg code "编程需求"`
- **快捷键**（可选）：Ctrl+Space 唤醒交互模式
- **自动触发**：命令执行失败后自动弹出修复建议

# 四、技术架构与选型

## 4.1 整体架构（5层解耦）

```
┌─────────────────────────────────────────────────────┐
│              部署层（安装/更新/卸载）                  │
│  Shell/PowerShell 脚本 + 自动化配置 + 模型管理        │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│              交互层（终端/桌面版交互）                 │
│  Ratatui (终端UI) + Tauri (桌面版) + VS Code 插件     │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│              核心功能层（业务实现）                    │
│  命令生成 + 错误修复 + 编程辅助 + 历史学习 + 脚本管理  │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│              AI 引擎层（本地 LLM 推理）                │
│  llama.cpp + 模型加载 + 推理优化 + 上下文管理         │
└─────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────┐
│              基础层（跨平台适配 + 存储）               │
│  跨平台 API + 本地加密存储 + 配置管理 + 日志系统      │
└─────────────────────────────────────────────────────┘
```

## 4.2 核心技术选型

| 技术栈 | 选型 | 选型理由 |
|--------|------|---------|
| **开发语言** | Rust | 轻量、高性能、跨平台单二进制编译、无依赖，贴合零配置、快速启动的功能定位 |
| **LLM 引擎** | llama.cpp | 轻量、跨平台、可嵌入 Rust 项目、支持离线推理、响应速度快 |
| **Rust 绑定** | llama-cpp-rs 或 llama-cpp-2 | 成熟的 Rust binding，支持 FFI 调用，社区活跃 |
| **本地模型** | Qwen 1.8B / DeepSeek-Coder 1.3B | 轻量、代码能力强、开源免费、模型体积小（4-bit 量化后 <2GB） |
| **终端交互** | Ratatui | Rust 库、轻量、可自定义 UI、支持彩色输出与主题切换 |
| **桌面版** | Tauri | 轻量（比 Electron 小 90%）、Rust 生态、跨平台、安全 |
| **部署/存储** | Shell/PowerShell 脚本 | 零配置安装、适配跨平台部署需求 |
| **加密存储** | AES-256-GCM | 本地数据加密、保护用户历史与脚本、贴合隐私安全需求 |
| **模型加速** | Metal (macOS) / CUDA (Nvidia) / Vulkan (通用) | 可选 GPU 加速，提升推理速度 |

## 4.3 核心技术实现路径

### 4.3.1 终端集成方案

**Linux/macOS**：
```bash
# 在 ~/.bashrc / ~/.zshrc 中注入
alias tg='termgenius'
function tg_error_handler() {
    if [ $? -ne 0 ]; then
        termgenius fix --last-error
    fi
}
trap tg_error_handler ERR
```

**Windows PowerShell**：
```powershell
# 在 $PROFILE 中注入
function tg { termgenius.exe $args }
$ErrorActionPreference = "Continue"
# 错误捕获通过 $Error[0] 获取
```

**错误捕获机制**：
- 通过 `$?` 和 `$PIPESTATUS` 检测命令退出码
- 捕获 stderr 输出（重定向到临时文件）
- 解析错误类型（正则匹配常见错误模式）

### 4.3.2 LLM 集成方案

**技术选型**：
- 使用 `llama-cpp-rs` 或 `llama-cpp-2`（Rust binding）
- 通过 FFI 调用 llama.cpp 的 C API
- 支持 4-bit / 8-bit 量化模型

**推理优化**：
- 4-bit 量化（GGUF 格式）：内存占用减少 75%
- Metal/CUDA 加速（可选）：推理速度提升 3-5 倍
- 上下文缓存：复用历史对话，减少重复计算
- 批处理推理：多个请求合并处理

**模型加载流程**：
```rust
// 伪代码示例
let model = LlamaModel::load_from_file("qwen-1.8b-q4.gguf")?;
let context = model.new_context(ContextParams {
    n_ctx: 2048,  // 上下文长度
    n_batch: 512, // 批处理大小
    use_gpu: true, // 启用 GPU 加速
})?;
```

### 4.3.3 跨平台适配

**文件路径**：
- 使用 Rust 的 `std::path::PathBuf`（自动适配 Windows/Unix 路径）
- 配置文件位置：
  - Linux/macOS: `~/.config/termgenius/`
  - Windows: `%APPDATA%\termgenius\`

**系统命令差异**：
- 维护命令映射表（如 `ls` → `dir` on Windows）
- 根据 `std::env::consts::OS` 动态生成命令

## 4.4 性能基准（Baseline）

| 配置 | 模型 | 推理时间 | 内存占用 | CPU 占用 | 备注 |
|------|------|---------|---------|---------|------|
| 低配（8GB RAM, i5） | Qwen 1.8B 4-bit | <2s | ~500MB | 30-50% | 无 GPU 加速 |
| 标准（16GB RAM, i7） | DeepSeek 1.3B 4-bit | <1s | ~800MB | 20-40% | 无 GPU 加速 |
| 高配（32GB RAM, GPU） | Qwen 7B 4-bit | <0.5s | ~2GB | GPU 加速 | Metal/CUDA |
| 服务器（64GB RAM, GPU） | DeepSeek 6.7B 8-bit | <0.3s | ~4GB | GPU 加速 | 企业版推荐 |

**性能优化目标**：
- 冷启动时间：<3s（首次加载模型）
- 热启动时间：<0.5s（模型已缓存）
- 推理响应时间：<1s（标准配置）
- 内存占用：<1GB（常驻内存）

# 五、开发计划与里程碑

## 5.1 分阶段开发

### 第一阶段（7-10天）：MVP 开发

**目标**：聚焦 P0 级功能，完成核心功能开发

**任务清单**：
- Day 1-2：项目架构搭建 + llama.cpp 集成测试
- Day 3-4：命令生成与解释功能（自然语言 ↔ Shell）
- Day 5-6：错误捕获与修复功能
- Day 7：终端集成（bash/zsh/PowerShell）
- Day 8：跨平台测试（Linux/macOS/Windows）
- Day 9：安装脚本开发 + 模型下载逻辑
- Day 10：Bug 修复 + 文档编写

**验收标准**：
- 安装成功率 ≥95%（三大系统测试）
- 命令生成准确率 ≥90%（100 个测试用例）
- 错误修复成功率 ≥85%（50 个常见错误）
- 推理响应时间 <2s（标准配置）

### 第二阶段（1-2个月）：迭代版本

**目标**：聚焦 P1 级功能，优化用户体验

**任务清单**：
- Week 1-2：编程辅助功能（Python/Go/Shell 代码生成）
- Week 3-4：命令历史学习与推荐
- Week 5-6：批量执行与脚本管理
- Week 7-8：终端 UI 优化 + 性能优化

**验收标准**：
- 代码生成准确率 ≥85%（50 个测试用例）
- 推荐命令准确率 ≥80%（基于历史学习）
- 用户留存率 ≥60%（7 日留存）
- GitHub Star ≥500

### 第三阶段（3-6个月）：生态版本

**目标**：聚焦 P2 级功能，搭建生态

**任务清单**：
- Month 3：桌面版开发（Tauri）
- Month 4：VS Code 插件开发
- Month 5：多模型支持 + 企业版功能
- Month 6：社区插件生态搭建

**验收标准**：
- 桌面版下载量 ≥1000
- VS Code 插件安装量 ≥500
- 企业用户 ≥10 家
- GitHub Star ≥2000

## 5.2 核心里程碑

| 里程碑 | 时间 | 核心功能 | 关键指标 |
|--------|------|---------|---------|
| **里程碑1** | 10天 | MVP 上线，支持 P0 级核心离线功能 | 安装成功率≥95%，命令准确率≥90% |
| **里程碑2** | 2个月 | 迭代版上线，支持 P1 级功能 | GitHub 500+ Star，上架1-2个主流渠道 |
| **里程碑3** | 4个月 | 桌面版+VS Code 插件上线，支持多模型 | GitHub 2000+ Star，用户留存率≥60% |
| **里程碑4** | 6个月 | 企业版上线，开放社区插件接口 | GitHub 5000+ Star，企业用户≥50家 |

## 5.3 质量保证体系

### 5.3.1 自动化测试

**单元测试**：
- 核心逻辑覆盖率 ≥80%
- 使用 Rust 的 `cargo test` 框架
- 测试用例：命令解析、错误识别、模型推理

**集成测试**：
- 跨平台测试矩阵：3 系统 × 4 终端 = 12 种组合
- 自动化测试脚本（GitHub Actions）
- 测试覆盖：安装、命令生成、错误修复、卸载

**性能测试**：
- 推理时间监控（每次 commit 自动测试）
- 内存占用监控（长时间运行测试）
- 性能回归检测（对比历史版本）

### 5.3.2 用户测试

**种子用户测试**：
- 每个版本邀请 20-30 名种子用户（后端/运维）
- 收集反馈问卷（功能满意度、Bug 报告、改进建议）
- 快速迭代修复（48 小时内响应）

**Beta 测试**：
- 公开 Beta 版本（GitHub Releases）
- 收集 Issue 和 PR
- 每周发布更新日志

### 5.3.3 CI/CD 流程

```yaml
# GitHub Actions 示例
on: [push, pull_request]
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - name: Checkout
      - name: Build
      - name: Run tests
      - name: Performance benchmark
      - name: Upload artifacts
```

# 六、GitHub 上线与推广

## 6.1 上线准备

### 6.1.1 仓库结构

```
termgenius/
├── README.md              # 项目介绍（含演示动图）
├── LICENSE                # MIT 许可证
├── CONTRIBUTING.md        # 贡献指南
├── CODE_OF_CONDUCT.md     # 行为准则
├── CHANGELOG.md           # 更新日志
├── docs/                  # 文档目录
│   ├── installation.md    # 安装指南
│   ├── usage.md           # 使用教程
│   ├── faq.md             # 常见问题
│   └── architecture.md    # 架构设计
├── src/                   # 源代码
├── tests/                 # 测试代码
├── scripts/               # 安装脚本
│   ├── install.sh         # Linux/macOS 安装
│   └── install.ps1        # Windows 安装
└── .github/
    ├── workflows/         # CI/CD
    └── ISSUE_TEMPLATE/    # Issue 模板
```

### 6.1.2 专业 README

**必备内容**：
- 项目 Logo + 一句话介绍
- 核心功能演示动图（GIF，15秒内）
- 一键安装命令（复制即用）
- 快速开始（3 步上手）
- 功能特性（对比竞品）
- 常见问题 FAQ
- 贡献指南链接
- Star 历史图表（后期添加）

**示例结构**：
```markdown
# TermGenius - 本地离线 AI 终端助手

> 100% 本地运行，零配置，让终端操作更智能

[演示动图]

## 一键安装
```bash
curl -fsSL https://termgenius.dev/install.sh | bash
```

## 核心功能
- 🚀 自然语言转命令
- 🔧 命令报错自动修复
- 💻 终端内 AI 编程
- 🔒 100% 本地离线
- ⚡ 零配置开箱即用
```

### 6.1.3 许可证与合规

**主项目许可证**：MIT（最宽松，利于传播）

**第三方依赖许可证检查**：
- llama.cpp: MIT
- Rust 标准库: MIT/Apache 2.0
- Ratatui: MIT
- 确保所有依赖兼容 MIT

## 6.2 分阶段推广

### 6.2.1 初期推广（0-1个月，目标 500 Star）

**渠道**：
- GitHub：发布 Release，添加 Topics（ai, terminal, cli, rust, local-llm）
- 掘金：发布技术文章《我用 Rust 做了一个本地 AI 终端助手》
- V2EX：/go/programmer 板块分享
- Hacker News：Show HN 帖子
- Reddit：r/rust, r/commandline, r/selfhosted
- 程序员社群：Rust 中文社区、后端技术群

**内容策略**：
- 强调"本地离线"差异化优势
- 展示实际使用场景（运维、开发）
- 提供详细安装教程
- 及时回复评论和问题

### 6.2.2 中期推广（1-3个月，目标 2000 Star）

**渠道**：
- 技术博主/UP 主：邀请体验并分享（B站、YouTube）
- 主流安装渠道：上架 Homebrew、Chocolatey、AUR
- 技术会议：Rust Conf、开源峰会（线上分享）
- 开源周刊：Rust Weekly、Console Weekly

**内容策略**：
- 发布深度技术文章（架构设计、性能优化）
- 制作视频教程（5-10 分钟）
- 收集用户案例（真实使用场景）
- 定期更新（每周 1-2 次迭代）

### 6.2.3 长期推广（3-6个月，目标 5000 Star）

**渠道**：
- 企业端推广：技术社区、开发者大会
- 开源贡献活动：Hacktoberfest、GSoC
- 媒体报道：InfoQ、开源中国、OSCHINA
- 合作伙伴：与 IDE、终端工具合作

**内容策略**：
- 企业案例分享（隐私保护场景）
- 社区插件生态建设
- 举办线上 Meetup
- 发布年度报告（用户数据、功能迭代）

## 6.3 高 Star 技巧

### 6.3.1 开箱即用

- 一键安装脚本（无需手动配置）
- 自动检测系统环境
- 安装失败自动回滚
- 提供离线安装包

### 6.3.2 演示直观

- 15 秒核心功能短视频
- GIF 动图（命令生成、错误修复）
- 实际使用场景截图
- 对比演示（传统方式 vs TermGenius）

### 6.3.3 高频更新

- 上线初期每周 1-2 次迭代
- 及时修复用户反馈的 Bug
- 发布详细的 CHANGELOG
- 在 README 中展示最近更新

### 6.3.4 关键词优化

- GitHub Topics：ai, terminal, cli, rust, local-llm, offline, developer-tools
- README 关键词：AI 终端、本地离线、零配置、命令助手
- 贴合热点：AI、隐私保护、开发者工具

### 6.3.5 及时互动

- 24 小时内回复 Issue
- 48 小时内审核 PR
- 感谢贡献者（在 README 中列出）
- 收集用户需求（建立 Roadmap）

## 6.4 开源社区规范

### 6.4.1 许可证说明

**主项目**：MIT License
- 允许商业使用、修改、分发
- 仅需保留版权声明
- 无担保责任

**第三方依赖许可证兼容性**：
- llama.cpp: MIT ✅
- Rust 标准库: MIT/Apache 2.0 ✅
- Ratatui: MIT ✅
- 所有依赖均兼容 MIT

### 6.4.2 贡献指南（CONTRIBUTING.md）

**代码规范**：
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 检查代码质量
- 遵循 Rust 命名规范
- 添加必要的注释和文档

**提交规范**：
```
<type>(<scope>): <subject>

<body>

<footer>
```

类型：
- feat: 新功能
- fix: Bug 修复
- docs: 文档更新
- style: 代码格式
- refactor: 重构
- test: 测试
- chore: 构建/工具

**测试要求**：
- 新功能必须包含单元测试
- 测试覆盖率不低于 80%
- 所有测试必须通过

**PR 流程**：
1. Fork 仓库
2. 创建功能分支（`git checkout -b feature/xxx`）
3. 提交代码（遵循提交规范）
4. 推送到 Fork 仓库
5. 创建 Pull Request
6. 等待 CI 检查通过
7. 等待 Maintainer 审核

### 6.4.3 Issue 模板

**Bug Report**：
```markdown
## Bug 描述
简洁描述遇到的问题

## 复现步骤
1. 执行命令 xxx
2. 输入 xxx
3. 看到错误 xxx

## 预期行为
应该显示 xxx

## 实际行为
实际显示 xxx

## 环境信息
- OS: [e.g. Ubuntu 22.04]
- Terminal: [e.g. bash 5.1]
- TermGenius 版本: [e.g. v0.1.0]

## 日志输出
```
粘贴相关日志
```
```

**Feature Request**：
```markdown
## 功能描述
简洁描述期望的功能

## 使用场景
为什么需要这个功能？解决什么问题？

## 期望行为
期望如何使用这个功能

## 替代方案
是否有其他解决方案？

## 补充信息
其他相关信息
```

**Question**：
```markdown
## 问题描述
简洁描述你的问题

## 已尝试的方法
已经尝试了哪些方法？

## 环境信息
- OS: [e.g. Ubuntu 22.04]
- Terminal: [e.g. bash 5.1]
- TermGenius 版本: [e.g. v0.1.0]
```

## 6.5 文档与内容规划

### 6.5.1 用户文档

**快速开始（5分钟上手）**：
```markdown
# 快速开始

## 1. 安装
```bash
curl -fsSL https://termgenius.dev/install.sh | bash
```

## 2. 验证安装
```bash
tg --version
```

## 3. 第一个命令
```bash
tg "列出当前目录所有文件"
```

## 4. 错误修复
```bash
# 执行一个错误命令
rm /root/test.txt
# TermGenius 自动弹出修复建议
```
```

**常见问题 FAQ**：
- 如何更新 TermGenius？
- 如何切换模型？
- 如何卸载？
- 推理速度慢怎么办？
- 如何清理历史记录？
- 支持哪些终端？
- 如何自定义配置？

**命令示例库（100+ 实用场景）**：
- 文件操作：查找、删除、重命名、压缩
- 系统管理：进程、服务、用户、权限
- 网络操作：curl、wget、ssh、scp
- Git 操作：提交、分支、合并、回滚
- Docker 操作：容器、镜像、网络、卷
- 数据处理：grep、awk、sed、jq

**故障排查指南**：
- 安装失败：网络问题、权限问题、依赖问题
- 推理失败：模型损坏、内存不足、配置错误
- 终端集成失败：Shell 配置冲突、权限问题

### 6.5.2 开发者文档

**架构设计文档**：
- 整体架构图
- 模块划分
- 数据流图
- 技术选型说明

**API 参考手册**：
- 核心 API 文档
- 插件接口文档
- 配置文件格式
- 命令行参数

**插件开发指南**：
```markdown
# 插件开发指南

## 插件结构
```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, input: &str) -> Result<String>;
}
```

## 示例插件
```rust
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }
    fn version(&self) -> &str { "0.1.0" }
    fn execute(&self, input: &str) -> Result<String> {
        Ok(format!("Processed: {}", input))
    }
}
```
```

**贡献者手册**：
- 如何搭建开发环境
- 如何运行测试
- 如何调试
- 如何提交 PR

### 6.5.3 内容营销

**技术博客（3-5 篇深度文章）**：
1. 《TermGenius 架构设计：如何用 Rust 构建本地 AI 终端助手》
2. 《性能优化实战：将 LLM 推理时间从 5s 降到 1s》
3. 《跨平台终端集成：适配 bash/zsh/PowerShell 的技术方案》
4. 《本地 AI 模型选型：Qwen vs DeepSeek vs Llama》
5. 《隐私保护设计：如何实现 100% 本地离线运行》

**视频教程（B站/YouTube）**：
1. 5 分钟快速上手（安装 + 基础使用）
2. 10 分钟进阶教程（编程辅助 + 脚本管理）
3. 15 分钟架构解析（技术实现原理）
4. 实战案例：用 TermGenius 提升运维效率

**案例分享**：
- 后端开发者：日常开发提效 50%
- 运维工程师：批量运维脚本生成
- 学生：学习 Linux 命令的好帮手
- 企业：代码隐私保护的最佳实践

# 七、风险与应对

## 7.1 技术风险

| 风险 | 影响 | 概率 | 应对方案 |
|------|------|------|---------|
| 推理速度慢 | 用户体验差 | 中 | 1. 选用 4-bit 量化轻量模型<br>2. 启用 GPU 加速（Metal/CUDA）<br>3. 优化推理代码，控制响应时间在 1s 内<br>4. 提供模型版本选择（精简版/标准版） |
| 跨平台兼容性差 | 安装失败率高 | 中 | 1. 针对三大系统、四大终端做专项适配测试<br>2. 建立自动化测试矩阵（12 种组合）<br>3. 提供详细的故障排查文档<br>4. 收集用户反馈快速修复 |
| 模型下载失败 | 首次安装失败 | 高 | 1. 提供国内镜像源（ModelScope）+ 官方源双通道<br>2. 支持断点续传 + SHA256 校验<br>3. 提供离线安装包（预打包模型）<br>4. 安装失败自动回滚 |
| 内存占用过高 | 低配设备无法运行 | 低 | 1. 提供精简版模型（<1GB）<br>2. 优化内存管理（及时释放）<br>3. 支持模型卸载（不使用时释放内存）<br>4. 明确最低配置要求（4GB RAM） |

## 7.2 市场风险

| 风险 | 影响 | 概率 | 应对方案 |
|------|------|------|---------|
| 用户反馈差 | Star 增长慢 | 中 | 1. 上线前邀请 20-30 名种子用户测试<br>2. 收集反馈并快速优化<br>3. 每周更新功能/修复 Bug<br>4. 及时回复 Issue 和评论 |
| 竞品出现 | 市场份额被抢占 | 中 | 1. 强化"本地离线+零配置"差异化优势<br>2. 快速迭代，保持功能领先<br>3. 建立社区生态壁垒<br>4. 加大推广力度，抢占市场先机 |
| 推广效果差 | 用户增长慢 | 低 | 1. 多渠道推广（GitHub/掘金/V2EX/HN）<br>2. 联系技术博主/UP 主合作<br>3. 上架主流安装渠道<br>4. 制作高质量演示视频 |
| 企业用户拓展难 | 商业化受阻 | 低 | 1. 提供企业版试用（免费 30 天）<br>2. 收集企业需求，定制化开发<br>3. 参加企业技术会议推广<br>4. 建立企业案例库 |

## 7.3 开发风险

| 风险 | 影响 | 概率 | 应对方案 |
|------|------|------|---------|
| 进度滞后 | 延期上线 | 中 | 1. 细化每日开发任务，明确优先级（先 P0 再 P1）<br>2. 每周复盘开发进度，及时调整<br>3. 遇到技术难题，借助 Rust/llama.cpp 社区资源<br>4. 优先实现核心功能，非核心功能延后迭代 |
| 技术难题 | 功能无法实现 | 低 | 1. 提前进行技术预研（llama.cpp 集成测试）<br>2. 参考成熟开源项目（如 llama.cpp 示例）<br>3. 在 Rust 社区寻求帮助<br>4. 必要时调整技术方案 |
| 人力不足 | 开发效率低 | 低 | 1. 聚焦核心功能，避免功能蔓延<br>2. 复用成熟开源库（减少重复造轮子）<br>3. 吸引社区贡献者参与开发<br>4. 必要时寻找合作伙伴 |
| Bug 过多 | 用户体验差 | 中 | 1. 建立完善的测试体系（单元测试+集成测试）<br>2. 种子用户内测，提前发现问题<br>3. 快速响应用户反馈（48 小时内修复）<br>4. 建立 Bug 优先级机制（P0/P1/P2） |

## 7.4 隐私安全风险

| 风险 | 影响 | 概率 | 应对方案 |
|------|------|------|---------|
| 命令历史泄露 | 用户隐私泄露 | 低 | 1. 本地数据采用 AES-256-GCM 加密存储<br>2. 密钥基于设备指纹派生（不上传）<br>3. 支持手动清理历史记录<br>4. 在 README 中明确隐私保护说明 |
| 敏感信息泄露 | 密码/Token 泄露 | 中 | 1. 自动检测并脱敏敏感信息（密码、API key、token）<br>2. 命令历史过滤敏感模式（如 `export TOKEN=xxx`）<br>3. 提供"隐私模式"（不记录历史）<br>4. 企业版增加敏感命令拦截 |
| 模型版权问题 | 法律风险 | 低 | 1. 选用开源免费、版权清晰的模型（Qwen/DeepSeek）<br>2. 在文档中明确模型来源和许可证<br>3. 避免使用有版权争议的模型<br>4. 企业版支持自定义模型（用户自负责任） |
| 代码注入攻击 | 安全漏洞 | 低 | 1. 对 AI 生成的命令进行安全检查<br>2. 提供"预览模式"（dry-run，不直接执行）<br>3. 危险命令（rm -rf /）需二次确认<br>4. 沙箱执行（可选，企业版） |

## 7.5 安全加固措施

### 7.5.1 敏感信息过滤

**自动检测规则**：
```rust
// 伪代码示例
fn is_sensitive(cmd: &str) -> bool {
    let patterns = [
        r"password\s*=\s*\S+",
        r"token\s*=\s*\S+",
        r"api[_-]?key\s*=\s*\S+",
        r"secret\s*=\s*\S+",
        r"export\s+\w*TOKEN\w*=",
    ];
    patterns.iter().any(|p| Regex::new(p).unwrap().is_match(cmd))
}

fn sanitize(cmd: &str) -> String {
    // 将敏感信息替换为 ***
    cmd.replace(/* 敏感模式 */, "***")
}
```

**用户控制**：
- 支持自定义敏感模式（正则表达式）
- 支持关闭历史记录功能
- 支持"隐私模式"（临时会话，不保存）

### 7.5.2 加密方案

**加密算法**：AES-256-GCM
- 密钥长度：256 位
- 认证加密：防止篡改
- 随机 IV：每次加密使用不同 IV

**密钥管理**：
```rust
// 伪代码示例
fn derive_key() -> [u8; 32] {
    // 基于设备指纹派生密钥
    let device_id = get_device_id(); // CPU ID + MAC 地址
    let salt = b"termgenius-v1";
    pbkdf2::derive(device_id, salt, 100_000)
}
```

**存储格式**：
```json
{
  "version": 1,
  "encrypted_data": "base64...",
  "iv": "base64...",
  "tag": "base64..."
}
```

### 7.5.3 企业版安全功能

**权限管理**：
- RBAC（基于角色的访问控制）
- 支持 LDAP/SSO 集成
- 用户分组管理

**操作审计**：
- 记录所有命令执行日志
- 记录 AI 生成的命令
- 支持日志导出（CSV/JSON）
- 符合合规要求（SOC 2、ISO 27001）

**敏感命令拦截**：
```rust
// 伪代码示例
fn is_dangerous(cmd: &str) -> bool {
    let dangerous_patterns = [
        r"rm\s+-rf\s+/",
        r"dd\s+if=.*of=/dev/sd",
        r"mkfs\.",
        r":(){ :|:& };:", // Fork bomb
    ];
    dangerous_patterns.iter().any(|p| Regex::new(p).unwrap().is_match(cmd))
}
```

**沙箱执行（可选）**：
- 使用 Docker 容器隔离执行
- 限制文件系统访问
- 限制网络访问
- 限制资源使用（CPU/内存）

# 八、项目价值与长期规划

## 8.1 核心价值

### 8.1.1 用户价值

**个人开发者**：
- 提效 50%：减少查文档、试错时间
- 降低门槛：新手也能快速上手复杂命令
- 提升体验：自然语言交互，更符合人类思维
- 隐私保护：代码和操作日志不上传云端

**企业用户**：
- 数据安全：100% 本地运行，符合企业安全规范
- 批量部署：支持局域网内统一管理
- 合规审计：操作日志可追溯，满足合规要求
- 降低成本：无需购买云端 AI 服务

### 8.1.2 技术价值

**开源社区**：
- 填补本地离线 AI 终端市场空白
- 提供 Rust + llama.cpp 集成最佳实践
- 推动本地 AI 技术普及
- 建立活跃的开源社区

**个人成长**：
- 提升 Rust 开发能力
- 深入理解 LLM 推理原理
- 积累跨平台开发经验
- 提升技术影响力（GitHub Star、技术文章）

### 8.1.3 商业价值（可选）

**开源版**：
- 免费使用，建立用户基础
- 吸引开发者贡献
- 形成品牌影响力

**企业版**：
- 增值功能：批量部署、权限管理、定制化开发
- 技术支持：7x24 专属支持
- 定价策略：$99/用户/年（或按需定制）
- 收入反哺：20% 收入用于开源社区建设

## 8.2 长期规划（1-2年）

### 8.2.1 产品规划

**Year 1（0-12个月）**：
- Q1：MVP 上线，完成核心功能
- Q2：迭代版上线，优化用户体验
- Q3：桌面版 + VS Code 插件上线
- Q4：企业版上线，拓展企业用户

**Year 2（12-24个月）**：
- Q1：多语言编程辅助（Java、C++、Rust）
- Q2：终端自动化功能（录制回放、定时任务）
- Q3：智能运维功能（故障诊断、性能分析）
- Q4：移动端支持（iOS/Android 终端）

### 8.2.2 社区规划

**社区建设**：
- 建立官方网站（termgenius.dev）
- 搭建社区论坛（Discourse）
- 建立 Discord/Slack 社群
- 定期举办线上 Meetup

**贡献者激励**：
- 贡献者排行榜（在 README 中展示）
- 优秀贡献者奖励（周边、证书）
- 举办贡献活动（Hacktoberfest）
- 提供贡献者专属权益

**社区规范**：
- 建立行为准则（Code of Conduct）
- 设立社区管理员（Maintainer）
- 定期发布社区报告（月度/季度）
- 收集社区反馈（问卷调查）

### 8.2.3 技术规划

**性能优化**：
- 推理速度优化：目标 <0.5s（标准配置）
- 内存占用优化：目标 <500MB（常驻内存）
- 启动速度优化：目标 <1s（热启动）
- 支持更多硬件加速（AMD GPU、Apple Silicon）

**模型优化**：
- 支持更多轻量模型（Phi、Gemma）
- 支持模型微调（Fine-tuning）
- 支持多模型组合（专家模型）
- 支持模型量化优化（2-bit、1-bit）

**功能扩展**：
- 终端操作自动化（录制回放、脚本生成）
- 智能故障排查（日志分析、根因定位）
- 性能分析（命令耗时、资源占用）
- 团队协作（命令分享、脚本库）

### 8.2.4 商业化规划（可选）

**开源版 vs 企业版功能对比**：

| 功能 | 开源版 | 企业版 |
|------|--------|--------|
| 核心 AI 功能 | ✅ | ✅ |
| 命令生成与修复 | ✅ | ✅ |
| 编程辅助 | ✅ | ✅ |
| 本地模型 | ✅ | ✅ + 私有模型 |
| 桌面版 | ✅ | ✅ |
| VS Code 插件 | ✅ | ✅ |
| 批量部署 | ❌ | ✅ |
| 权限管理 | ❌ | ✅ RBAC + SSO |
| 操作审计 | ❌ | ✅ 日志审计 + 合规报告 |
| 敏感命令拦截 | ❌ | ✅ |
| 沙箱执行 | ❌ | ✅ |
| 定制化开发 | ❌ | ✅ |
| 技术支持 | 社区 | 7x24 专属支持 |
| 定价 | 免费 | $99/用户/年 |

**商业化原则**：
- 核心功能永久开源免费
- 企业版仅增加管理/合规功能
- 不影响开源版用户体验
- 收入 20% 反哺开源社区

**收入预测（Year 2）**：
- 企业用户：50 家 × 平均 20 用户 = 1000 用户
- 年收入：1000 × $99 = $99,000
- 反哺社区：$99,000 × 20% = $19,800

**收入用途**：
- 开源社区建设（40%）
- 全职开发者招募（30%）
- 基础设施成本（20%）
- 市场推广（10%）

## 8.3 成功指标

### 8.3.1 短期指标（0-6个月）

| 指标 | 目标 | 衡量方式 |
|------|------|---------|
| GitHub Star | 2000+ | GitHub 统计 |
| 安装量 | 10,000+ | 下载统计 + 安装脚本统计 |
| 用户留存率 | ≥60% | 7 日留存率 |
| 安装成功率 | ≥95% | 安装脚本统计 |
| 核心功能准确率 | ≥90% | 用户反馈 + 测试用例 |
| Issue 响应时间 | <24h | GitHub 统计 |
| PR 合并时间 | <48h | GitHub 统计 |

### 8.3.2 中期指标（6-12个月）

| 指标 | 目标 | 衡量方式 |
|------|------|---------|
| GitHub Star | 5000+ | GitHub 统计 |
| 安装量 | 50,000+ | 下载统计 |
| 月活用户 | 20,000+ | 匿名统计（可选） |
| 企业用户 | 50+ | 企业版注册统计 |
| 社区贡献者 | 100+ | GitHub 统计 |
| 插件数量 | 20+ | 插件仓库统计 |
| 技术文章 | 10+ | 博客/社区统计 |

### 8.3.3 长期指标（12-24个月）

| 指标 | 目标 | 衡量方式 |
|------|------|---------|
| GitHub Star | 10,000+ | GitHub 统计 |
| 安装量 | 200,000+ | 下载统计 |
| 月活用户 | 100,000+ | 匿名统计（可选） |
| 企业用户 | 200+ | 企业版注册统计 |
| 社区贡献者 | 500+ | GitHub 统计 |
| 插件数量 | 100+ | 插件仓库统计 |
| 年收入（可选） | $500,000+ | 企业版收入 |

# 九、附录

## 9.1 技术资源

### 9.1.1 核心技术栈

**Rust 生态**：
- Rust 官方文档：https://doc.rust-lang.org/
- Rust 中文社区：https://rust.cc/
- Cargo 包管理：https://crates.io/
- Rust 异步编程：https://tokio.rs/

**LLM 推理**：
- llama.cpp 仓库：https://github.com/ggerganov/llama.cpp
- llama-cpp-rs：https://github.com/utilityai/llama-cpp-rs
- llama-cpp-2：https://github.com/mdrokz/rust-llama.cpp
- GGUF 模型格式：https://github.com/ggerganov/ggml

**模型资源**：
- Qwen 官方仓库：https://github.com/QwenLM/Qwen
- DeepSeek-Coder：https://github.com/deepseek-ai/DeepSeek-Coder
- Hugging Face：https://huggingface.co/
- ModelScope（国内镜像）：https://modelscope.cn/

**终端 UI**：
- Ratatui：https://github.com/ratatui-org/ratatui
- Crossterm：https://github.com/crossterm-rs/crossterm
- Termion：https://github.com/redox-os/termion

**桌面开发**：
- Tauri 官方文档：https://tauri.app/
- Tauri 中文文档：https://tauri.app/zh-cn/

### 9.1.2 学习资源

**Rust 学习**：
- 《Rust 程序设计语言》：https://kaisery.github.io/trpl-zh-cn/
- 《Rust By Example》：https://doc.rust-lang.org/rust-by-example/
- 《Rust 异步编程》：https://rust-lang.github.io/async-book/

**LLM 学习**：
- 《动手学深度学习》：https://zh.d2l.ai/
- 《大语言模型入门》：https://llm-course.github.io/
- 《Transformer 论文解读》：https://arxiv.org/abs/1706.03762

**终端开发**：
- 《命令行的艺术》：https://github.com/jlevy/the-art-of-command-line
- 《Shell 脚本编程》：https://www.shellscript.sh/

## 9.2 推广资源

### 9.2.1 社区渠道

**中文社区**：
- 掘金：https://juejin.cn/
- V2EX：https://www.v2ex.com/go/programmer
- 思否：https://segmentfault.com/
- CSDN：https://www.csdn.net/
- 开源中国：https://www.oschina.net/
- Rust 中文社区：https://rust.cc/

**英文社区**：
- Hacker News：https://news.ycombinator.com/
- Reddit：r/rust, r/commandline, r/selfhosted
- Dev.to：https://dev.to/
- Lobsters：https://lobste.rs/

**技术周刊**：
- Rust Weekly：https://this-week-in-rust.org/
- Console Weekly：https://console.dev/
- TLDR Newsletter：https://tldr.tech/

### 9.2.2 安装渠道

**Linux**：
- Homebrew：https://brew.sh/
- AUR（Arch User Repository）：https://aur.archlinux.org/
- Snap Store：https://snapcraft.io/
- APT/YUM 仓库（自建）

**macOS**：
- Homebrew：https://brew.sh/
- MacPorts：https://www.macports.org/

**Windows**：
- Chocolatey：https://chocolatey.org/
- Scoop：https://scoop.sh/
- WinGet：https://github.com/microsoft/winget-cli

**通用**：
- GitHub Releases：直接下载二进制文件
- Cargo：`cargo install termgenius`

### 9.2.3 媒体资源

**技术媒体**：
- InfoQ：https://www.infoq.cn/
- 极客时间：https://time.geekbang.org/
- 少数派：https://sspai.com/

**视频平台**：
- B站：https://www.bilibili.com/
- YouTube：https://www.youtube.com/
- 抖音：技术类短视频

**播客**：
- Rust 中文播客：https://rustcc.cn/podcast
- 代码之外：https://beyondcode.fm/

## 9.3 测试资源

### 9.3.1 测试环境

**操作系统**：
- Linux：Ubuntu 18.04/20.04/22.04, CentOS 7/8, Debian 10/11
- macOS：10.15 (Catalina), 11 (Big Sur), 12 (Monterey), 13 (Ventura)
- Windows：10, 11

**终端**：
- Linux/macOS：bash, zsh, fish
- Windows：PowerShell, Windows Terminal, Git Bash

**硬件配置**：
- 低配：8GB RAM, i5 CPU, 无 GPU
- 标准：16GB RAM, i7 CPU, 无 GPU
- 高配：32GB RAM, i9 CPU, Nvidia GPU
- 服务器：64GB RAM, Xeon CPU, Nvidia GPU

### 9.3.2 测试用例

**命令生成测试（100 个用例）**：
- 文件操作：查找、删除、重命名、压缩（20 个）
- 系统管理：进程、服务、用户、权限（20 个）
- 网络操作：curl、wget、ssh、scp（20 个）
- Git 操作：提交、分支、合并、回滚（20 个）
- Docker 操作：容器、镜像、网络、卷（20 个）

**错误修复测试（50 个用例）**：
- 拼写错误：命令名拼写错误（10 个）
- 参数错误：参数缺失或错误（10 个）
- 权限错误：权限不足（10 个）
- 路径错误：文件/目录不存在（10 个）
- 语法错误：Shell 语法错误（10 个）

**编程辅助测试（50 个用例）**：
- Python 脚本：文件处理、数据分析、Web 爬虫（20 个）
- Go 程序：CLI 工具、Web 服务、并发处理（15 个）
- Shell 脚本：批量运维、自动化部署（15 个）

### 9.3.3 性能测试

**推理性能测试**：
```bash
# 测试脚本示例
for i in {1..100}; do
    time tg "列出当前目录所有文件"
done | awk '{sum+=$2} END {print "平均时间:", sum/100, "秒"}'
```

**内存占用测试**：
```bash
# 监控内存占用
while true; do
    ps aux | grep termgenius | awk '{print $6}'
    sleep 1
done
```

**并发测试**：
```bash
# 并发执行测试
for i in {1..10}; do
    tg "测试命令 $i" &
done
wait
```

## 9.4 参考项目

### 9.4.1 类似项目

**AI 终端助手**：
- GitHub Copilot CLI：https://github.com/github/copilot-cli
- Warp Terminal：https://www.warp.dev/
- Fig：https://fig.io/（已被收购）

**传统终端工具**：
- tldr：https://github.com/tldr-pages/tldr
- thefuck：https://github.com/nvbn/thefuck
- zsh-autosuggestions：https://github.com/zsh-users/zsh-autosuggestions

**本地 LLM 项目**：
- ollama：https://github.com/ollama/ollama
- LocalAI：https://github.com/mudler/LocalAI
- llama.cpp：https://github.com/ggerganov/llama.cpp

### 9.4.2 技术参考

**Rust CLI 项目**：
- ripgrep：https://github.com/BurntSushi/ripgrep
- bat：https://github.com/sharkdp/bat
- fd：https://github.com/sharkdp/fd
- exa：https://github.com/ogham/exa

**Rust + LLM 项目**：
- llm-chain：https://github.com/sobelio/llm-chain
- candle：https://github.com/huggingface/candle

## 9.5 常见问题（FAQ）

### 9.5.1 安装相关

**Q: 如何安装 TermGenius？**
A: 执行一键安装命令：
```bash
curl -fsSL https://termgenius.dev/install.sh | bash
```

**Q: 安装失败怎么办？**
A: 
1. 检查网络连接
2. 查看错误日志：`cat ~/.termgenius/install.log`
3. 尝试离线安装包
4. 在 GitHub 提 Issue

**Q: 如何卸载？**
A: 执行卸载命令：
```bash
termgenius uninstall
```

### 9.5.2 使用相关

**Q: 如何生成命令？**
A: 使用 `tg` 命令：
```bash
tg "列出所有大于100MB的文件"
```

**Q: 如何修复错误？**
A: 命令执行失败后，TermGenius 会自动弹出修复建议，按 Enter 执行。

**Q: 如何切换模型？**
A: 使用配置命令：
```bash
tg config --model qwen-1.8b
```

**Q: 推理速度慢怎么办？**
A:
1. 切换到精简版模型
2. 启用 GPU 加速（如果有 GPU）
3. 关闭其他占用资源的程序

### 9.5.3 隐私相关

**Q: 数据会上传到云端吗？**
A: 不会。TermGenius 100% 本地运行，所有数据仅存储在本地。

**Q: 如何清理历史记录？**
A: 使用清理命令：
```bash
tg history clear
```

**Q: 如何关闭历史记录功能？**
A: 修改配置文件：
```bash
tg config --history-enabled false
```

---

## 结语

TermGenius 致力于打造最好用的本地离线 AI 终端助手，让终端操作更智能、更高效、更安全。我们相信，通过开源社区的力量，TermGenius 将成为程序员终端工具箱中不可或缺的一员。

**欢迎加入我们**：
- GitHub：https://github.com/termgenius/termgenius
- 官网：https://termgenius.dev
- 社区：https://discord.gg/termgenius
- 邮箱：hello@termgenius.dev

**让我们一起，让终端更智能！**

---

> 文档版本：v1.0  
> 最后更新：2026-05-01  
> 作者：TermGenius Team  
> 许可证：CC BY-SA 4.0

