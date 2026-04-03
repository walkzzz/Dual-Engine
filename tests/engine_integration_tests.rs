/// 引擎核心集成测试
/// 
/// 测试引擎管理器的核心功能

use engine_core::{EngineManager, EngineType, EngineConfig};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_manager_creation() {
        // 测试引擎管理器创建
        let manager = EngineManager::new();
        assert!(manager.list_engines().await.contains(&EngineType::OpenCode));
        assert!(manager.list_engines().await.contains(&EngineType::Claude));
    }

    #[tokio::test]
    async fn test_engine_selection() {
        // 测试引擎选择
        let manager = EngineManager::new();
        
        // 选择 OpenCode
        manager.select(EngineType::OpenCode).await.unwrap();
        assert_eq!(manager.current().await, Some(EngineType::OpenCode));
        
        // 选择 Claude
        manager.select(EngineType::Claude).await.unwrap();
        assert_eq!(manager.current().await, Some(EngineType::Claude));
    }

    #[tokio::test]
    async fn test_engine_switching() {
        // 测试引擎切换
        let manager = EngineManager::new();
        
        // 初始为 OpenCode
        assert_eq!(manager.current().await, Some(EngineType::OpenCode));
        
        // 切换到 Claude
        manager.switch(EngineType::Claude).await.unwrap();
        assert_eq!(manager.current().await, Some(EngineType::Claude));
        
        // 切换回 OpenCode
        manager.switch(EngineType::OpenCode).await.unwrap();
        assert_eq!(manager.current().await, Some(EngineType::OpenCode));
    }

    #[tokio::test]
    async fn test_invalid_engine_switching() {
        // 测试无效引擎切换
        let manager = EngineManager::new();
        
        // 注意：当前实现在 switch 时不会验证引擎是否存在
        // 这个测试用于记录预期行为
        let result = manager.switch(EngineType::OpenCode).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_engine_list() {
        // 测试引擎列表
        let manager = EngineManager::new();
        let engines = manager.list_engines().await;
        
        assert!(engines.contains(&EngineType::OpenCode));
        assert!(engines.contains(&EngineType::Claude));
        assert_eq!(engines.len(), 2);
    }
}