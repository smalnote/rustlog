use std::time::Duration;

use anyhow::Result;
use futures::StreamExt;
use redis::{AsyncTypedCommands, Client, aio::MultiplexedConnection};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_util::sync::CancellationToken;

pub struct ValkeyRepository {
    client: Client,
    pub_conn: MultiplexedConnection,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelMessage {
    pub username: String,
    pub content: String,
}

pub struct ChannelPublisher {
    channel: String,
    pub_conn: MultiplexedConnection,
}

impl ChannelPublisher {
    fn new(pub_conn: MultiplexedConnection, channel: &str) -> Self {
        ChannelPublisher {
            channel: channel.into(),
            pub_conn,
        }
    }
    pub async fn publish(&mut self, message: &ChannelMessage) -> Result<usize> {
        let message = serde_json::to_string(message)?;
        self.pub_conn
            .publish(&self.channel, message)
            .await
            .map_err(anyhow::Error::new)
    }
}

pub trait FromChannelMessage: Send + 'static {
    fn from(message: Result<ChannelMessage>) -> Self;
}

impl ValkeyRepository {
    pub async fn new(url: &str) -> Result<Self> {
        let client = redis::Client::open(url)?;
        let pub_conn = client
            .get_multiplexed_tokio_connection_with_response_timeouts(
                Duration::from_secs(3),
                Duration::from_secs(3),
            )
            .await?;
        Ok(Self { client, pub_conn })
    }

    /// 发布消息到频道,返回有多少个订阅者.
    pub fn get_channel(&self, channel: &str) -> ChannelPublisher {
        let pub_conn = self.pub_conn.clone();
        ChannelPublisher::new(pub_conn, channel)
    }

    /// 订阅频道，返回一个 Receiver，外部用异步方式接收消息
    pub async fn subscribe<T>(
        &self,
        channel: &str,
        shutdown: CancellationToken,
    ) -> Result<UnboundedReceiver<T>>
    where
        T: FromChannelMessage,
    {
        let mut pubsub = self.client.get_async_pubsub().await?;
        pubsub.subscribe(channel).await?;

        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut on_message = pubsub.on_message();
            loop {
                tokio::select! {
                    message = on_message.next() => {
                        if let Some(message) = message {
                            match message.get_payload::<String>() {
                                Ok(payload) => {
                                    let channel_message: Result<ChannelMessage> = serde_json::from_str(&payload)
                                        .map_err(anyhow::Error::new);
                                    let _ = tx.send(T::from(channel_message));
                                }
                                Err(err) => {
                                    let _ = tx.send(T::from(Err(anyhow::Error::new(err))));
                                }
                            }
                        } else {
                            break;
                        }
                    },
                    _ = shutdown.cancelled() => {
                        break;
                    },
                }
            }
        });

        Ok(rx)
    }
}
