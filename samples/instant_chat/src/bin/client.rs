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
use tokio_util::sync::CancellationToken;
use tonic::{
    Request,
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig, Uri},
};
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

    #[arg(long, value_parser = validate_name)]
    username: String,

    #[arg(long, value_parser = validate_name, default_value = "public")]
    chatroom: String,

    #[arg(long, help = "TLS CA file")]
    tls_ca: String,
}
/// 用户名只能是字母、数字、下划线，3~32 个字符
fn validate_name(s: &str) -> Result<String, String> {
    let re = Regex::new(r"^[a-zA-Z0-9_]{3,32}$").unwrap();
    if re.is_match(s) {
        Ok(s.to_string())
    } else {
        Err("must be length of 3~32, composite of alphanum and underscore".to_string())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let addr: Uri = args.addr.parse()?;

    let domain = addr
        .host()
        .ok_or("no domain name in addr")
        .map_err(|err| anyhow::format_err!("{err}"))?;
    let ca_cert = tokio::fs::read(args.tls_ca).await?;
    let ca = Certificate::from_pem(ca_cert);
    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name(domain);

    let channel = Channel::builder(addr).tls_config(tls)?.connect().await?;
    let mut client = InstantChatClient::new(channel);

    let (to_server_tx, to_server_rx) = mpsc::channel::<ClientMessage>(32);
    let outbound = ReceiverStream::new(to_server_rx);
    let mut chat_request = Request::new(outbound);
    let metadata = chat_request.metadata_mut();
    metadata.insert("username", MetadataValue::try_from(&args.username)?);
    metadata.insert("chatroom", MetadataValue::try_from(&args.chatroom)?);
    let mut response_stream = client.chat(chat_request).await?.into_inner();

    let (input_tx, mut input_rx) = mpsc::channel::<String>(32);
    let quit_token = CancellationToken::new();

    // 输入任务
    {
        let quit_token = quit_token.clone();
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
                                quit_token.cancel();
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                if quit_token.is_cancelled() {
                    break;
                }
            }
        });
    }

    let mut ui = Ui::new(&args.chatroom)?;
    let mut messages = vec![];
    let mut input_buffer = String::new();
    ui.draw(&messages, &input_buffer)?;
    loop {
        tokio::select! {
            reply = response_stream.message() => {
                match reply {
                    Ok(None) => {
                        quit_token.cancel();
                    },
                    Ok(Some(reply)) => {
                        if !reply.username.eq(&args.username) {
                            messages.push(format!("{}: {}", reply.username, reply.content));
                        }
                    },
                    Err(status) => messages.push(format!("(Server): {}", status)),
                };
            },
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
            },
            _ = quit_token.cancelled() => {
                // send close to server before exit
                drop(to_server_tx);
                break;
            },
        }
        ui.draw(&messages, &input_buffer)?;
    }

    ui.cleanup()
}

pub struct Ui {
    chatroom: String,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Ui {
    pub fn new(chatroom: &str) -> anyhow::Result<Self> {
        // 启动 TUI
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(Self {
            chatroom: chatroom.into(),
            terminal,
        })
    }

    pub fn draw(&mut self, messages: &[String], input: &str) -> anyhow::Result<()> {
        self.terminal.draw(|f| {
            Self::render_ui(f, &self.chatroom, messages, input);
        })?;
        Ok(())
    }

    fn render_ui<B: tui::backend::Backend>(
        f: &mut Frame<B>,
        chatroom: &str,
        messages: &[String],
        input: &str,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(f.size());

        let items: Vec<ListItem> = messages.iter().map(|m| ListItem::new(m.as_str())).collect();

        let message_list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Chatroom: {}", chatroom)),
        );
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
