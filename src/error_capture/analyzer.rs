//! 错误分析器

use std::collections::HashMap;

/// 错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    /// 命令未找到
    CommandNotFound,
    /// 权限拒绝
    PermissionDenied,
    /// 文件未找到
    FileNotFound,
    /// 语法错误
    SyntaxError,
    /// 网络错误
    NetworkError,
    /// 其他错误
    Other,
}

/// 错误分析器
pub struct ErrorAnalyzer {
    os: String,
}

impl ErrorAnalyzer {
    /// 创建新的错误分析器
    pub fn new(os: String) -> Self {
        Self { os }
    }

    /// 分析错误信息
    pub fn analyze(&self, error_output: &str) -> (ErrorType, String) {
        let error_lower = error_output.to_lowercase();

        // 根据操作系统和错误信息判断错误类型
        let error_type = if self.is_command_not_found(&error_lower) {
            ErrorType::CommandNotFound
        } else if self.is_permission_denied(&error_lower) {
            ErrorType::PermissionDenied
        } else if self.is_file_not_found(&error_lower) {
            ErrorType::FileNotFound
        } else if self.is_syntax_error(&error_lower) {
            ErrorType::SyntaxError
        } else if self.is_network_error(&error_lower) {
            ErrorType::NetworkError
        } else {
            ErrorType::Other
        };

        let suggestion = self.get_suggestion(&error_type, error_output);

        (error_type, suggestion)
    }

    /// 判断是否为命令未找到错误
    fn is_command_not_found(&self, error: &str) -> bool {
        match self.os.as_str() {
            "Windows" => {
                error.contains("不是内部或外部命令") ||
                error.contains("is not recognized") ||
                error.contains("not found")
            }
            _ => {
                error.contains("command not found") ||
                error.contains("not found") ||
                error.contains("no such command")
            }
        }
    }

    /// 判断是否为权限拒绝错误
    fn is_permission_denied(&self, error: &str) -> bool {
        error.contains("permission denied") ||
        error.contains("access denied") ||
        error.contains("拒绝访问") ||
        error.contains("access is denied") ||
        error.contains("operation not permitted")
    }

    /// 判断是否为文件未找到错误
    fn is_file_not_found(&self, error: &str) -> bool {
        error.contains("no such file") ||
        error.contains("cannot find") ||
        error.contains("找不到文件") ||
        error.contains("file not found") ||
        error.contains("does not exist")
    }

    /// 判断是否为语法错误
    fn is_syntax_error(&self, error: &str) -> bool {
        error.contains("syntax error") ||
        error.contains("invalid syntax") ||
        error.contains("语法错误") ||
        error.contains("unexpected token") ||
        error.contains("parse error")
    }

    /// 判断是否为网络错误
    fn is_network_error(&self, error: &str) -> bool {
        error.contains("network") ||
        error.contains("connection") ||
        error.contains("timeout") ||
        error.contains("unreachable") ||
        error.contains("dns")
    }

    /// 获取修复建议
    fn get_suggestion(&self, error_type: &ErrorType, error_output: &str) -> String {
        match error_type {
            ErrorType::CommandNotFound => {
                match self.os.as_str() {
                    "Windows" => {
                        "建议：\n1. 检查命令拼写是否正确\n2. 确认软件是否已安装\n3. 检查 PATH 环境变量".to_string()
                    }
                    _ => {
                        "建议：\n1. 检查命令拼写是否正确\n2. 使用 'which <命令>' 查找命令位置\n3. 可能需要安装软件包（apt/yum/brew install）".to_string()
                    }
                }
            }
            ErrorType::PermissionDenied => {
                match self.os.as_str() {
                    "Windows" => {
                        "建议：\n1. 以管理员身份运行命令\n2. 检查文件/目录权限\n3. 关闭可能占用文件的程序".to_string()
                    }
                    _ => {
                        "建议：\n1. 使用 sudo 运行命令\n2. 检查文件权限（ls -l）\n3. 使用 chmod 修改权限".to_string()
                    }
                }
            }
            ErrorType::FileNotFound => {
                "建议：\n1. 检查文件路径是否正确\n2. 使用绝对路径\n3. 确认文件是否存在（ls/dir）".to_string()
            }
            ErrorType::SyntaxError => {
                "建议：\n1. 检查命令参数是否正确\n2. 检查引号是否匹配\n3. 查看命令帮助（--help）".to_string()
            }
            ErrorType::NetworkError => {
                "建议：\n1. 检查网络连接\n2. 检查防火墙设置\n3. 尝试使用代理".to_string()
            }
            ErrorType::Other => {
                format!("错误信息：\n{}\n\n建议：请检查命令和参数是否正确", error_output)
            }
        }
    }

    /// 提取可能的命令名称
    pub fn extract_command_name(&self, error_output: &str) -> Option<String> {
        // 尝试从错误信息中提取命令名称
        let patterns = vec![
            r"'([^']+)'",  // 单引号
            r#""([^"]+)""#,  // 双引号
            r"command '([^']+)'",
            r"bash: ([^:]+):",
        ];

        for pattern in patterns {
            if let Some(caps) = regex::Regex::new(pattern).ok()?.captures(error_output) {
                if let Some(cmd) = caps.get(1) {
                    return Some(cmd.as_str().to_string());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_not_found() {
        let analyzer = ErrorAnalyzer::new("Linux".to_string());
        let (error_type, _) = analyzer.analyze("bash: xyz: command not found");
        assert_eq!(error_type, ErrorType::CommandNotFound);
    }

    #[test]
    fn test_permission_denied() {
        let analyzer = ErrorAnalyzer::new("Linux".to_string());
        let (error_type, _) = analyzer.analyze("permission denied");
        assert_eq!(error_type, ErrorType::PermissionDenied);
    }

    #[test]
    fn test_file_not_found() {
        let analyzer = ErrorAnalyzer::new("Linux".to_string());
        let (error_type, _) = analyzer.analyze("no such file or directory");
        assert_eq!(error_type, ErrorType::FileNotFound);
    }
}
