use std::pin::Pin;

use crate::stub::instant_chat_server::InstantChat;
use crate::stub::{ClientMessage, ServerMessage, Type};
use crate::valkey_repository::{FromPayload, ValkeyRepository};
use futures::Stream;
use serde::{Deserialize, Serialize};
use tokio::task;
use tokio_stream::StreamExt;
use tonic::{Request, Status, Streaming};

#[allow(dead_code)]
pub struct ValkeyChatService {
    repository: ValkeyRepository,
}

impl ValkeyChatService {
    pub async fn new(valkey_url: &str) -> Result<Self, redis::RedisError> {
        let repository = ValkeyRepository::new(valkey_url).await?;
        let service = ValkeyChatService { repository };
        Ok(service)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct ChannelMessage {
    username: String,
    content: String,
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
        let mut inbound = request.into_inner();

        let mut channel = self.repository.get_channel("chatroom");
        task::spawn(async move {
            let username = username.clone();
            while let Some(req) = inbound.next().await {
                if let Ok(req) = req {
                    let channel_message = ChannelMessage {
                        username: username.clone(),
                        content: req.content,
                    };

                    let channel_message = serde_json::to_string(&channel_message)
                        .map_err(|_| {
                            Status::internal("failed to serialize channel message to json")
                        })
                        .unwrap();

                    let _ = channel.publish(&channel_message).await;
                }
            }
        });

        let rx = self
            .repository
            .subscribe::<Result<ServerMessage, Status>>("chatroom")
            .await
            .map_err(|err| tonic::Status::internal(format!("failed to subscribe: {:?}", err)))?;
        let output_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
        Ok(tonic::Response::new(Box::pin(output_stream)))
    }
}

impl FromPayload for Result<ServerMessage, Status> {
    fn from_payload(payload: String) -> Result<ServerMessage, Status> {
        let channel_message: ChannelMessage = serde_json::from_str(&payload)
            .map_err(|_| Status::internal("failed to decode playload to channel message"))?;

        Ok(ServerMessage {
            r#type: Type::Message.into(),
            username: channel_message.username,
            content: channel_message.content,
            at: None,
        })
    }
}
