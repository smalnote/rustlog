use clap::Parser;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode},
};
use regex::Regex;
use std::{io, time::Duration};
use tokio::{sync::mpsc, task};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, metadata::MetadataValue, transport::Uri};
use tui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use instant_chat::stub::{ClientMessage, Type, instant_chat_client::InstantChatClient};

/// InstantChat client
#[derive(Parser, Debug)]
#[command(name = "instantchat-client", author, version, about)]
struct Args {
    /// Address to bind to, e.g. [::1]:50051
    #[arg(long, default_value = "http://[::1]:50051")]
    addr: String,

    #[arg(long, value_parser = validate_username)]
    username: String,
}
/// 用户名只能是字母、数字、下划线，3~32 个字符
fn validate_username(s: &str) -> Result<String, String> {
    let re = Regex::new(r"^[a-zA-Z0-9_]{3,32}$").unwrap();
    if re.is_match(s) {
        Ok(s.to_string())
    } else {
        Err("Username must be length of 3~32, composite of alphanum and underscore".to_string())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _: Uri = args.addr.parse()?;

    let mut client = InstantChatClient::connect(args.addr).await?;

    let (to_server_tx, to_server_rx) = mpsc::channel::<ClientMessage>(32);
    let outbound = ReceiverStream::new(to_server_rx);
    let mut chat_request = Request::new(outbound);
    chat_request
        .metadata_mut()
        .insert("username", MetadataValue::try_from(&args.username)?);
    let mut response = client.chat(chat_request).await?.into_inner();

    let (ui_msg_tx, mut ui_msg_rx) = mpsc::channel::<String>(128);
    let (input_tx, mut input_rx) = mpsc::channel::<String>(32);
    let (quit_tx, mut quit_rx) = mpsc::channel::<()>(1);

    // 接收任务
    {
        let ui_msg_tx = ui_msg_tx.clone();
        task::spawn(async move {
            while let Ok(Some(reply)) = response.message().await {
                if !args.username.eq(&reply.username) {
                    let _ = ui_msg_tx
                        .send(format!("{}: {}", reply.username, reply.content))
                        .await;
                }
            }
        });
    }

    // 输入任务
    {
        // let ui_msg_tx = ui_msg_tx.clone();
        let input_tx = input_tx.clone();
        let quit_tx = quit_tx.clone();
        task::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        match key.code {
                            KeyCode::Enter => {
                                input_tx.send("<ENTER>".into()).await.ok();
                            }
                            KeyCode::Char(c) => {
                                input_tx.send(c.to_string()).await.ok();
                            }
                            KeyCode::Backspace => {
                                input_tx.send("<BACKSPACE>".into()).await.ok();
                            }
                            KeyCode::Esc => {
                                quit_tx.send(()).await.ok();
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    let mut ui = Ui::new()?;
    let mut messages = vec![];
    let mut input_buffer = String::new();
    ui.draw(&messages, &input_buffer)?;
    loop {
        tokio::select! {
            Some(msg) = ui_msg_rx.recv() => {
                messages.push(msg);
            }
            Some(input) = input_rx.recv() => {
                match input.as_str() {
                    "<ENTER>" => {
                        if !input_buffer.trim().is_empty() {
                            messages.push(format!("You: {}", input_buffer));
                            let chat_request = ClientMessage {
                                r#type: Type::Message.into(),
                                content: input_buffer.clone(),
                                at: None,
                            };
                            to_server_tx.send(chat_request).await.ok();
                            input_buffer.clear();
                        }
                    }
                    "<BACKSPACE>" => { input_buffer.pop(); }
                    _ => input_buffer.push_str(&input),
                }
            }
            _ = quit_rx.recv() => break,
        }
        ui.draw(&messages, &input_buffer)?;
    }

    // 清理
    ui.cleanup()
}

pub struct Ui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Ui {
    pub fn new() -> anyhow::Result<Self> {
        // 启动 TUI
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, messages: &[String], input: &str) -> anyhow::Result<()> {
        self.terminal.draw(|f| {
            Self::render_ui(f, messages, input);
        })?;
        Ok(())
    }

    fn render_ui<B: tui::backend::Backend>(f: &mut Frame<B>, messages: &[String], input: &str) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(f.size());

        let items: Vec<ListItem> = messages.iter().map(|m| ListItem::new(m.as_str())).collect();

        let message_list =
            List::new(items).block(Block::default().borders(Borders::ALL).title("Chat"));
        f.render_widget(message_list, chunks[0]);

        let input_box =
            Paragraph::new(input).block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input_box, chunks[1]);

        f.set_cursor(chunks[1].x + input.len() as u16 + 1, chunks[1].y + 1);
    }

    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        self.terminal.backend_mut().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
