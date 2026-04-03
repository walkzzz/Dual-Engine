use clap::{Parser, Subcommand};
use engine_core::{EngineManager, EngineType};
use engine_claude::create_claude_engine;
use engine_opencode::create_opencode_engine;
use shared_types::{EngineRequest, Message, Role};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();

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
            eprintln!("Unknown engine: {}", cli.engine);
            std::process::exit(1);
        }
    }

    match &cli.command {
        Some(Commands::Run { prompt }) => {
            info!("Running with engine: {}", cli.engine);
            
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

            let response = manager.run(request).await?;
            println!("{}", response.content);
        }
        Some(Commands::Switch { engine }) => {
            match engine.as_str() {
                "opencode" => manager.switch(EngineType::OpenCode).await?,
                "claude" => manager.switch(EngineType::Claude).await?,
                _ => {
                    eprintln!("Unknown engine: {}", engine);
                    std::process::exit(1);
                }
            }
            println!("Switched to engine: {}", engine);
        }
        Some(Commands::Status) => {
            println!("Current engine: {:?}", manager.current().await);
            println!("Available engines: {:?}", manager.list_engines().await);
            println!("OpenCode path: {:?}", cli.opencode_path);
            println!("Claude path: {:?}", cli.claude_path);
        }
        None => {
            println!("Dual Engine v0.1.0");
            println!("");
            println!("Usage:");
            println!("  de                          - Start CLI mode");
            println!("  de run -p 'msg'             - Run prompt");
            println!("  de --opencode-path /path    - Set OpenCode path");
            println!("  de --claude-path /path      - Set Claude path");
            println!("  de switch claude            - Switch engine");
            println!("  de status                   - Show status");
        }
    }

    Ok(())
}