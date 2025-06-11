# Instant Chat Room

多人实时聊天室.

## Tech Stack

- Tokio
- gRPC
- Tonic
- Valkey(publish/subscribe stream)

## TODO List

- Support username(passed by gRPC header) [OK]
- Connect/Disconnect message
- Multiple chat room
- Rusty refactor(implements Rust std lib if possible: From, Iterator, etc)
