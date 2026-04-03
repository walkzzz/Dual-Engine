/// Dual-Engine 核心错误类型定义
/// 
/// 使用 thiserror 提供统一的错误处理

use thiserror::Error;

/// 引擎核心错误
#[derive(Error, Debug, Clone, PartialEq)]
pub enum EngineError {
    #[error("引擎未找到：{0}")]
    NotFound(String),
    
    #[error("引擎初始化失败：{0}")]
    InitializationFailed(String),
    
    #[error("引擎执行失败：{0}")]
    ExecutionFailed(String),
    
    #[error("引擎销毁失败：{0}")]
    DestroyFailed(String),
    
    #[error("引擎状态异常：{0}")]
    InvalidState(String),
    
    #[error("引擎已销毁")]
    EngineDestroyed,
}

/// 配置错误
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConfigError {
    #[error("配置文件未找到：{0}")]
    NotFound(String),
    
    #[error("配置解析失败：{0}")]
    ParseError(String),
    
    #[error("配置验证失败：{0}")]
    ValidationError(String),
    
    #[error("缺少必要配置：{0}")]
    MissingRequired(String),
    
    #[error("API Key 未设置：{0}")]
    MissingApiKey(String),
    
    #[error("无效的配置值：{key}={value}")]
    InvalidValue { key: String, value: String },
}

/// API 调用错误
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ApiError {
    #[error("API 调用失败：{0}")]
    RequestFailed(String),
    
    #[error("API 响应解析失败：{0}")]
    ParseError(String),
    
    #[error("API 认证失败：{0}")]
    AuthenticationFailed(String),
    
    #[error("API 速率限制：{0}")]
    RateLimited(String),
    
    #[error("API 超时：{0}秒后超时")]
    Timeout(u64),
    
    #[error("API 服务不可用：{0}")]
    ServiceUnavailable(String),
    
    #[error("无效响应状态码：{status}")]
    InvalidStatus { status: u16 },
}

/// 输入验证错误
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    #[error("输入为空")]
    EmptyInput,
    
    #[error("输入超长：{length} 字符，最大允许 {max}")]
    InputTooLong { length: usize, max: usize },
    
    #[error("包含非法字符：{0}")]
    InvalidCharacters(String),
    
    #[error("潜在注入攻击：{0}")]
    PotentialInjection(String),
    
    #[error("编码无效：{0}")]
    InvalidEncoding(String),
}

/// 资源管理错误
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ResourceError {
    #[error("资源不足：{0}")]
    Insufficient(String),
    
    #[error("资源泄漏：{0}")]
    Leak(String),
    
    #[error("资源超限：{resource} 使用 {used}/{limit}")]
    ExceededLimit { resource: String, used: String, limit: String },
    
    #[error("资源竞争：{0}")]
    Contention(String),
}

/// 统一结果类型
pub type Result<T, E = DualEngineError> = std::result::Result<T, E>;

/// 双引擎统一错误类型
#[derive(Error, Debug)]
pub enum DualEngineError {
    #[error("引擎错误：{0}")]
    Engine(#[from] EngineError),
    
    #[error("配置错误：{0}")]
    Config(#[from] ConfigError),
    
    #[error("API 错误：{0}")]
    Api(#[from] ApiError),
    
    #[error("验证错误：{0}")]
    Validation(#[from] ValidationError),
    
    #[error("资源错误：{0}")]
    Resource(#[from] ResourceError),
    
    #[error("IO 错误：{0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON 错误：{0}")]
    Json(#[from] serde_json::Error),
    
    #[error("其他错误：{0}")]
    Other(String),
}

impl Clone for DualEngineError {
    fn clone(&self) -> Self {
        match self {
            DualEngineError::Engine(e) => DualEngineError::Engine(e.clone()),
            DualEngineError::Config(e) => DualEngineError::Config(e.clone()),
            DualEngineError::Api(e) => DualEngineError::Api(e.clone()),
            DualEngineError::Validation(e) => DualEngineError::Validation(e.clone()),
            DualEngineError::Resource(e) => DualEngineError::Resource(e.clone()),
            DualEngineError::Io(e) => DualEngineError::Io(std::io::Error::new(e.kind(), e.to_string())),
            DualEngineError::Json(e) => DualEngineError::Other(format!("JSON error: {}", e)),
            DualEngineError::Other(msg) => DualEngineError::Other(msg.clone()),
        }
    }
}

impl From<&str> for DualEngineError {
    fn from(s: &str) -> Self {
        DualEngineError::Other(s.to_string())
    }
}

impl From<String> for DualEngineError {
    fn from(s: String) -> Self {
        DualEngineError::Other(s)
    }
}

// 便捷构造函数
impl EngineError {
    pub fn not_found<S: Into<String>>(engine: S) -> Self {
        EngineError::NotFound(engine.into())
    }
    
    pub fn init_failed<S: Into<String>>(reason: S) -> Self {
        EngineError::InitializationFailed(reason.into())
    }
}

impl ConfigError {
    pub fn missing_api_key<S: Into<String>>(provider: S) -> Self {
        ConfigError::MissingApiKey(provider.into())
    }
}

impl ApiError {
    pub fn timeout(secs: u64) -> Self {
        ApiError::Timeout(secs)
    }
    
    pub fn rate_limited<S: Into<String>>(msg: S) -> Self {
        ApiError::RateLimited(msg.into())
    }
}

impl ValidationError {
    pub fn empty() -> Self {
        ValidationError::EmptyInput
    }
    
    pub fn too_long(length: usize, max: usize) -> Self {
        ValidationError::InputTooLong { length, max }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_error_display() {
        let err = EngineError::not_found("opencode");
        assert_eq!(err.to_string(), "引擎未找到：opencode");
    }

    #[test]
    fn test_config_error_display() {
        let err = ConfigError::missing_api_key("moonshot");
        assert_eq!(err.to_string(), "API Key 未设置：moonshot");
    }

    #[test]
    fn test_api_error_display() {
        let err = ApiError::timeout(120);
        assert_eq!(err.to_string(), "API 超时：120 秒后超时");
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::too_long(200, 100);
        assert_eq!(err.to_string(), "输入超长：200 字符，最大允许 100");
    }

    #[test]
    fn test_error_from_conversion() {
        let err: DualEngineError = "custom error".into();
        match err {
            DualEngineError::Other(msg) => assert_eq!(msg, "custom error"),
            _ => panic!("Wrong error type"),
        }
    }
}