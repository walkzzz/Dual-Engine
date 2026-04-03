/// 输入验证模块
/// 
/// 提供用户输入验证和清理功能

use crate::error::{ValidationError, Result};
use std::collections::HashSet;

/// 输入验证器
pub struct InputValidator {
    max_length: usize,
    allow_patterns: HashSet<char>,
    deny_patterns: HashSet<&'static str>,
}

impl Default for InputValidator {
    fn default() -> Self {
        Self {
            max_length: 10000,
            allow_patterns: HashSet::new(),
            deny_patterns: Self::default_deny_patterns(),
        }
    }
}

impl InputValidator {
    /// 创建新的验证器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置最大长度
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = max;
        self
    }

    /// 默认拒绝的危险模式
    fn default_deny_patterns() -> HashSet<&'static str> {
        let mut patterns = HashSet::new();
        // 潜在的命令注入
        patterns.insert("```bash");
        patterns.insert("```sh");
        patterns.insert("$(");
        patterns.insert("`");
        // 潜在的 XSS
        patterns.insert("<script>");
        patterns.insert("</script>");
        patterns.insert("javascript:");
        // 潜在的路径遍历
        patterns.insert("../");
        patterns.insert("..\\");
        patterns
    }

    /// 验证输入
    pub fn validate(&self, input: &str) -> Result<String> {
        // 检查空输入
        if input.trim().is_empty() {
            return Err(ValidationError::empty().into());
        }

        // 检查长度
        if input.len() > self.max_length {
            return Err(ValidationError::too_long(input.len(), self.max_length).into());
        }

        // 检查危险模式
        for pattern in &self.deny_patterns {
            if input.contains(pattern) {
                return Err(ValidationError::PotentialInjection(
                    format!("检测到危险模式：{}", pattern)
                ).into());
            }
        }

        // 检查编码问题 (字符串已经是 &str，所以肯定是 UTF-8)
        // 这个检查可以省略，因为 &str 保证是有效的 UTF-8

        Ok(input.to_string())
    }

    /// 清理输入（移除危险字符但保留有效内容）
    pub fn sanitize(&self, input: &str) -> String {
        let mut result = input.to_string();
        
        // 移除危险模式
        for pattern in &self.deny_patterns {
            result = result.replace(pattern, "");
        }
        
        // 移除控制字符（保留换行和制表符）
        result = result
            .chars()
            .filter(|c| c.is_ascii_graphic() || c.is_whitespace() || *c == '\n' || *c == '\t')
            .collect();
        
        result.trim().to_string()
    }

    /// 快速验证（仅基础检查）
    pub fn quick_validate(input: &str) -> Result<()> {
        if input.trim().is_empty() {
            return Err(ValidationError::empty().into());
        }
        if input.len() > 10000 {
            return Err(ValidationError::too_long(input.len(), 10000).into());
        }
        Ok(())
    }
}

/// 验证 Prompt 输入
pub fn validate_prompt(prompt: &str) -> Result<String> {
    InputValidator::new().validate(prompt)
}

/// 验证并清理 Prompt
pub fn sanitize_prompt(prompt: &str) -> String {
    InputValidator::new().sanitize(prompt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_input() {
        let validator = InputValidator::new();
        assert!(matches!(validator.validate(""), Err(crate::error::DualEngineError::Validation(ValidationError::EmptyInput))));
    }

    #[test]
    fn test_validate_long_input() {
        let validator = InputValidator::new().with_max_length(10);
        let result = validator.validate("This is a very long input that exceeds the limit");
        assert!(matches!(result, Err(crate::error::DualEngineError::Validation(ValidationError::InputTooLong { .. }))));
    }

    #[test]
    fn test_validate_dangerous_pattern() {
        let validator = InputValidator::new();
        let result = validator.validate("Hello $(whoami)");
        assert!(matches!(result, Err(crate::error::DualEngineError::Validation(ValidationError::PotentialInjection(_)))));
    }

    #[test]
    fn test_validate_valid_input() {
        let validator = InputValidator::new();
        let result = validator.validate("Hello, world! 你好世界");
        assert!(result.is_ok());
    }

    #[test]
    fn test_sanitize_removes_dangerous() {
        let validator = InputValidator::new();
        let sanitized = validator.sanitize("Hello $(whoami) world");
        assert!(!sanitized.contains("$("));
    }

    #[test]
    fn test_quick_validate() {
        assert!(InputValidator::quick_validate("valid input").is_ok());
        assert!(InputValidator::quick_validate("").is_err());
    }
}