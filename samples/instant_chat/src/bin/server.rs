use clap::Parser;
use instant_chat::stub::instant_chat_server::InstantChatServer;
use instant_chat::valkey_chat_service::ValkeyChatService;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

/// InstantChat server
#[derive(Parser, Debug)]
#[command(name = "instantchat-server", author, version, about)]
struct Args {
    /// Address to bind to, e.g. [::1]:50051
    #[arg(long, default_value = "[::1]:50051")]
    addr: String,

    /// Valkey/Redis host:port, e.g. 127.0.0.1:6379
    #[arg(long, default_value = "127.0.0.1:6379")]
    valkey_addr: String,

    /// Valkey/Redis password
    #[arg(long)]
    valkey_password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let addr = args.addr.parse()?;

    let shutdown_token = CancellationToken::new();

    // URL form: redist://:password@host:port/?option=value
    let valkey_url = format!(
        "redis://:{}@{}/?protocol=resp3",
        &args.valkey_password, &args.valkey_addr
    );
    let chat_service = ValkeyChatService::new(&valkey_url, shutdown_token.clone()).await?;

    println!("InstantChatServer listening on {}", addr);

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(instant_chat::stub::INSTANTCHAT_DESCRIPTOR)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(InstantChatServer::new(chat_service))
        .add_service(reflection_service)
        .serve_with_shutdown(addr, async {
            signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C handler");
            println!("Shutdown signal received.");
            shutdown_token.cancel();
        })
        .await?;

    Ok(())
}
