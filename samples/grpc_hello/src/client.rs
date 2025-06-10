use std::error::Error;

use chrono::Utc;
use grpc_hello::Timestamp;
use hello_world::HelloRequest;
use hello_world::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "gRPC".into(),
    });

    let reply = client.say_hello(request).await?.into_inner();
    let message = reply.message.clone();
    let at: chrono::DateTime<Utc> = Timestamp(reply.at.unwrap()).into();

    println!("RESPONSE@{:?}, message={:?}", at, message);

    Ok(())
}
