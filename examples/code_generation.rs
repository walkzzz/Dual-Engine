// 示例 2: 代码生成
// 运行：cargo run --bin de -- run -p "用 Rust 写一个线程安全的计数器"

use std::sync::{Arc, Mutex};

/// 线程安全计数器示例
/// 这是 AI 可能生成的代码示例
#[derive(Debug, Clone)]
struct Counter {
    value: Arc<Mutex<i32>>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Arc::new(Mutex::new(0)),
        }
    }

    fn increment(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }

    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

fn main() {
    println!("线程安全计数器示例");
    println!("==================");
    
    let counter = Counter::new();
    
    // 在多个线程中使用
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter_clone = counter.clone();
        let handle = std::thread::spawn(move || {
            println!("线程 {} 开始", i);
            for _ in 0..100 {
                counter_clone.increment();
            }
            println!("线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数值：{}", counter.get());
    println!("期望值：1000");
}