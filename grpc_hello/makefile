run_server:
	cargo run --package grpc_hello --bin server

grpcurl_list:
	grpcurl -v -plaintext localhost:50051 list

grpcurl_sayhello:
	grpcurl -plaintext -import-path ./proto/helloworld -proto helloworld.proto localhost:50051 helloworld.Greeter/SayHello

run_client:
	cargo run --package grpc_hello --bin client
