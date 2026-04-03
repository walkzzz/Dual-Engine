use async_trait::async_trait;
use engine_core::{AIEngine, EngineError, EngineRequest, EngineResponse, EngineType, Result};
use shared_types::*;
use std::process::Command;
use std::sync::Arc;
use tracing::{debug, info};

pub struct ClaudeEngine {
    claude_path: String,
}

impl ClaudeEngine {
    pub fn new(claude_path: Option<String>) -> Self {
        let path = claude_path.unwrap_or_else(|| {
            std::env::var("CLAUDE_BIN")
                .or_else(|_| std::env::var("CLAUDE_CODE_PATH"))
                .unwrap_or_else(|_| "bin/claude.exe".to_string())
        });
        
        Self {
            claude_path: path,
        }
    }
}

impl Default for ClaudeEngine {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl AIEngine for ClaudeEngine {
    fn engine_type(&self) -> EngineType {
        EngineType::Claude
    }

    fn name(&self) -> &str {
        "claude"
    }

    async fn run(&self, request: EngineRequest) -> Result<EngineResponse> {
        info!("Claude engine processing request");

        let last_msg = request.messages
            .iter()
            .rev()
            .find(|m| m.role == Role::User)
            .ok_or_else(|| EngineError::Engine("No user message found".to_string()))?;

        info!("Calling claude with: {}", last_msg.content);

        let output = Command::new(&self.claude_path)
            .arg(&last_msg.content)
            .output()
            .map_err(|e| EngineError::Engine(format!("Failed to run claude: {}", e)))?;

        let content = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            debug!("Claude stderr: {}", stderr);
            return Err(EngineError::Engine(format!("Claude error: {}", stderr)));
        }

        info!("Claude response: {}", content.chars().take(100).collect::<String>());

        Ok(EngineResponse {
            content,
            tool_calls: vec![],
            finish_reason: FinishReason::EndTurn,
            usage: None,
        })
    }

    async fn execute_tool(&self, tool_call: ToolCall) -> Result<ToolResult> {
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

pub fn create_claude_engine(claude_path: Option<String>) -> Arc<dyn AIEngine> {
    Arc::new(ClaudeEngine::new(claude_path))
}