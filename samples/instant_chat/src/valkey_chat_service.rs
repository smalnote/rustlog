use std::pin::Pin;

use crate::instantchat::instant_chat_service_server::InstantChatService;
use crate::instantchat::{ChatRequest, ChatResponse};
use crate::valkey_repository::{FromPayload, ValkeyRepository};
use futures::Stream;
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

#[tonic::async_trait]
impl InstantChatService for ValkeyChatService {
    type ChatStream = Pin<Box<dyn Stream<Item = Result<ChatResponse, Status>> + Send + 'static>>;

    async fn chat(
        &self,
        request: Request<Streaming<ChatRequest>>,
    ) -> Result<tonic::Response<Self::ChatStream>, tonic::Status> {
        let mut inbound = request.into_inner();

        let mut channel = self.repository.get_channel("chatroom");
        task::spawn(async move {
            while let Some(req) = inbound.next().await {
                if let Ok(req) = req {
                    let _ = channel.publish(&req.content).await;
                }
            }
        });

        let rx = self
            .repository
            .subscribe::<Result<ChatResponse, Status>>("chatroom")
            .await
            .map_err(|err| tonic::Status::internal(format!("failed to subscribe: {:?}", err)))?;
        let output_stream = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);
        Ok(tonic::Response::new(Box::pin(output_stream)))
    }
}

impl FromPayload for Result<ChatResponse, Status> {
    fn from_payload(payload: String) -> Result<ChatResponse, Status> {
        Ok(ChatResponse {
            r#type: 1,
            content: payload,
            at: None,
        })
    }
}
