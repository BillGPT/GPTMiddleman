// src/models/openai/data_types/embeddings_response.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingsResponse {
    pub object: String,
    pub data: Vec<EmbeddingsData>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingsData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

// // impl实现对Embeddings的Vec中元素的数量统计
// impl EmbeddingsResponse {
//     pub fn embeddings_len(&self) -> usize {
//         let a = self.data[0].embedding.len();
//         a
//     }
// }