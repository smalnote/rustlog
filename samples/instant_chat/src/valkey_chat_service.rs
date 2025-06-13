use std::pin::Pin;

use crate::stub::instant_chat_server::InstantChat;
use crate::stub::{ClientMessage, ServerMessage, Type};
use crate::valkey_repository::{ChannelMessage, FromChannelMessage, ValkeyRepository};
use anyhow::Result;
use futures::Stream;
use tokio::task;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tonic::{Request, Status, Streaming};

#[allow(dead_code)]
pub struct ValkeyChatService {
    shutdown: CancellationToken,
    repository: ValkeyRepository,
}

impl ValkeyChatService {
    pub async fn new(valkey_url: &str, shutdown: CancellationToken) -> Result<Self> {
        let repository = ValkeyRepository::new(valkey_url).await?;
        let service = ValkeyChatService {
            shutdown,
            repository,
        };
        Ok(service)
    }
}

#[tonic::async_trait]
impl InstantChat for ValkeyChatService {
    type ChatStream = Pin<Box<dyn Stream<Item = Result<ServerMessage, Status>> + Send + 'static>>;

    async fn chat(
        &self,
        request: Request<Streaming<ClientMessage>>,
    ) -> Result<tonic::Response<Self::ChatStream>, tonic::Status> {
        // extract username from metadata
        let username = request
            .metadata()
            .get("username")
            .ok_or(Status::invalid_argument("no username in metadata"))
            .and_then(|username| {
                username.to_str().map(|str| str.to_owned()).map_err(|_| {
                    Status::invalid_argument("failed to get username(string) from metadata")
                })
            })?;

        let rx = self
            .repository
            .subscribe::<Result<ServerMessage, Status>>("chatroom", self.shutdown.clone())
            .await
            .map_err(|err| tonic::Status::internal(format!("failed to subscribe: {:?}", err)))?;
        let output_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

        let mut inbound = request.into_inner();
        let mut channel = self.repository.get_channel("chatroom");
        let shutdown_token = self.shutdown.clone();
        task::spawn(async move {
            let username = username.clone();
            let connect_message = ChannelMessage {
                username: "(System)".into(),
                content: format!("user {} connected", username),
            };
            let _ = channel.publish(&connect_message).await;
            loop {
                tokio::select! {
                    req = inbound.next() => {
                         match req {
                            Some(Ok(req)) => {
                            let channel_message = ChannelMessage {
                                username: username.clone(),
                                content: req.content,
                            };

                            let _ = channel.publish(&channel_message).await;
                            },
                            Some(Err(status)) => {
                                println!("user connection error: {} {}", status.code(), status.message());
                                break;
                            },
                            None => {
                                break;
                            },
                        }
                    },
                    _ = shutdown_token.cancelled() => {
                        break;
                    },
                }
            }

            let disconnect_message = ChannelMessage {
                username: "(System)".into(),
                content: format!("user {} disconnected", username),
            };
            let _ = channel.publish(&disconnect_message).await;
            println!("user {} disconnected", username);
        });

        Ok(tonic::Response::new(Box::pin(output_stream)))
    }
}

impl FromChannelMessage for Result<ServerMessage, Status> {
    fn from(channel_message: Result<ChannelMessage>) -> Result<ServerMessage, Status> {
        channel_message
            .map(|m| ServerMessage {
                r#type: Type::Message.into(),
                username: m.username,
                content: m.content,
                at: None,
            })
            .map_err(|err| {
                Status::data_loss(format!("extract message from repository failed: {}", err))
            })
    }
}
