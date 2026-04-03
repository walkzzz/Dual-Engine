/// Dual-Engine TUI - 启动 OpenCode 原生 TUI 界面
/// 
/// det (dual-engine-tui) 现在直接启动 OpenCode 的 TUI 模式
use std::env;
use std::process::Command;

fn main() {
    // 获取环境变量
    let moonshot_key = env::var("MOONSHOT_API_KEY").unwrap_or_default();
    let dashscope_key = env::var("DASHSCOPE_API_KEY").unwrap_or_default();
    let local_endpoint = env::var("LOCAL_ENDPOINT").unwrap_or_else(|_| {
        // 默认使用 MoonShot API
        "https://api.moonshot.cn/v1".to_string()
    });

    // 打印启动信息
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         Dual-Engine TUI - OpenCode Mode                   ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ 启动 OpenCode 原生 TUI 界面...                              ║");
    println!("║                                                           ║");
    if !moonshot_key.is_empty() {
        println!("║ ✓ MoonShot API Key 已设置                                 ║");
    }
    if !dashscope_key.is_empty() {
        println!("║ ✓ DashScope API Key 已设置                                ║");
    }
    println!("║ 端点：{:<48} ║", local_endpoint);
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ 快捷键:                                                   ║");
    println!("║   Ctrl+q - 退出 TUI                                       ║");
    println!("║   Ctrl+c - 强制退出                                       ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // 获取当前脚本所在目录
    let exe_path = env::current_exe()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join("opencode.exe");

    // 启动 OpenCode TUI
    let status = Command::new(&exe_path)
        .env("MOONSHOT_API_KEY", &moonshot_key)
        .env("DASHSCOPE_API_KEY", &dashscope_key)
        .env("LOCAL_ENDPOINT", &local_endpoint)
        .status()
        .expect("Failed to start OpenCode TUI");

    if !status.success() {
        eprintln!();
        eprintln!("❌ OpenCode TUI 退出，状态码：{}", status);
        eprintln!();
        eprintln!("💡 提示:");
        eprintln!("  1. 确保 opencode.exe 在 det.exe 同一目录下");
        eprintln!("  2. 设置 API Key: export MOONSHOT_API_KEY=sk-xxx");
        eprintln!("  3. 或者直接使用：opencode");
        std::process::exit(1);
    }
}