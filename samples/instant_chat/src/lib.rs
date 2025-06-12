pub mod valkey_chat_service;
pub mod valkey_repository;

#[allow(clippy::all, unused_qualifications)]
pub mod stub {
    include!("generated/instant_chat.v1.rs");
    pub const INSTANTCHAT_DESCRIPTOR: &[u8] =
        include_bytes!("generated/instant_chat_descriptor.bin");
}
