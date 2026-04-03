pub use shared_types::{EngineRequest, EngineResponse, FinishReason, Message, Role, Tool, ToolCall, ToolResult, Usage};

use async_trait::async_trait;
use shared_types::*;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EngineType {
    OpenCode,
    Claude,
}

impl std::fmt::Display for EngineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineType::OpenCode => write!(f, "opencode"),
            EngineType::Claude => write!(f, "claude"),
        }
    }
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Engine error: {0}")]
    Engine(String),
    #[error("Tool execution error: {0}")]
    Tool(String),
    #[error("No engine selected")]
    NoEngine,
    #[error("Engine not found: {0}")]
    NotFound(String),
    #[error("Request cancelled")]
    Cancelled,
    #[error("Context exceeded")]
    ContextExceeded,
}

pub type Result<T> = std::result::Result<T, EngineError>;

#[async_trait]
pub trait AIEngine: Send + Sync {
    fn engine_type(&self) -> EngineType;
    fn name(&self) -> &str;
    
    async fn run(&self, request: EngineRequest) -> Result<EngineResponse>;
    
    async fn execute_tool(&self, tool_call: ToolCall) -> Result<ToolResult>;
    
    async fn reset(&self) -> Result<()>;
}

pub struct EngineManager {
    engines: RwLock<HashMap<EngineType, Arc<dyn AIEngine>>>,
    current: RwLock<Option<EngineType>>,
}

impl EngineManager {
    pub fn new() -> Self {
        Self {
            engines: RwLock::new(HashMap::new()),
            current: RwLock::new(None),
        }
    }

    pub async fn register(&self, engine_type: EngineType, engine: Arc<dyn AIEngine>) {
        let mut engines = self.engines.write().await;
        engines.insert(engine_type, engine);
        info!("Registered engine: {:?}", engine_type);
    }

    pub async fn select(&self, engine_type: EngineType) -> Result<()> {
        let engines = self.engines.read().await;
        if !engines.contains_key(&engine_type) {
            return Err(EngineError::NotFound(engine_type.to_string()));
        }
        let mut current = self.current.write().await;
        *current = Some(engine_type);
        info!("Switched to engine: {:?}", engine_type);
        Ok(())
    }

    pub async fn current_engine(&self) -> Result<Arc<dyn AIEngine>> {
        let current = self.current.read().await;
        let engine_type = current.ok_or(EngineError::NoEngine)?;
        
        let engines = self.engines.read().await;
        let engine = engines.get(&engine_type)
            .ok_or(EngineError::NotFound(engine_type.to_string()))?
            .clone();
        
        Ok(engine)
    }

    pub async fn run(&self, request: EngineRequest) -> Result<EngineResponse> {
        let engine = self.current_engine().await?;
        engine.run(request).await
    }

    pub async fn switch(&self, engine_type: EngineType) -> Result<()> {
        self.select(engine_type).await
    }

    pub async fn list_engines(&self) -> Vec<EngineType> {
        let engines = self.engines.read().await;
        engines.keys().copied().collect()
    }

    pub async fn current(&self) -> Option<EngineType> {
        let current = self.current.read().await;
        current.clone()
    }
}

impl Default for EngineManager {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;

pub struct ParallelExecutor {
    max_parallel: usize,
}

impl ParallelExecutor {
    pub fn new(max_parallel: usize) -> Self {
        Self { max_parallel }
    }

    pub async fn execute_all(
        &self,
        engine: Arc<dyn AIEngine>,
        tool_calls: Vec<ToolCall>,
    ) -> Vec<ToolResult> {
        use futures::stream::StreamExt;
        
        let stream = futures::stream::iter(tool_calls)
            .map(|call| {
                let engine = engine.clone();
                async move {
                    engine.execute_tool(call).await.unwrap_or_else(|e| ToolResult {
                        tool_call_id: String::new(),
                        content: format!("Error: {}", e),
                        is_error: true,
                    })
                }
            })
            .buffer_unordered(self.max_parallel);
        
        stream.collect().await
    }
}

pub struct EngineBuilder {
    manager: EngineManager,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            manager: EngineManager::new(),
        }
    }

    pub async fn with_opencode(self, engine: Arc<dyn AIEngine>) -> Self {
        self.manager.register(EngineType::OpenCode, engine).await;
        self
    }

    pub async fn with_claude(self, engine: Arc<dyn AIEngine>) -> Self {
        self.manager.register(EngineType::Claude, engine).await;
        self
    }

    pub async fn with_default(self, engine_type: EngineType) -> Self {
        self.manager.select(engine_type).await.unwrap();
        self
    }

    pub fn build(self) -> EngineManager {
        self.manager
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}