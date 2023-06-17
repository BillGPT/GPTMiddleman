// src/models/openai/data_types/embeddings_request.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingsRequest {
    model: String,
    input: String,
}

impl EmbeddingsRequest {
    pub fn new(model: String, input: String) -> EmbeddingsRequest {
        EmbeddingsRequest {
            model,
            input,
        }
    }
}