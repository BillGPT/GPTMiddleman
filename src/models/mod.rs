// src/models/mod.rs
pub mod openai;
pub mod storage;
pub mod vector_search;

pub use openai::ChatRequest;
pub use openai::{ChatResponse, ChatResponseStream};
pub use openai::{process_chat_request, process_chat_stream_request};

pub use openai::EmbeddingsRequest;
pub use openai::EmbeddingsResponse;
pub use openai::EmbeddingsData;
pub use openai::process_embedding_request;
pub use openai::SimilarityResult;

pub use storage::MEMORY_CACHE;
pub use storage::{
    create_connection, 
    process_create_user_request, 
    process_update_user_request, 
    process_delete_users_request, 
    process_read_user_request,
    process_decrement_user_request
};

pub use storage::{
    CreateUserRequest, CreateUserResponse, 
    ReadUsersResponse, 
    UpdateUserRequest, UpdateUserResponse, 
    DeleteUsersRequest, DeleteUsersResponse, 
    ReadUserRequest, ReadUserResponse,
    DecrementUserRequest, DecrementUserResponse,
};
pub use storage::process_read_users_request;

pub use vector_search::top_n_similar;