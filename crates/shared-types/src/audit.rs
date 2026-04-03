/// 审计日志模块
/// 
/// 记录敏感操作和 API 调用用于安全审计

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::sync::RwLock;

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 操作类型
    pub action: AuditAction,
    /// 用户/会话标识
    pub user_id: Option<String>,
    /// 操作对象
    pub target: String,
    /// 操作结果
    pub result: AuditResult,
    /// 详细信息
    pub details: Option<String>,
    /// IP 地址 (如果可用)
    pub ip_address: Option<String>,
}

/// 操作类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    /// API 调用
    ApiCall,
    /// 引擎切换
    EngineSwitch,
    /// 配置修改
    ConfigChange,
    /// 文件访问
    FileAccess,
    /// 命令执行
    CommandExecution,
    /// 认证事件
    Authentication,
    /// 速率限制触发
    RateLimit,
    /// 错误事件
    Error,
}

/// 操作结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AuditResult {
    Success,
    Failure(String),
    Denied(String),
}

/// 审计日志记录器
pub struct AuditLogger {
    /// 日志存储
    logs: Arc<RwLock<Vec<AuditLogEntry>>>,
    /// 最大日志数量
    max_entries: usize,
    /// 总操作数
    total_operations: AtomicU64,
    /// 失败操作数
    failed_operations: AtomicU64,
    /// 是否启用
    enabled: AtomicU64,
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new(10000)
    }
}

impl AuditLogger {
    /// 创建新的审计日志记录器
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::with_capacity(max_entries / 10))),
            max_entries,
            total_operations: AtomicU64::new(0),
            failed_operations: AtomicU64::new(0),
            enabled: AtomicU64::new(1),
        }
    }

    /// 启用/禁用审计
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(if enabled { 1 } else { 0 }, Ordering::Relaxed);
    }

    /// 是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed) == 1
    }

    /// 记录日志
    pub async fn log(&self, entry: AuditLogEntry) {
        if !self.is_enabled() {
            return;
        }

        self.total_operations.fetch_add(1, Ordering::Relaxed);
        
        if matches!(entry.result, AuditResult::Failure(_) | AuditResult::Denied(_)) {
            self.failed_operations.fetch_add(1, Ordering::Relaxed);
        }

        let mut logs = self.logs.write().await;
        
        // 如果超出限制，删除最旧的日志
        if logs.len() >= self.max_entries {
            let remove_count = self.max_entries / 10;
            logs.drain(0..remove_count);
        }
        
        logs.push(entry);
    }

    /// 便捷方法：记录 API 调用
    pub async fn log_api_call(
        &self,
        user_id: Option<String>,
        engine: &str,
        result: AuditResult,
        details: Option<String>,
    ) {
        self.log(AuditLogEntry {
            timestamp: Utc::now(),
            action: AuditAction::ApiCall,
            user_id,
            target: engine.to_string(),
            result,
            details,
            ip_address: None,
        })
        .await;
    }

    /// 便捷方法：记录错误
    pub async fn log_error(
        &self,
        user_id: Option<String>,
        target: &str,
        error: &str,
    ) {
        self.log(AuditLogEntry {
            timestamp: Utc::now(),
            action: AuditAction::Error,
            user_id,
            target: target.to_string(),
            result: AuditResult::Failure(error.to_string()),
            details: Some(error.to_string()),
            ip_address: None,
        })
        .await;
    }

    /// 获取最近的日志
    pub async fn get_recent(&self, count: usize) -> Vec<AuditLogEntry> {
        let logs = self.logs.read().await;
        logs.iter().rev().take(count).cloned().collect()
    }

    /// 获取统计信息
    pub fn stats(&self) -> AuditStats {
        AuditStats {
            total_operations: self.total_operations.load(Ordering::Relaxed),
            failed_operations: self.failed_operations.load(Ordering::Relaxed),
            failure_rate: if self.total_operations.load(Ordering::Relaxed) > 0 {
                self.failed_operations.load(Ordering::Relaxed) as f64 
                    / self.total_operations.load(Ordering::Relaxed) as f64
            } else {
                0.0
            },
            cached_entries: self.logs.try_read().map(|l| l.len()).unwrap_or(0),
            is_enabled: self.is_enabled(),
        }
    }

    /// 导出日志为 JSON
    pub async fn export_json(&self) -> Result<String, serde_json::Error> {
        let logs = self.logs.read().await;
        serde_json::to_string_pretty(&*logs)
    }

    /// 清空日志
    pub async fn clear(&self) {
        self.logs.write().await.clear();
    }
}

/// 审计统计信息
#[derive(Debug, Clone)]
pub struct AuditStats {
    pub total_operations: u64,
    pub failed_operations: u64,
    pub failure_rate: f64,
    pub cached_entries: usize,
    pub is_enabled: bool,
}

impl std::fmt::Display for AuditStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "审计统计：总操作 {}, 失败 {} ({:.1}%), 缓存 {} 条, {}",
            self.total_operations,
            self.failed_operations,
            self.failure_rate * 100.0,
            self.cached_entries,
            if self.is_enabled { "已启用" } else { "已禁用" }
        )
    }
}

// 全局审计日志实例 (可选)
lazy_static::lazy_static! {
    pub static ref GLOBAL_AUDIT_LOGGER: AuditLogger = AuditLogger::new(10000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_logger_basic() {
        let logger = AuditLogger::new(100);
        
        logger.log_api_call(
            Some("user1".to_string()),
            "opencode",
            AuditResult::Success,
            Some("Test call".to_string()),
        ).await;
        
        let recent = logger.get_recent(10).await;
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].action, AuditAction::ApiCall);
        assert_eq!(recent[0].result, AuditResult::Success);
    }

    #[tokio::test]
    async fn test_audit_logger_stats() {
        let logger = AuditLogger::new(100);
        
        logger.log_api_call(None, "opencode", AuditResult::Success, None).await;
        logger.log_api_call(None, "opencode", AuditResult::Failure("error".to_string()), None).await;
        
        let stats = logger.stats();
        assert_eq!(stats.total_operations, 2);
        assert_eq!(stats.failed_operations, 1);
        assert!((stats.failure_rate - 0.5).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_audit_logger_max_entries() {
        let logger = AuditLogger::new(10);
        
        // 记录超过最大数量的日志
        for i in 0..20 {
            logger.log_api_call(
                None,
                "opencode",
                AuditResult::Success,
                Some(format!("Call {}", i)),
            ).await;
        }
        
        let recent = logger.get_recent(100).await;
        assert!(recent.len() <= 10);
    }

    #[test]
    fn test_audit_entry_serialization() {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            action: AuditAction::ApiCall,
            user_id: Some("user1".to_string()),
            target: "opencode".to_string(),
            result: AuditResult::Success,
            details: Some("test".to_string()),
            ip_address: None,
        };
        
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("api_call"));
        assert!(json.contains("user1"));
    }
}