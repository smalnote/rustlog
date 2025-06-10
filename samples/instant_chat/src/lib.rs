pub mod valkey_chat_service;
pub mod valkey_repository;

pub mod instantchat {
    tonic::include_proto!("instantchat.v1");
}
