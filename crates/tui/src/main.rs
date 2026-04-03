/// Dual-Engine TUI - 支持斜杠命令切换引擎
/// 
/// 功能:
/// - 默认启动 OpenCode TUI
/// - 支持 /engine 命令切换引擎
/// - /engine opencode - 切换到 OpenCode TUI
/// - /engine claude - 切换到 Claude Code TUI

use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         Dual-Engine TUI - Interactive Mode                ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ 支持斜杠命令切换引擎：                                    ║");
    println!("║   /engine opencode  - 切换到 OpenCode TUI                 ║");
    println!("║   /engine claude    - 切换到 Claude Code TUI              ║");
    println!("║   /engine list      - 列出可用引擎                        ║");
    println!("║   /help             - 显示帮助                            ║");
    println!("║   /quit             - 退出                                ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    let mut current_engine = "opencode".to_string();

    loop {
        print!("det> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // 处理斜杠命令
        if input.starts_with('/') {
            match handle_command(input, &mut current_engine) {
                CommandResult::Continue => continue,
                CommandResult::Exit => break,
                CommandResult::Launch(engine) => {
                    launch_engine(&engine);
                    current_engine = engine;
                }
            }
        } else {
            // 默认启动当前引擎的 TUI
            launch_engine(&current_engine);
        }
    }
}

enum CommandResult {
    Continue,
    Exit,
    Launch(String),
}

fn handle_command(input: &str, current_engine: &mut String) -> CommandResult {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return CommandResult::Continue;
    }

    match parts[0] {
        "/engine" | "/e" => {
            if parts.len() < 2 {
                println!("用法：/engine <opencode|claude|list>");
                return CommandResult::Continue;
            }
            
            match parts[1] {
                "list" => {
                    println!();
                    println!("可用引擎:");
                    println!("  opencode  - OpenCode TUI (Go-based, 支持 MoonShot/DashScope)");
                    println!("  claude    - Claude Code TUI (Rust-based)");
                    println!();
                    println!("当前引擎：{}", current_engine);
                    println!();
                }
                "opencode" => {
                    println!("切换到 OpenCode TUI...");
                    *current_engine = "opencode".to_string();
                    return CommandResult::Launch("opencode".to_string());
                }
                "claude" => {
                    println!("切换到 Claude Code TUI...");
                    *current_engine = "claude".to_string();
                    return CommandResult::Launch("claude".to_string());
                }
                _ => {
                    println!("未知引擎：{}", parts[1]);
                    println!("可用引擎：opencode, claude");
                }
            }
            CommandResult::Continue
        }
        "/help" | "/h" | "/?" => {
            println!();
            println!("可用命令:");
            println!("  /engine <name>  - 切换引擎 (opencode|claude)");
            println!("  /engine list    - 列出可用引擎");
            println!("  /help           - 显示此帮助");
            println!("  /quit           - 退出");
            println!();
            println!("直接回车启动当前引擎的 TUI 模式");
            println!();
            CommandResult::Continue
        }
        "/quit" | "/exit" | "/q" => {
            println!("再见！");
            CommandResult::Exit
        }
        _ => {
            println!("未知命令：{}", parts[0]);
            println!("输入 /help 查看可用命令");
            println!();
            CommandResult::Continue
        }
    }
}

fn launch_engine(engine: &str) {
    let moonshot_key = env::var("MOONSHOT_API_KEY").unwrap_or_default();
    let dashscope_key = env::var("DASHSCOPE_API_KEY").unwrap_or_default();
    let local_endpoint = env::var("LOCAL_ENDPOINT")
        .unwrap_or_else(|_| "https://api.moonshot.cn/v1".to_string());

    let exe_path = env::current_exe()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join(format!("{}.exe", engine));

    println!();
    println!("启动 {} TUI...", engine);
    println!();

    let status = Command::new(&exe_path)
        .env("MOONSHOT_API_KEY", &moonshot_key)
        .env("DASHSCOPE_API_KEY", &dashscope_key)
        .env("LOCAL_ENDPOINT", &local_endpoint)
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                eprintln!("{} TUI 退出，状态码：{}", engine, s);
            }
        }
        Err(e) => {
            eprintln!("无法启动 {} TUI: {}", engine, e);
            eprintln!("请确保 {} 存在于 bin/ 目录中", engine);
        }
    }
}