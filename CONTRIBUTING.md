# Contributing to TermGenius

感谢你对 TermGenius 的关注！我们欢迎所有形式的贡献。

## 行为准则

参与本项目即表示你同意遵守我们的 [行为准则](CODE_OF_CONDUCT.md)。

## 如何贡献

### 报告 Bug

如果你发现了 Bug，请：

1. 检查 [Issues](https://github.com/liujycode/TermGenius/issues) 是否已有相同问题
2. 如果没有，创建新 Issue，使用 Bug Report 模板
3. 提供详细的复现步骤、环境信息、错误日志

### 提出功能建议

1. 检查 [Issues](https://github.com/liujycode/TermGenius/issues) 是否已有相同建议
2. 创建新 Issue，使用 Feature Request 模板
3. 描述功能的使用场景和预期行为

### 提交代码

#### 开发环境搭建

1. **安装 Rust**（1.75+）:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **克隆仓库**:
```bash
git clone https://github.com/liujycode/TermGenius.git
cd TermGenius
```

3. **构建项目**:
```bash
cargo build
```

4. **运行测试**:
```bash
cargo test
```

#### 代码规范

**格式化**:
```bash
cargo fmt
```

**Lint 检查**:
```bash
cargo clippy -- -D warnings
```

**命名规范**:
- 变量/函数：`snake_case`
- 类型/Trait：`PascalCase`
- 常量：`SCREAMING_SNAKE_CASE`
- 模块：`snake_case`

**注释规范**:
```rust
/// 函数功能简述
///
/// # Arguments
/// * `param` - 参数说明
///
/// # Returns
/// 返回值说明
///
/// # Examples
/// ```
/// let result = function(param);
/// ```
pub fn function(param: Type) -> ReturnType {
    // 实现
}
```

#### 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**类型（type）**:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

**示例**:
```
feat(cli): 添加命令历史学习功能

- 实现本地历史记录存储
- 添加频率统计算法
- 支持个性化推荐

Closes #123
```

#### Pull Request 流程

1. **Fork 仓库**

2. **创建功能分支**:
```bash
git checkout -b feature/your-feature-name
```

3. **开发并测试**:
```bash
# 编写代码
# 运行测试
cargo test
# 格式化代码
cargo fmt
# Lint 检查
cargo clippy
```

4. **提交代码**:
```bash
git add .
git commit -m "feat: your feature description"
```

5. **推送到 Fork 仓库**:
```bash
git push origin feature/your-feature-name
```

6. **创建 Pull Request**:
   - 填写 PR 模板
   - 关联相关 Issue
   - 等待 CI 检查通过
   - 等待 Maintainer 审核

#### 测试要求

- 新功能必须包含单元测试
- 测试覆盖率不低于 80%
- 所有测试必须通过

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 查看测试覆盖率
cargo tarpaulin --out Html
```

## 开发指南

### 项目结构

```
TermGenius/
├── src/
│   ├── main.rs           # 入口
│   ├── cli/              # 命令行接口
│   ├── llm/              # LLM 引擎
│   ├── terminal/         # 终端集成
│   ├── ui/               # 终端 UI
│   └── utils/            # 工具函数
├── tests/                # 集成测试
├── docs/                 # 文档
└── scripts/              # 脚本
```

### 调试技巧

**启用日志**:
```bash
RUST_LOG=debug cargo run
```

**使用 lldb/gdb**:
```bash
rust-lldb target/debug/termgenius
```

## 社区

- GitHub Discussions: [讨论区](https://github.com/liujycode/TermGenius/discussions)
- Issues: [问题追踪](https://github.com/liujycode/TermGenius/issues)

## 许可证

贡献的代码将采用 [MIT License](LICENSE) 开源。

---

再次感谢你的贡献！🎉
