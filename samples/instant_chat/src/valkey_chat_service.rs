use std::pin::Pin;

use crate::stub::instant_chat_server::InstantChat;
use crate::stub::{ClientMessage, ServerMessage, Type};
use crate::valkey_repository::{ChannelMessage, FromChannelMessage, ValkeyRepository};
use anyhow::Result;
use futures::Stream;
use tokio::task;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tonic::metadata::MetadataMap;
use tonic::{Request, Status, Streaming};
use tracing::{debug, error};

#[allow(dead_code)]
pub struct ValkeyChatService {
    shutdown: CancellationToken,
    repository: ValkeyRepository,
}

#[derive(Debug, Clone, PartialEq)]
struct ChatMetadata {
    username: String,
    chatroom: String,
}

impl TryFrom<&MetadataMap> for ChatMetadata {
    type Error = Status;

    fn try_from(m: &MetadataMap) -> std::result::Result<Self, Self::Error> {
        let username = m
            .get("username")
            .ok_or(Status::invalid_argument("no username in metadata"))
            .and_then(|username| {
                username.to_str().map(|str| str.to_owned()).map_err(|_| {
                    Status::invalid_argument("failed to get username(string) from metadata")
                })
            })?;
        let chatroom = m
            .get("chatroom")
            .ok_or(Status::invalid_argument("no chatroom in metadata"))
            .and_then(|username| {
                username.to_str().map(|str| str.to_owned()).map_err(|_| {
                    Status::invalid_argument("failed to get chatroom(string) from metadata")
                })
            })?;
        Ok(ChatMetadata { username, chatroom })
    }
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
        let meta: ChatMetadata = request.metadata().try_into()?;
        let chat_token = self.shutdown.child_token();

        // listen to chatroom channel
        let rx = self
            .repository
            .subscribe::<Result<ServerMessage, Status>>(&meta.chatroom, chat_token.clone())
            .await
            .map_err(|err| tonic::Status::internal(format!("failed to subscribe: {:?}", err)))?;
        let output_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

        let mut inbound = request.into_inner();
        let mut channel = self.repository.get_channel(&meta.chatroom);
        let handle_client_message_task = async move {
            let connect_message = ChannelMessage {
                username: "(System)".into(),
                content: format!("user {} connected", &meta.username),
            };
            let _ = channel.publish(&connect_message).await;
            debug!(
                username = &meta.username,
                chatroom = &meta.chatroom,
                "user connected to chatroom"
            );
            loop {
                tokio::select! {
                    req = inbound.next() => {
                         match req {
                            Some(Ok(req)) => {
                            let channel_message = ChannelMessage {
                                username: meta.username.clone(),
                                content: req.content,
                            };

                            let _ = channel.publish(&channel_message).await;
                            },
                            Some(Err(status)) => {
                                error!(code = ?status.code(), message = ?status.message(), "user connection error");
                                break;
                            },
                            None => {
                                break;
                            },
                        }
                    },
                    _ = chat_token.cancelled() => {
                        break;
                    },
                }
            }

            chat_token.cancel();
            let disconnect_message = ChannelMessage {
                username: "(System)".into(),
                content: format!("user {} disconnected", &meta.username),
            };
            let _ = channel.publish(&disconnect_message).await;
            debug!(
                username = &meta.username,
                chatroom = &meta.chatroom,
                "user disconnected from chatroom"
            );
        };
        task::spawn(handle_client_message_task);

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
