// src/models/openai/mod.rs
pub mod data_types;
pub mod chat_model;
pub mod embeddings_model;

pub use data_types::{
    ChatRequest,
    ChatResponse,
    ChatResponseStream,
    EmbeddingsRequest,
    EmbeddingsResponse,
    EmbeddingsData,
    SimilarityResult
};
pub use chat_model::{process_chat_request, process_chat_stream_request};
pub use embeddings_model::{process_embedding_request};