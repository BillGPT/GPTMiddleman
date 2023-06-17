// src/models/openai/data_types/chat_request.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    model: String,
    pub messages: Vec<Message>,
    #[serde(default = "ChatRequest::default_stream_value")]
    pub stream: bool,
    #[serde(default = "ChatRequest::default_temperature_value")]
    temperature: f32,
    #[serde(default = "ChatRequest::default_top_p_value")]
    top_p: f32,
    #[serde(default = "ChatRequest::default_n_value")]
    n: i32,
    #[serde(default = "ChatRequest::default_max_tokens_value")]
    max_tokens: i32,
    #[serde(default = "ChatRequest::default_presence_penalty_value")]
    presence_penalty: f32,
    #[serde(default = "ChatRequest::default_frequency_penalty_value")]
    frequency_penalty: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    role: String,
    pub content: String,
}

impl ChatRequest {
    fn default_stream_value() -> bool {
        false
    }

    fn default_temperature_value() -> f32 {
        1.0
    }

    fn default_top_p_value() -> f32 {
        1.0
    }

    fn default_n_value() -> i32 {
        1
    }

    fn default_max_tokens_value() -> i32 {
        2048
    }

    fn default_presence_penalty_value() -> f32 {
        0.0
    }

    fn default_frequency_penalty_value() -> f32 {
        0.0
    }

    // pub fn new(model: String, messages: Vec<Message>) -> Self {
    //     ChatRequest {
    //         model,
    //         messages,
    //         stream: Self::default_stream_value(),
    //         temperature: Self::default_temperature_value(),
    //         top_p: Self::default_top_p_value(),
    //         n: Self::default_n_value(),
    //         max_tokens: Self::default_max_tokens_value(),
    //         presence_penalty: Self::default_presence_penalty_value(),
    //         frequency_penalty: Self::default_frequency_penalty_value(),
    //     }
    // }

    // pub fn add_message(&mut self, role: String, content: String) {
    //     let message = Message { role, content };
    //     self.messages.push(message);
    // }
}