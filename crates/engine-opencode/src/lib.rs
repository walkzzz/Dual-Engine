use async_trait::async_trait;
use engine_core::{AIEngine, EngineError, EngineRequest, EngineResponse, EngineType, Result};
use shared_types::*;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;
use tracing::{debug, info};

pub struct OpenCodeEngine {
    opencode_path: String,
    env: HashMap<String, String>,
}

impl OpenCodeEngine {
    pub fn new(binary_path: Option<String>) -> Self {
        let path = binary_path.unwrap_or_else(|| {
            std::env::var("OPENCODE_BIN")
                .or_else(|_| std::env::var("OPENCODE_PATH"))
                .unwrap_or_else(|_| "bin/opencode.exe".to_string())
        });

        let mut env = HashMap::new();
        
        // Always set DashScope env vars
        if let Ok(key) = std::env::var("DASHSCOPE_API_KEY") {
            if !key.is_empty() {
                env.insert("DASHSCOPE_API_KEY".to_string(), key);
            }
        }
        if let Ok(endpoint) = std::env::var("LOCAL_ENDPOINT") {
            if !endpoint.is_empty() {
                env.insert("LOCAL_ENDPOINT".to_string(), endpoint);
            }
        }
        if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
            if !key.is_empty() {
                env.insert("OPENROUTER_API_KEY".to_string(), key);
            }
        }

        Self {
            opencode_path: path,
            env,
        }
    }
}

impl Default for OpenCodeEngine {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl AIEngine for OpenCodeEngine {
    fn engine_type(&self) -> EngineType {
        EngineType::OpenCode
    }

    fn name(&self) -> &str {
        "opencode"
    }

    async fn run(&self, request: EngineRequest) -> Result<EngineResponse> {
        info!("OpenCode engine processing request");

        let last_msg = request.messages
            .iter()
            .rev()
            .find(|m| m.role == Role::User)
            .ok_or_else(|| EngineError::Engine("No user message found".to_string()))?;

        info!("Calling opencode with: {}", last_msg.content);

        let mut cmd = Command::new(&self.opencode_path);
        cmd.arg("-p")
            .arg(&last_msg.content)
            .arg("-f")
            .arg("json");

        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        let output = cmd.output()
            .map_err(|e| EngineError::Engine(format!("Failed to run opencode: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            debug!("OpenCode stderr: {}", stderr);
            return Err(EngineError::Engine(format!("OpenCode error: {}", stderr)));
        }

        info!("OpenCode response: {}", content.chars().take(100).collect::<String>());

        Ok(EngineResponse {
            content,
            tool_calls: vec![],
            finish_reason: FinishReason::EndTurn,
            usage: None,
        })
    }

    async fn execute_tool(&self, tool_call: ToolCall) -> Result<ToolResult> {
        // This won't be called in subprocess mode
        // Tools are executed by the external opencode process
        Ok(ToolResult {
            tool_call_id: tool_call.id,
            content: "Tools handled by external process".to_string(),
            is_error: false,
        })
    }

    async fn reset(&self) -> Result<()> {
        Ok(())
    }
}

pub fn create_opencode_engine(opencode_path: Option<String>) -> Arc<dyn AIEngine> {
    Arc::new(OpenCodeEngine::new(opencode_path))
}