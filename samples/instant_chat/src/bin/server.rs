use clap::Parser;
use instant_chat::stub::instant_chat_server::InstantChatServer;
use instant_chat::valkey_chat_service::ValkeyChatService;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_reflection::server::Builder;
use tracing::info;
use tracing_subscriber::EnvFilter;

/// InstantChat server
#[derive(Parser, Debug)]
#[command(name = "instantchat-server", author, version, about)]
struct Args {
    /// Address to bind to, e.g. [::1]:50051
    #[arg(long, default_value = "0.0.0.0:50051")]
    addr: String,

    /// Valkey/Redis host:port, e.g. 127.0.0.1:6379
    #[arg(long, default_value = "127.0.0.1:6379")]
    valkey_addr: String,

    /// Valkey/Redis password
    #[arg(long, env = "VALKEY_PASSWORD")]
    valkey_password: String,

    #[arg(long, help = "TLS certificate file")]
    tls_cert: String,

    #[arg(long, help = "TLS key file")]
    tls_key: String,

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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let addr = args.addr.parse()?;

    let cert = tokio::fs::read(args.tls_cert).await?;
    let key = tokio::fs::read(args.tls_key).await?;
    let identity = Identity::from_pem(cert, key);

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

    let shutdown_token = CancellationToken::new();

    // URL form: redist://:password@host:port/?option=value
    let valkey_url = format!(
        "redis://:{}@{}/?protocol=resp3",
        &args.valkey_password, &args.valkey_addr
    );
    let chat_service = ValkeyChatService::new(&valkey_url, shutdown_token.clone()).await?;

    info!(?addr, "starting instant chat server");

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(instant_chat::stub::INSTANTCHAT_DESCRIPTOR)
        .build_v1()
        .unwrap();

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(InstantChatServer::new(chat_service))
        .add_service(reflection_service)
        .serve_with_shutdown(addr, async {
            signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C handler");
            info!("shutting down server ...");
            shutdown_token.cancel();
        })
        .await?;

    Ok(())
}
