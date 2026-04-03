/// Dual-Engine TUI - 直接启动 OpenCode 原生 TUI
/// 
/// det 现在完全等同于 opencode，但支持通过环境变量自动配置 API
use std::env;
use std::process::Command;

fn main() {
    // 获取环境变量
    let moonshot_key = env::var("MOONSHOT_API_KEY").unwrap_or_default();
    let dashscope_key = env::var("DASHSCOPE_API_KEY").unwrap_or_default();
    let local_endpoint = env::var("LOCAL_ENDPOINT")
        .unwrap_or_else(|_| "https://api.moonshot.cn/v1".to_string());

    // 获取当前脚本所在目录
    let exe_path = env::current_exe()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("opencode.exe");

    // 直接启动 OpenCode，传递环境变量
    let status = Command::new(&exe_path)
        .env("MOONSHOT_API_KEY", &moonshot_key)
        .env("DASHSCOPE_API_KEY", &dashscope_key)
        .env("LOCAL_ENDPOINT", &local_endpoint)
        .status();

    match status {
        Ok(s) => {
            std::process::exit(s.code().unwrap_or(1));
        }
        Err(e) => {
            eprintln!("❌ 无法启动 OpenCode TUI: {}", e);
            eprintln!();
            eprintln!("💡 提示:");
            eprintln!("  1. 确保 opencode.exe 在 det.exe 同一目录下");
            eprintln!("  2. 或者直接使用：opencode");
            std::process::exit(1);
        }
    }
}