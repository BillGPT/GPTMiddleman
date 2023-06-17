// src/models/openai/data_types/embeddings_data.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbeddingsData {
    pub uuid: String,
    pub input: String,
    pub embedding: Vec<f32>,
    pub total_tokens: i32,
}