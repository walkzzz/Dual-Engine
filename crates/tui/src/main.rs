use anyhow::Result;
use ratatui::prelude::Position;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use engine_core::{EngineManager, EngineType};
use engine_claude::create_claude_engine;
use engine_opencode::create_opencode_engine;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use shared_types::{EngineRequest, Message, Role};
use std::io;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

struct App {
    engine_manager: EngineManager,
    input: String,
    output: Vec<String>,
    messages: Vec<Message>,
    is_running: bool,
    current_engine: EngineType,
}

impl App {
    fn new() -> Self {
        Self {
            engine_manager: EngineManager::new(),
            input: String::new(),
            output: Vec::new(),
            messages: Vec::new(),
            is_running: false,
            current_engine: EngineType::OpenCode,
        }
    }

    async fn init(&mut self) -> Result<()> {
        self.engine_manager
            .register(EngineType::OpenCode, create_opencode_engine(None))
            .await;
        self.engine_manager
            .register(EngineType::Claude, create_claude_engine(None))
            .await;
        self.engine_manager
            .select(EngineType::OpenCode)
            .await?;

        self.output
            .push("Dual Engine CLI v0.1.0".to_string());
        self.output.push("Type :help for commands".to_string());
        self.output.push("".to_string());

        Ok(())
    }

    async fn switch_engine(&mut self, engine: EngineType) -> Result<()> {
        self.engine_manager.select(engine).await?;
        self.current_engine = engine;
        self.output
            .push(format!("Switched to {:?} engine", engine));
        Ok(())
    }

    async fn run_query(&mut self) -> Result<()> {
        if self.input.trim().is_empty() {
            return Ok(());
        }

        let input = self.input.clone();
        self.input.clear();

        if input.starts_with(':') {
            return self.handle_command(&input[1..]).await;
        }

        self.output.push(format!("> {}", input));
        self.messages.push(Message {
            role: Role::User,
            content: input.clone(),
            tool_calls: vec![],
            tool_results: vec![],
        });

        let request = EngineRequest {
            messages: self.messages.clone(),
            tools: vec![],
            context: std::collections::HashMap::new(),
        };

        match self.engine_manager.run(request).await {
            Ok(response) => {
                self.output.push(response.content.clone());
                self.messages.push(Message {
                    role: Role::Assistant,
                    content: response.content,
                    tool_calls: vec![],
                    tool_results: vec![],
                });
            }
            Err(e) => {
                self.output.push(format!("Error: {}", e));
            }
        }

        Ok(())
    }

    async fn handle_command(&mut self, cmd: &str) -> Result<()> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts[0] {
            "help" => {
                self.output.push(":opencode  - Switch to OpenCode engine".to_string());
                self.output.push(":claude    - Switch to Claude engine".to_string());
                self.output.push(":status    - Show current engine".to_string());
                self.output.push(":clear     - Clear output".to_string());
            }
            "opencode" => {
                self.switch_engine(EngineType::OpenCode).await?;
            }
            "claude" => {
                self.switch_engine(EngineType::Claude).await?;
            }
            "status" => {
                self.output
                    .push(format!("Current engine: {:?}", self.current_engine));
            }
            "clear" => {
                self.output.clear();
            }
            _ => {
                self.output.push(format!("Unknown command: {}", cmd));
            }
        }
        Ok(())
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new(Line::from(vec![
        Span::raw("Dual Engine - "),
        Span::styled(
            format!("{:?}", app.current_engine),
            Style::default().fg(Color::Cyan),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Engine"));

    frame.render_widget(header, chunks[0]);

    let output: Vec<ListItem> = app
        .output
        .iter()
        .map(|line| ListItem::new(line.clone()))
        .collect();

    let output_widget = List::new(output)
        .block(Block::default().borders(Borders::ALL).title("Output"));

    frame.render_widget(output_widget, chunks[1]);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default().fg(Color::Green));

    frame.render_widget(input, chunks[2]);
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let mut app = App::new();
    app.init().await?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Enter => {
                            app.run_query().await?;
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}