use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod error;
pub mod validator;
pub mod rate_limiter;
pub mod audit;

pub use error::*;
pub use validator::*;
pub use rate_limiter::*;
pub use audit::*;

/// 引擎状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EngineStatus {
    Uninitialized,
    Initializing,
    Ready,
    Busy,
    Idle,
    Error(String),
    Destroyed,
}

/// 引擎配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EngineConfig {
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub model: Option<String>,
    pub max_tokens: Option<u32>,
    pub timeout_secs: Option<u64>,
}

/// 资源使用情况
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    pub memory_mb: f64,
    pub cpu_percent: f64,
    pub active_connections: usize,
    pub last_active: Option<std::time::Instant>,
}

/// 引擎类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineType {
    OpenCode,
    Claude,
    MoonShot,
    DashScope,
    Groq,
}

impl std::fmt::Display for EngineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineType::OpenCode => write!(f, "opencode"),
            EngineType::Claude => write!(f, "claude"),
            EngineType::MoonShot => write!(f, "moonshot"),
            EngineType::DashScope => write!(f, "dashscope"),
            EngineType::Groq => write!(f, "groq"),
        }
    }
}

/// Engine trait (re-export from engine-core)
pub use async_trait::async_trait;

#[async_trait]
pub trait Engine: Send + Sync {
    fn engine_type(&self) -> EngineType;
    fn name(&self) -> &str;
    async fn initialize(&mut self, config: EngineConfig) -> Result<(), DualEngineError>;
    async fn execute(&self, request: EngineRequest) -> Result<EngineResponse, DualEngineError>;
    async fn destroy(&mut self) -> Result<(), DualEngineError>;
    fn status(&self) -> EngineStatus;
    fn is_available(&self) -> bool {
        matches!(self.status(), EngineStatus::Ready)
    }
    fn resource_usage(&self) -> ResourceUsage;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
    #[serde(default)]
    pub tool_results: Vec<ToolResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_call_id: String,
    pub content: String,
    #[serde(default)]
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineRequest {
    pub messages: Vec<Message>,
    pub tools: Vec<Tool>,
    #[serde(default)]
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineResponse {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub finish_reason: FinishReason,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    EndTurn,
    ToolUse,
    MaxTokens,
    Stop,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}