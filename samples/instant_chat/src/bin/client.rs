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
use tracing::debug;
use tracing_subscriber::EnvFilter;
use tui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use unicode_width::UnicodeWidthStr;

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

    #[arg(
        long,
        env = "RUST_LOG",
        default_value = "error",
        help = "Log level, e.g. info, debug, error, instant_chat=trace"
    )]
    log_level: String,

    #[arg(
        long,
        env = "LOG_JSON",
        help = "Log format in JSON",
        default_value = "false"
    )]
    log_json: bool,

    #[arg(
        long,
        env = "TRAFFIC_TAG",
        default_value = "",
        help = "Traffic tag list passed to server"
    )]
    traffic_tag: Vec<String>,
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
    let env_filter = EnvFilter::new(args.log_level);

    if args.log_json {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(env_filter)
            .with_target(true)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(true)
            .init();
    }

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
    for tag in args.traffic_tag.iter() {
        metadata.append("x-traffic-tag", MetadataValue::try_from(tag)?);
    }
    debug!(args.username, args.chatroom, "starting chat");
    let mut response_stream = client.chat(chat_request).await?.into_inner();
    debug!(args.username, args.chatroom, "chat started");

    let (ui_tx, mut ui_rx) = mpsc::channel::<UiEvent>(32);
    let quit_token = CancellationToken::new();

    // 输入任务
    {
        let quit_token = quit_token.clone();
        task::spawn(async move {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    match event::read().unwrap() {
                        Event::Key(key) => match key.code {
                            KeyCode::Enter => {
                                ui_tx.send(UiEvent::Enter).await.ok();
                            }
                            KeyCode::Char(c) => {
                                ui_tx.send(UiEvent::Char(c)).await.ok();
                            }
                            KeyCode::Backspace => {
                                ui_tx.send(UiEvent::Backspace).await.ok();
                            }
                            KeyCode::Esc => {
                                quit_token.cancel();
                                break;
                            }
                            _ => {}
                        },
                        Event::Resize(_, _) => {
                            ui_tx.send(UiEvent::Resize).await.ok();
                        }
                        _ => {}
                    }
                }
                if quit_token.is_cancelled() {
                    break;
                }
            }
        });
    }

    let mut ui = Ui::new(&args.username, &args.chatroom)?;
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
                    Err(status) => messages.push(format!("(Server): {status}")),
                };
            },
            Some(ui_event) = ui_rx.recv() => {
                match ui_event {
                    UiEvent::Enter => {
                        if !input_buffer.trim().is_empty() {
                            messages.push(format!("You: {input_buffer}"));
                            let chat_request = ClientMessage {
                                r#type: Type::Message.into(),
                                content: input_buffer.clone(),
                                at: None,
                            };
                            to_server_tx.send(chat_request).await.ok();
                            input_buffer.clear();
                        }
                    },
                    UiEvent::Backspace => { input_buffer.pop(); },
                    // leave it to redraw
                    UiEvent::Resize => { },
                    UiEvent::Char(c) => input_buffer.push(c),
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
    username: String,
    chatroom: String,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    list_state: ListState,
}

pub enum UiEvent {
    Enter,
    Backspace,
    Resize,
    Char(char),
}

impl Ui {
    pub fn new(username: &str, chatroom: &str) -> anyhow::Result<Self> {
        // 启动 TUI
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(Self {
            username: username.into(),
            chatroom: chatroom.into(),
            terminal,
            list_state: Default::default(),
        })
    }

    pub fn draw(&mut self, messages: &[String], input: &str) -> anyhow::Result<()> {
        self.list_state
            .select(Some(messages.len().saturating_sub(1)));
        self.terminal.draw(|f| {
            Self::render_ui(
                f,
                &self.username,
                &self.chatroom,
                messages,
                input,
                &mut self.list_state,
            );
        })?;
        Ok(())
    }

    fn render_ui<B: tui::backend::Backend>(
        f: &mut Frame<B>,
        username: &str,
        chatroom: &str,
        messages: &[String],
        input: &str,
        list_state: &mut ListState,
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
                .title(format!("{username}@{chatroom}")),
        );
        f.render_stateful_widget(message_list, chunks[0], list_state);

        let input_box =
            Paragraph::new(input).block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input_box, chunks[1]);

        f.set_cursor(
            chunks[1].x + UnicodeWidthStr::width(input) as u16 + 1,
            chunks[1].y + 1,
        );
    }

    pub fn cleanup(&mut self) -> anyhow::Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        self.terminal.backend_mut().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
