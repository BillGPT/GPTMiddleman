// src/handlers/mod.rs
pub mod chat;
pub mod embeddings;
pub mod chat_json;
pub mod create_user;
pub mod read_users;
pub mod update_user;
pub mod delete_users;
pub mod read_user;
pub mod decrement_user;

pub use chat::{handle_chat, handle_stream_chat};
pub use embeddings::handle_embeddings;
pub use chat_json::{handle_chat_json, handle_stream_chat_json};
pub use create_user::handle_create_user;
pub use read_users::handle_read_users;
pub use update_user::handle_update_user;
pub use delete_users::handle_delete_user;
pub use read_user::handle_read_user;
pub use decrement_user::handle_decrement_user;