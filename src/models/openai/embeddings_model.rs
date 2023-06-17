// src/models/openai/embeddings_model.rs
// use serde_json::Result;
use reqwest;
use serde_json::json;
use super::{EmbeddingsRequest, EmbeddingsResponse};

pub async fn process_embedding_request(request: EmbeddingsRequest) -> Result<EmbeddingsResponse, reqwest::Error> {
    let url = "https://api.openai.com/v1/embeddings";
    let api_key: String = crate::middleware::get_env_var("OPENAI_API_KEY");
    
    let client = reqwest::Client::new();
    let body = json!(request);

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    let response_text = response.text().await?;
    // println!("Response: {}", response_text);
    // 解析JSON字符串为ChatResponse结构体
    let response_json: EmbeddingsResponse = serde_json::from_str(&response_text).unwrap();

    Ok(response_json)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_make_api_request() {
//         let request = EmbeddingsRequest::_new("text-embedding-ada-002".to_string(), "The food was delicious and the waiter...".to_string());
//         // println!("{:?}", request);
//         let response = process_embedding_request(request).await.unwrap();
//         println!("{:?}", response._embeddings_len());
//     }
// }