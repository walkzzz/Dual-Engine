/// 速率限制模块
/// 
/// 实现令牌桶算法进行 API 调用限流

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// 速率限制器（令牌桶算法）
pub struct RateLimiter {
    /// 桶容量（最大令牌数）
    capacity: u32,
    /// 当前令牌数
    tokens: AtomicU32,
    /// 令牌补充速率（每秒令牌数）
    refill_rate: u32,
    /// 上次补充时间（毫秒时间戳）
    last_refill: AtomicU64,
    /// 总请求数
    total_requests: AtomicU64,
    /// 被限流请求数
    limited_requests: AtomicU64,
}

impl RateLimiter {
    /// 创建新的速率限制器
    /// 
    /// # 参数
    /// * `capacity` - 桶容量（最大令牌数/最大并发数）
    /// * `refill_rate` - 令牌补充速率（每秒令牌数）
    /// 
    /// # 示例
    /// ```rust
    /// // 每秒最多 10 个请求
    /// let limiter = RateLimiter::new(10, 10);
    /// ```
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        Self {
            capacity,
            tokens: AtomicU32::new(capacity),
            refill_rate,
            last_refill: AtomicU64::new(Instant::now().elapsed().as_millis() as u64),
            total_requests: AtomicU64::new(0),
            limited_requests: AtomicU64::new(0),
        }
    }

    /// 尝试获取一个令牌
    /// 
    /// # 返回
    /// * `Ok(())` - 获取成功，可以继续执行
    /// * `Err(String)` - 获取失败，需要等待
    pub fn try_acquire(&self) -> Result<(), String> {
        self.refill();

        let mut tokens = self.tokens.load(Ordering::Relaxed);
        
        while tokens > 0 {
            match self.tokens.compare_exchange_weak(
                tokens,
                tokens - 1,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    self.total_requests.fetch_add(1, Ordering::Relaxed);
                    return Ok(());
                }
                Err(current) => tokens = current,
            }
        }

        self.limited_requests.fetch_add(1, Ordering::Relaxed);
        Err(format!(
            "速率限制：请等待 {} 毫秒后重试",
            1000 / self.refill_rate.max(1)
        ))
    }

    /// 阻塞等待直到获取令牌
    /// 
    /// # 参数
    /// * `timeout` - 最大等待时间
    pub async fn acquire(&self, timeout: Duration) -> Result<(), String> {
        let start = Instant::now();
        
        loop {
            match self.try_acquire() {
                Ok(()) => return Ok(()),
                Err(_) => {
                    if start.elapsed() >= timeout {
                        return Err(format!("等待超时：{} 秒", timeout.as_secs()));
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    /// 补充令牌
    fn refill(&self) {
        let now = Instant::now().elapsed().as_millis() as u64;
        let last = self.last_refill.load(Ordering::Relaxed);
        let elapsed = now - last;

        if elapsed >= 1000 {
            // 至少过了 1 秒
            let tokens_to_add = ((elapsed / 1000) as u32) * self.refill_rate;
            let current = self.tokens.load(Ordering::Relaxed);
            let new_tokens = (current + tokens_to_add).min(self.capacity);

            if self.tokens.compare_exchange(
                current,
                new_tokens,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ).is_ok() {
                self.last_refill.store(now, Ordering::Relaxed);
            }
        }
    }

    /// 获取统计信息
    pub fn stats(&self) -> RateLimiterStats {
        self.refill();
        RateLimiterStats {
            available_tokens: self.tokens.load(Ordering::Relaxed),
            capacity: self.capacity,
            refill_rate: self.refill_rate,
            total_requests: self.total_requests.load(Ordering::Relaxed),
            limited_requests: self.limited_requests.load(Ordering::Relaxed),
            limited_rate: if self.total_requests.load(Ordering::Relaxed) > 0 {
                self.limited_requests.load(Ordering::Relaxed) as f64 
                    / self.total_requests.load(Ordering::Relaxed) as f64
            } else {
                0.0
            },
        }
    }

    /// 重置限制器
    pub fn reset(&self) {
        self.tokens.store(self.capacity, Ordering::Relaxed);
        self.last_refill.store(Instant::now().elapsed().as_millis() as u64, Ordering::Relaxed);
        self.total_requests.store(0, Ordering::Relaxed);
        self.limited_requests.store(0, Ordering::Relaxed);
    }
}

/// 速率限制统计信息
#[derive(Debug, Clone)]
pub struct RateLimiterStats {
    pub available_tokens: u32,
    pub capacity: u32,
    pub refill_rate: u32,
    pub total_requests: u64,
    pub limited_requests: u64,
    pub limited_rate: f64,
}

impl std::fmt::Display for RateLimiterStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "速率限制：{}/{} tokens, 速率：{}/s, 总请求：{}, 限流：{} ({:.1}%)",
            self.available_tokens,
            self.capacity,
            self.refill_rate,
            self.total_requests,
            self.limited_requests,
            self.limited_rate * 100.0
        )
    }
}

/// 预定义的速率限制配置
impl RateLimiter {
    /// 激进配置：每秒 20 请求
    pub fn aggressive() -> Self {
        Self::new(20, 20)
    }

    /// 标准配置：每秒 10 请求
    pub fn standard() -> Self {
        Self::new(10, 10)
    }

    /// 保守配置：每秒 5 请求
    pub fn conservative() -> Self {
        Self::new(5, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_initial_state() {
        let limiter = RateLimiter::new(10, 10);
        let stats = limiter.stats();
        assert_eq!(stats.available_tokens, 10);
        assert_eq!(stats.capacity, 10);
    }

    #[tokio::test]
    async fn test_rate_limiter_acquire() {
        let limiter = RateLimiter::new(5, 5);
        
        // 前 5 次应该都成功
        for _ in 0..5 {
            assert!(limiter.try_acquire().is_ok());
        }
        
        // 第 6 次应该失败
        assert!(limiter.try_acquire().is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_refill() {
        let limiter = RateLimiter::new(2, 10); // 每秒补充 10 个
        
        // 耗尽令牌
        let _ = limiter.try_acquire();
        let _ = limiter.try_acquire();
        
        // 等待补充
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // 应该又有令牌了
        assert!(limiter.try_acquire().is_ok());
    }

    #[test]
    fn test_rate_limiter_stats() {
        let limiter = RateLimiter::new(10, 10);
        
        // 执行一些请求
        let _ = limiter.try_acquire();
        let _ = limiter.try_acquire();
        let _ = limiter.try_acquire();
        
        let stats = limiter.stats();
        assert_eq!(stats.total_requests, 3);
        assert!(stats.limited_rate >= 0.0 && stats.limited_rate <= 1.0);
    }

    #[test]
    fn test_rate_limiter_reset() {
        let limiter = RateLimiter::new(5, 5);
        
        // 耗尽令牌
        for _ in 0..5 {
            let _ = limiter.try_acquire();
        }
        
        // 重置
        limiter.reset();
        
        // 应该恢复满令牌
        let stats = limiter.stats();
        assert_eq!(stats.available_tokens, 5);
        assert_eq!(stats.total_requests, 0);
    }
}