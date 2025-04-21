use std::error::Error;

use grpc_hello::Timestamp;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("helloworld_descriptor");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let now = chrono::Utc::now();
        println!(
            "REQUEST@{:?} Got a request from {:?}",
            now,
            request.remote_addr()
        );

        let reply = HelloReply {
            message: format!("Hello, {}!", request.into_inner().name),
            at: Some(Timestamp::from(now).0),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
