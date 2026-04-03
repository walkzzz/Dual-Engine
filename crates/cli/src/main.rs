use clap::{Parser, Subcommand};
use engine_core::{EngineManager, EngineType};
use engine_claude::create_claude_engine;
use engine_opencode::create_opencode_engine;
use shared_types::{EngineRequest, Message, Role};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Parser)]
#[command(name = "dual-engine")]
#[command(about = "Dual Engine CLI - Switch between OpenCode and Claude engines")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, default_value = "opencode")]
    engine: String,

    #[arg(long)]
    opencode_path: Option<String>,

    #[arg(long)]
    claude_path: Option<String>,

    #[arg(long, default_value = "120")]
    timeout_secs: u64,
    
    #[arg(long, help = "API provider: moonshot, dashscope, groq")]
    provider: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        prompt: String,
    },
    Switch {
        engine: String,
    },
    Status,
    Setup,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();

    // 根据 provider 设置环境变量
    if let Some(provider) = &cli.provider {
        match provider.as_str() {
            "moonshot" => {
                std::env::set_var("LOCAL_ENDPOINT", "https://api.moonshot.cn/v1");
            }
            "dashscope" => {
                std::env::set_var("LOCAL_ENDPOINT", "https://dashscope.aliyuncs.com/compatible-mode/v1");
            }
            "groq" => {
                std::env::set_var("GROQ_API_KEY", std::env::var("GROQ_API_KEY").unwrap_or_default());
            }
            _ => {}
        }
    }

    let manager = EngineManager::new();

    // Register engines with custom paths
    manager
        .register(EngineType::OpenCode, create_opencode_engine(cli.opencode_path.clone()))
        .await;
    manager
        .register(EngineType::Claude, create_claude_engine(cli.claude_path.clone()))
        .await;

    match cli.engine.as_str() {
        "opencode" => manager.select(EngineType::OpenCode).await?,
        "claude" => manager.select(EngineType::Claude).await?,
        _ => {
            eprintln!("❌ Unknown engine: {}", cli.engine);
            eprintln!();
            eprintln!("Available engines:");
            eprintln!("  • opencode - OpenCode engine (Go-based, supports MoonShot/DashScope)");
            eprintln!("  • claude   - Claude Code engine (Rust-based)");
            eprintln!();
            eprintln!("💡 Solution: Use 'de status' to see available engines");
            std::process::exit(1);
        }
    }

    match &cli.command {
        Some(Commands::Run { prompt }) => {
            info!("Running with engine: {} (timeout: {}s)", cli.engine, cli.timeout_secs);
            
            let request = EngineRequest {
                messages: vec![Message {
                    role: Role::User,
                    content: prompt.clone(),
                    tool_calls: vec![],
                    tool_results: vec![],
                }],
                tools: vec![],
                context: std::collections::HashMap::new(),
            };

            let timeout_duration = Duration::from_secs(cli.timeout_secs);
            
            match timeout(timeout_duration, manager.run(request)).await {
                Ok(Ok(response)) => {
                    println!("{}", response.content);
                }
                Ok(Err(e)) => {
                    eprintln!("❌ Engine error: {}", e);
                    eprintln!();
                    eprintln!("💡 Troubleshooting:");
                    eprintln!("  1. Check if API key is set: echo $MOONSHOT_API_KEY");
                    eprintln!("  2. Verify config file: cat ~/.opencode.json");
                    eprintln!("  3. Test with simple prompt: de run -p 'hi'");
                    eprintln!();
                    eprintln!("📚 Documentation: https://github.com/walkzzz/Dual-Engine#readme");
                    std::process::exit(1);
                }
                Err(_) => {
                    eprintln!("❌ Request timeout ({} seconds)", cli.timeout_secs);
                    eprintln!();
                    eprintln!("💡 Solutions:");
                    eprintln!("  • Increase timeout: de run -p 'msg' --timeout-secs 300");
                    eprintln!("  • Check network connection");
                    eprintln!("  • Verify API key is valid");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Switch { engine }) => {
            match engine.as_str() {
                "opencode" => manager.switch(EngineType::OpenCode).await?,
                "claude" => manager.switch(EngineType::Claude).await?,
                _ => {
                    eprintln!("❌ Unknown engine: {}", engine);
                    eprintln!();
                    eprintln!("Available engines: opencode, claude");
                    eprintln!("💡 Usage: de switch <engine>");
                    std::process::exit(1);
                }
            }
            println!("✅ Switched to engine: {}", engine);
        }
        Some(Commands::Status) => {
            println!("🔧 Current engine: {:?}", manager.current().await);
            println!("📦 Available engines: {:?}", manager.list_engines().await);
            println!("📁 OpenCode path: {:?}", cli.opencode_path);
            println!("📁 Claude path: {:?}", cli.claude_path);
        }
        Some(Commands::Setup) => {
            println!("🚀 Dual-Engine Setup Wizard");
            println!("============================");
            println!();
            println!("Step 1: Choose AI Provider");
            println!("  1. MoonShot (Kimi) - Recommended for Chinese users");
            println!("  2. DashScope (通义千问)");
            println!("  3. Groq - Fast inference");
            println!();
            println!("Step 2: Set API Key");
            println!("  Windows PowerShell:");
            println!("    $env:MOONSHOT_API_KEY=\"sk-your-key\"");
            println!();
            println!("  Linux/Mac:");
            println!("    export MOONSHOT_API_KEY=\"sk-your-key\"");
            println!();
            println!("Step 3: Test Connection");
            println!("  de run -p \"hello\"");
            println!();
            println!("📚 Full documentation: https://github.com/walkzzz/Dual-Engine");
        }
        None => {
            println!("Dual-Engine v0.2.0 - AI 编程助手双引擎");
            println!("");
            println!("🚀 Quick Start:");
            println!("  de run -p 'msg'              - Run prompt");
            println!("  de setup                      - Interactive setup wizard");
            println!("  de switch <engine>            - Switch engine");
            println!("  de status                     - Show status");
            println!("");
            println!("⚙️  Options:");
            println!("  --timeout-secs <SECS>         - Request timeout (default: 120)");
            println!("  --opencode-path <PATH>        - Set OpenCode path");
            println!("  --claude-path <PATH>          - Set Claude path");
            println!("");
            println!("📚 Documentation: https://github.com/walkzzz/Dual-Engine#readme");
        }
    }

    Ok(())
}