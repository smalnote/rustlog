# Instant Chat Room

多人实时聊天室.

## Tech Stack

- Tokio
- gRPC
- Tonic
- Valkey(publish/subscribe stream)

## TODO List

- Support username(passed by gRPC header) [OK]
- Connect/Disconnect message [OK]
- Named chat room [OK]
- Gracefully shutdown [OK]
- Rusty refactor(implements Rust std lib if possible: From, Iterator, etc) [OK]
- Enable TLS [OK]
- Structured log [OK]

> [!CAUTION]
> Gracefully shutting down tokio::main need to exit all task, or it will stuck.
> Communicate with channel between asynchronous task, instead of mutex.
