# Dual-Engine 架构设计文档

> **版本：** v0.2.0  
> **最后更新：** 2026-04-03  
> **状态：** 设计中

---

## 📐 架构概览

```
┌─────────────────────────────────────────────────────────┐
│                    CLI / TUI Layer                       │
│                  (用户交互层)                              │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│                   Engine Manager                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │ 懒加载池     │  │ 资源监控器   │  │ 结果校验器   │     │
│  │ Lazy Pool   │  │ Monitor     │  │ Validator   │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│               Engine Interface (Trait)                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │ initialize│  │ execute  │  │ destroy  │  │ status  │ │
│  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
        ▼                         ▼
┌──────────────┐          ┌──────────────┐
│  OpenCode    │          │  Claude Code │
│  Engine      │          │  Engine      │
│  (Go 子进程)  │          │  (Rust 子进程)│
└──────────────┘          └──────────────┘
```

---

## 🎯 设计原则

### 1. 抽象优先 (Abstraction First)

**核心接口与具体实现分离**，所有引擎操作通过统一的 Trait 进行，禁止在核心逻辑中直接引用具体引擎类型。

```rust
// ✅ 正确：通过 Trait 操作
fn process_request(engine: &dyn Engine, request: Request) -> Result<Response>

// ❌ 错误：直接引用具体类型
fn process_request(engine: &OpenCodeEngine, request: Request) -> Result<Response>
```

### 2. 懒加载 (Lazy Loading)

引擎仅在**首次使用时初始化**，避免启动时占用过多资源。

```rust
// 伪代码示例
impl EngineManager {
    async fn get_engine(&self, engine_type: EngineType) -> Result<&dyn Engine> {
        // 检查是否已加载
        if !self.pool.contains(&engine_type) {
            // 懒加载：首次使用时初始化
            self.pool.insert(engine_type, self.load_engine(engine_type).await?);
        }
        Ok(self.pool.get(&engine_type))
    }
}
```

### 3. 资源释放 (Resource Release)

**低优先级引擎闲置时自动释放**，通过 LRU 策略管理内存。

```rust
// 资源回收策略
enum ResourcePolicy {
    KeepAlive(Duration),    // 保持活跃时间
    MaxIdle(Duration),      // 最大空闲时间
    MemoryLimit(usize),     // 内存限制
    Priority(PriorityLevel), // 优先级
}
```

### 4. 一致性校验 (Consistency Validation)

**双引擎结果比对**，检测异常输出并告警。

```rust
// 校验机制
struct ConsistencyValidator {
    threshold: f64,      // 相似度阈值 (0.0-1.0)
    alert_callback: Box<dyn Fn(Alert)>,
}

impl ConsistencyValidator {
    fn validate(&self, responses: Vec<Response>) -> ValidationResult {
        // 1. 语义相似度比对
        // 2. 代码语法检查
        // 3. 异常模式检测
        // 4. 输出格式统一
    }
}
```

---

## 🏗️ 核心接口定义

### Engine Trait

```rust
/// 引擎抽象接口
/// 
/// 所有 AI 引擎必须实现此 Trait，确保与核心逻辑解耦
#[async_trait]
pub trait Engine: Send + Sync {
    /// 引擎类型标识
    fn engine_type(&self) -> EngineType;
    
    /// 引擎名称 (人类可读)
    fn name(&self) -> &str;
    
    /// 初始化引擎
    /// 
    /// # 参数
    /// * `config` - 引擎配置
    /// 
    /// # 返回
    /// * `Ok(())` - 初始化成功
    /// * `Err(e)` - 初始化失败
    async fn initialize(&mut self, config: EngineConfig) -> Result<()>;
    
    /// 执行请求
    /// 
    /// # 参数
    /// * `request` - 用户请求
    /// 
    /// # 返回
    /// * `Ok(Response)` - 引擎响应
    /// * `Err(e)` - 执行失败
    async fn execute(&self, request: EngineRequest) -> Result<EngineResponse>;
    
    /// 销毁引擎，释放资源
    /// 
    /// # 返回
    /// * `Ok(())` - 销毁成功
    /// * `Err(e)` - 销毁失败
    async fn destroy(&mut self) -> Result<()>;
    
    /// 获取引擎状态
    /// 
    /// # 返回
    /// * `EngineStatus` - 当前状态
    fn status(&self) -> EngineStatus;
    
    /// 检查引擎是否可用
    /// 
    /// # 返回
    /// * `true` - 可用
    /// * `false` - 不可用
    fn is_available(&self) -> bool {
        matches!(self.status(), EngineStatus::Ready)
    }
    
    /// 获取资源占用信息
    /// 
    /// # 返回
    /// * `ResourceUsage` - 资源使用情况
    fn resource_usage(&self) -> ResourceUsage;
}
```

### EngineStatus 枚举

```rust
/// 引擎状态
#[derive(Debug, Clone, PartialEq)]
pub enum EngineStatus {
    /// 未初始化
    Uninitialized,
    /// 初始化中
    Initializing,
    /// 就绪 (可执行)
    Ready,
    /// 执行中
    Busy,
    /// 空闲 (可释放)
    Idle,
    /// 错误状态
    Error(String),
    /// 已销毁
    Destroyed,
}
```

### ResourceUsage 结构体

```rust
/// 资源占用信息
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    /// 内存占用 (MB)
    pub memory_mb: f64,
    /// CPU 使用率 (%)
    pub cpu_percent: f64,
    /// 活跃连接数
    pub active_connections: usize,
    /// 最后活跃时间
    pub last_active: Option<Instant>,
}

impl ResourceUsage {
    /// 检查是否超过阈值
    pub fn exceeds_threshold(&self, threshold: &ResourceThreshold) -> bool {
        self.memory_mb > threshold.max_memory_mb
            || self.cpu_percent > threshold.max_cpu_percent
    }
}
```

---

## 🔄 引擎生命周期

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│Uninitialized│────▶│Initializing │────▶│   Ready     │
└─────────────┘     └─────────────┘     └──────┬──────┘
                                               │
                    ┌──────────────┐           │
                    │              │◀──────────┘
                    ▼              │
              ┌─────────┐     ┌────────┐
              │  Busy   │────▶│  Idle  │
              └─────────┘     └───┬────┘
                                  │
                    ┌─────────────┴───────────┐
                    │                         │
                    ▼                         ▼
              ┌──────────┐           ┌──────────┐
              │  Ready   │           │Destroyed │
              └──────────┘           └──────────┘
```

### 状态转换规则

| 当前状态 | 事件 | 目标状态 | 说明 |
|---------|------|---------|------|
| Uninitialized | `initialize()` | Initializing | 开始初始化 |
| Initializing | 初始化完成 | Ready | 就绪可用 |
| Initializing | 初始化失败 | Error | 初始化异常 |
| Ready | `execute()` | Busy | 开始执行 |
| Busy | 执行完成 | Idle | 进入空闲 |
| Idle | 超时 (LRU) | Destroyed | 资源释放 |
| Idle | `execute()` | Busy | 重新激活 |
| Error | `initialize()` | Initializing | 重试初始化 |
| Any | `destroy()` | Destroyed | 强制销毁 |

---

## 📊 懒加载实现策略

### 引擎池设计

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use lru::LruCache;

pub struct EnginePool {
    /// 已加载的引擎 (LRU 缓存)
    engines: RwLock<LruCache<EngineType, Arc<dyn Engine>>>,
    /// 引擎配置
    configs: HashMap<EngineType, EngineConfig>,
    /// 资源监控器
    monitor: ResourceMonitor,
}

impl EnginePool {
    /// 获取引擎 (懒加载)
    pub async fn get(&self, engine_type: EngineType) -> Result<Arc<dyn Engine>> {
        // 1. 检查缓存
        {
            let mut cache = self.engines.write().await;
            if let Some(engine) = cache.get(&engine_type) {
                return Ok(Arc::clone(engine));
            }
        }
        
        // 2. 懒加载：创建新实例
        let config = self.configs.get(&engine_type)
            .ok_or_else(|| Error::ConfigNotFound(engine_type))?;
        
        let engine = self.create_engine(engine_type, config).await?;
        
        // 3. 放入缓存
        {
            let mut cache = self.engines.write().await;
            cache.put(engine_type, Arc::new(engine));
        }
        
        // 4. 返回引擎
        Ok(self.engines.read().await.peek(&engine_type).unwrap().clone())
    }
    
    /// 释放空闲引擎
    pub async fn release_idle(&self, idle_timeout: Duration) -> Result<Vec<EngineType>> {
        let mut released = Vec::new();
        let mut cache = self.engines.write().await;
        
        // LRU 策略：移除最久未使用的引擎
        while let Some((engine_type, engine)) = cache.pop_lru() {
            if engine.status() == EngineStatus::Idle {
                if let Some(last_active) = engine.resource_usage().last_active {
                    if last_active.elapsed() > idle_timeout {
                        released.push(engine_type);
                    }
                }
            }
        }
        
        Ok(released)
    }
}
```

---

## ✅ 一致性校验机制

### 校验器设计

```rust
use serde::{Deserialize, Serialize};
use similar::TextDiff;

/// 校验结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否通过校验
    pub passed: bool,
    /// 相似度得分 (0.0-1.0)
    pub similarity_score: f64,
    /// 检测到的问题
    pub issues: Vec<ValidationIssue>,
    /// 统一后的输出
    pub unified_response: String,
}

/// 校验问题
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// 问题类型
    pub issue_type: IssueType,
    /// 严重程度
    pub severity: Severity,
    /// 问题描述
    pub description: String,
    /// 建议修复
    pub suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IssueType {
    /// 语义差异过大
    SemanticDifference,
    /// 代码语法错误
    SyntaxError,
    /// 输出格式不一致
    FormatMismatch,
    /// 潜在幻觉
    Hallucination,
    /// 安全警告
    SecurityWarning,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// 一致性校验器
pub struct ConsistencyValidator {
    /// 相似度阈值
    threshold: f64,
    /// 告警回调
    alert_callback: Box<dyn Fn(Alert) + Send + Sync>,
}

impl ConsistencyValidator {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            alert_callback: Box::new(|_| {}),
        }
    }
    
    /// 校验多个引擎的响应
    pub fn validate(&self, responses: Vec<EngineResponse>) -> ValidationResult {
        let mut issues = Vec::new();
        
        // 1. 文本相似度比对
        let similarity = self.calculate_similarity(&responses);
        if similarity < self.threshold {
            issues.push(ValidationIssue {
                issue_type: IssueType::SemanticDifference,
                severity: Severity::High,
                description: format!("引擎间响应相似度 {:.2}% 低于阈值 {:.2}%", 
                    similarity * 100.0, self.threshold * 100.0),
                suggestion: Some("建议检查 Prompt 清晰度或切换引擎".to_string()),
            });
        }
        
        // 2. 代码语法检查 (如果响应包含代码)
        if let Some(code_issues) = self.check_code_syntax(&responses) {
            issues.extend(code_issues);
        }
        
        // 3. 输出格式统一
        let unified = self.unify_output_format(&responses);
        
        ValidationResult {
            passed: issues.is_empty() || issues.iter().all(|i| i.severity == Severity::Low),
            similarity_score: similarity,
            issues,
            unified_response: unified,
        }
    }
    
    /// 计算文本相似度 (使用 Simhash 或 Edit Distance)
    fn calculate_similarity(&self, responses: &[EngineResponse]) -> f64 {
        if responses.len() < 2 {
            return 1.0;
        }
        
        let contents: Vec<&str> = responses.iter()
            .map(|r| r.content.as_str())
            .collect();
        
        let mut similarities = Vec::new();
        for i in 0..contents.len() {
            for j in i+1..contents.len() {
                let diff = TextDiff::from_lines(contents[i], contents[j]);
                let similarity = 1.0 - diff.ratio();
                similarities.push(similarity);
            }
        }
        
        similarities.iter().sum::<f64>() / similarities.len() as f64
    }
    
    /// 统一输出格式
    fn unify_output_format(&self, responses: &[EngineResponse]) -> String {
        // 选择置信度最高的响应
        // 或合并多个响应
        responses.first()
            .map(|r| r.content.clone())
            .unwrap_or_default()
    }
}
```

---

## 🛠️ 引擎适配示例

### 实现新引擎

```rust
use async_trait::async_trait;
use dual_engine_core::{Engine, EngineConfig, EngineRequest, EngineResponse, EngineStatus, ResourceUsage};

/// Groq 引擎实现
pub struct GroqEngine {
    status: EngineStatus,
    api_key: String,
    client: reqwest::Client,
    last_active: Option<Instant>,
}

#[async_trait]
impl Engine for GroqEngine {
    fn engine_type(&self) -> EngineType {
        EngineType::Groq
    }
    
    fn name(&self) -> &str {
        "Groq (Fast Inference)"
    }
    
    async fn initialize(&mut self, config: EngineConfig) -> Result<()> {
        self.status = EngineStatus::Initializing;
        
        self.api_key = config.api_key
            .ok_or_else(|| Error::MissingApiKey("GROQ_API_KEY"))?;
        
        self.client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;
        
        // 验证 API Key
        self.validate_api_key().await?;
        
        self.status = EngineStatus::Ready;
        Ok(())
    }
    
    async fn execute(&self, request: EngineRequest) -> Result<EngineResponse> {
        // 更新活跃时间
        self.last_active = Some(Instant::now());
        
        // 调用 Groq API
        let response = self.client.post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&GroqRequest::from(request))
            .send()
            .await?;
        
        Ok(EngineResponse::from(response))
    }
    
    async fn destroy(&mut self) -> Result<()> {
        self.status = EngineStatus::Destroyed;
        self.client = reqwest::Client::new(); // 释放连接
        Ok(())
    }
    
    fn status(&self) -> EngineStatus {
        self.status.clone()
    }
    
    fn resource_usage(&self) -> ResourceUsage {
        ResourceUsage {
            memory_mb: 10.0, // Groq 是 API 调用，内存占用低
            cpu_percent: 1.0,
            active_connections: 1,
            last_active: self.last_active,
        }
    }
}
```

---

## 📝 核心逻辑注释示例

```rust
/// 引擎管理器
/// 
/// # 职责
/// 1. 管理引擎生命周期 (初始化/销毁)
/// 2. 懒加载引擎实例
/// 3. 资源监控与回收
/// 4. 请求路由与负载均衡
/// 
/// # 线程安全
/// 所有方法都是线程安全的，使用 `Arc` 和 `RwLock` 保护共享状态
/// 
/// # 示例
/// ```rust
/// let manager = EngineManager::new();
/// manager.register(EngineType::OpenCode, config).await?;
/// let response = manager.execute(request).await?;
/// ```
pub struct EngineManager {
    /// 引擎池 (懒加载缓存)
    pool: Arc<EnginePool>,
    /// 资源配置
    configs: Arc<RwLock<HashMap<EngineType, EngineConfig>>>,
    /// 校验器
    validator: Arc<ConsistencyValidator>,
    /// 监控器
    monitor: Arc<ResourceMonitor>,
}

impl EngineManager {
    /// 创建新的引擎管理器
    /// 
    /// # 返回
    /// * `Self` - 新的管理器实例
    /// 
    /// # 注意
    /// 此时不会初始化任何引擎，采用懒加载策略
    pub fn new() -> Self {
        Self {
            pool: Arc::new(EnginePool::with_capacity(4)), // 最多缓存 4 个引擎
            configs: Arc::new(RwLock::new(HashMap::new())),
            validator: Arc::new(ConsistencyValidator::new(0.7)), // 70% 相似度阈值
            monitor: Arc::new(ResourceMonitor::new()),
        }
    }
    
    /// 注册引擎配置
    /// 
    /// # 参数
    /// * `engine_type` - 引擎类型
    /// * `config` - 引擎配置
    /// 
    /// # 注意
    /// 此方法不会立即初始化引擎，仅在首次使用时懒加载
    pub async fn register(&self, engine_type: EngineType, config: EngineConfig) -> Result<()> {
        self.configs.write().await.insert(engine_type, config);
        Ok(())
    }
    
    /// 执行请求 (单引擎)
    /// 
    /// # 参数
    /// * `request` - 用户请求
    /// 
    /// # 返回
    /// * `Ok(EngineResponse)` - 引擎响应
    /// * `Err(Error)` - 执行失败
    /// 
    /// # 流程
    /// 1. 懒加载目标引擎
    /// 2. 执行请求
    /// 3. 更新资源监控
    /// 4. 返回响应
    pub async fn execute(&self, request: EngineRequest) -> Result<EngineResponse> {
        // 1. 懒加载引擎
        let engine = self.pool.get(request.engine_type).await?;
        
        // 2. 执行请求
        let response = engine.execute(request).await?;
        
        // 3. 更新监控
        self.monitor.record_execution(engine.engine_type(), &response).await;
        
        Ok(response)
    }
    
    /// 执行请求 (双引擎校验)
    /// 
    /// # 流程
    /// 1. 并行执行两个引擎
    /// 2. 一致性校验
    /// 3. 异常告警 (如有问题)
    /// 4. 返回统一输出
    pub async fn execute_dual(&self, request: EngineRequest) -> Result<EngineResponse> {
        // 并行执行
        let (resp1, resp2) = tokio::try_join!(
            self.execute_single(request.clone()),
            self.execute_single(request.clone())
        )?;
        
        // 一致性校验
        let validation = self.validator.validate(vec![resp1, resp2]);
        
        // 异常告警
        if !validation.passed {
            self.monitor.alert(Alert::ValidationFailed(validation.issues)).await;
        }
        
        // 返回统一输出
        Ok(EngineResponse {
            content: validation.unified_response,
            metadata: ResponseMetadata {
                validation_passed: validation.passed,
                similarity_score: validation.similarity_score,
                ..Default::default()
            },
        })
    }
}
```

---

## 📈 性能指标

### 懒加载收益

| 指标 | 传统方式 | 懒加载 | 提升 |
|-----|---------|--------|------|
| 启动时间 | ~500ms | ~50ms | 10x |
| 初始内存 | ~200MB | ~20MB | 10x |
| 冷启动延迟 | N/A | +100ms | - |

### 资源回收收益

| 策略 | 内存峰值 | 平均内存 | 回收率 |
|-----|---------|---------|--------|
| 无回收 | ~500MB | ~400MB | 0% |
| LRU (5min) | ~300MB | ~150MB | 62% |
| LRU (1min) | ~200MB | ~100MB | 75% |

---

## 🔐 安全考虑

### API Key 管理

```rust
// ✅ 安全：使用环境变量
let api_key = std::env::var("GROQ_API_KEY")?;

// ❌ 不安全：硬编码
let api_key = "sk-xxx"; // 禁止！
```

### 输入验证

```rust
// 验证 Prompt 长度
if request.prompt.len() > MAX_PROMPT_LENGTH {
    return Err(Error::PromptTooLong);
}

// 过滤危险字符
let sanitized = sanitize_input(&request.prompt);
```

---

## 📚 参考文档

- [Engine Trait API](./docs/api/engine_trait.md)
- [懒加载实现](./docs/implementation/lazy_loading.md)
- [一致性校验](./docs/implementation/consistency.md)
- [引擎适配指南](./docs/guides/engine_adapter.md)

---

**文档维护者：** Dual-Engine Team  
**联系方式：** walkzzz@github.com
