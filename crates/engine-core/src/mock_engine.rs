/// Mock 引擎用于单元测试
/// 
/// 实现 Engine Trait 用于测试，不依赖真实 API

use async_trait::async_trait;
use shared_types::*;

/// Mock 引擎配置
#[derive(Debug, Clone)]
pub struct MockEngineConfig {
    pub should_fail: bool,
    pub fail_with: Option<EngineError>,
    pub response_content: String,
    pub delay_ms: u64,
}

impl Default for MockEngineConfig {
    fn default() -> Self {
        Self {
            should_fail: false,
            fail_with: None,
            response_content: "Mock response".to_string(),
            delay_ms: 0,
        }
    }
}

/// Mock 引擎实现
pub struct MockEngine {
    config: MockEngineConfig,
    status: EngineStatus,
    request_count: std::sync::atomic::AtomicU32,
}

impl MockEngine {
    pub fn new(config: MockEngineConfig) -> Self {
        Self {
            config,
            status: EngineStatus::Uninitialized,
            request_count: std::sync::atomic::AtomicU32::new(0),
        }
    }

    pub fn get_request_count(&self) -> u32 {
        self.request_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.request_count.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

#[async_trait]
impl Engine for MockEngine {
    fn engine_type(&self) -> shared_types::EngineType {
        shared_types::EngineType::OpenCode
    }

    fn name(&self) -> &str {
        "Mock Engine"
    }

    async fn initialize(&mut self, _config: EngineConfig) -> Result<(), DualEngineError> {
        if self.config.should_fail {
            self.status = EngineStatus::Error("Initialization failed".to_string());
            return Err(self.config.fail_with.clone().unwrap_or(EngineError::init_failed("test")).into());
        }
        self.status = EngineStatus::Ready;
        Ok(())
    }

    async fn execute(&self, request: EngineRequest) -> Result<EngineResponse, DualEngineError> {
        if self.status != EngineStatus::Ready {
            return Err(EngineError::InvalidState("Engine not ready".to_string()).into());
        }

        if self.config.should_fail {
            return Err(self.config.fail_with.clone().unwrap_or(EngineError::ExecutionFailed("test".to_string())).into());
        }

        // 模拟延迟
        if self.config.delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.config.delay_ms)).await;
        }

        self.request_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        Ok(EngineResponse {
            content: self.config.response_content.clone(),
            tool_calls: vec![],
            finish_reason: FinishReason::EndTurn,
            usage: Some(Usage {
                input_tokens: request.messages.iter().map(|m| m.content.len() as u32 / 4).sum(),
                output_tokens: self.config.response_content.len() as u32 / 4,
                total_tokens: 0,
            }),
        })
    }

    async fn destroy(&mut self) -> Result<(), DualEngineError> {
        self.status = EngineStatus::Destroyed;
        Ok(())
    }

    fn status(&self) -> EngineStatus {
        self.status.clone()
    }

    fn is_available(&self) -> bool {
        matches!(self.status, EngineStatus::Ready)
    }

    fn resource_usage(&self) -> ResourceUsage {
        ResourceUsage {
            memory_mb: 1.0,
            cpu_percent: 0.1,
            active_connections: 0,
            last_active: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_engine_success() {
        let config = MockEngineConfig {
            should_fail: false,
            response_content: "Success!".to_string(),
            ..Default::default()
        };
        let mut engine = MockEngine::new(config);
        
        // Initialize
        engine.initialize(EngineConfig::default()).await.unwrap();
        assert_eq!(engine.status(), EngineStatus::Ready);
        
        // Execute
        let request = EngineRequest {
            messages: vec![Message {
                role: Role::User,
                content: "Hello".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
            }],
            tools: vec![],
            context: HashMap::new(),
        };
        let response = engine.execute(request).await.unwrap();
        assert_eq!(response.content, "Success!");
        assert_eq!(engine.get_request_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_engine_failure() {
        let config = MockEngineConfig {
            should_fail: true,
            fail_with: Some(EngineError::ExecutionFailed("test error".to_string())),
            ..Default::default()
        };
        let mut engine = MockEngine::new(config);
        
        // Initialize should fail
        let result = engine.initialize(EngineConfig::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_engine_invalid_state() {
        let config = MockEngineConfig::default();
        let engine = MockEngine::new(config);
        
        // Execute without initializing should fail
        let request = EngineRequest {
            messages: vec![],
            tools: vec![],
            context: HashMap::new(),
        };
        let result = engine.execute(request).await;
        assert!(result.is_err());
    }
}