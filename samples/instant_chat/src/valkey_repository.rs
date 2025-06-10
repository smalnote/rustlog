use std::time::Duration;

use futures::StreamExt;
use redis::{AsyncTypedCommands, Client, RedisError, aio::MultiplexedConnection};
use tokio::sync::mpsc::{self, UnboundedReceiver};

pub struct ValkeyRepository {
    client: Client,
    pub_conn: MultiplexedConnection,
}

type Result<T> = std::result::Result<T, RedisError>;

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
    pub async fn publish(&mut self, message: &str) -> Result<usize> {
        self.pub_conn.publish(self.channel.clone(), message).await
    }
}

pub trait FromPayload: Send + 'static {
    fn from_payload(payload: String) -> Self;
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
    pub async fn subscribe<T>(&self, channel: &str) -> Result<UnboundedReceiver<T>>
    where
        T: FromPayload,
    {
        let mut pubsub = self.client.get_async_pubsub().await?;
        pubsub.subscribe(channel).await?;

        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut on_message = pubsub.on_message();
            while let Some(message) = on_message.next().await {
                match message.get_payload::<String>() {
                    Ok(payload) => {
                        let val = T::from_payload(payload);
                        let _ = tx.send(val);
                    }
                    Err(err) => {
                        eprintln!("Failed to parse payload: {:?}", err);
                    }
                }
            }
        });

        Ok(rx)
    }
}
