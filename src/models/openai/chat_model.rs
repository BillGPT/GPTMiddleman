// src/models/openai/chat_model.rs
// use serde_json::Result;
use reqwest;
use serde_json::json;
use super::{
    ChatRequest,
    ChatResponse,
    ChatResponseStream
};

pub async fn process_chat_request(request: ChatRequest) -> Result<ChatResponse, reqwest::Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key: String = crate::middleware::get_env_var("OPENAI_API_KEY");

    let client = reqwest::Client::new();
    let body = json!(request);
    println!("{}", body);

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    let response_text = response.text().await?;
    // 解析JSON字符串为ChatResponse结构体
    let response_json: ChatResponse = serde_json::from_str(&response_text).unwrap();
    println!("{:?}", response_json);

    Ok(response_json)
}

// Stream
use futures::stream::TryStreamExt; // 导入 TryStreamExt 以便使用 try_concat

pub async fn process_chat_stream_request(mut request: ChatRequest) -> Result<ChatResponseStream, reqwest::Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key: String = crate::middleware::get_env_var("OPENAI_API_KEY");

    let client = reqwest::Client::new();
    // 注意：这里发送的请求将 `stream` 参数设置为 `true`
    request.stream = true;
    let body = json!(request);
    println!("{}", body);

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    let response_stream = response
        .bytes_stream()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .into_async_read();

    Ok(ChatResponseStream { body: Box::new(response_stream) })
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_make_api_request() {
//         let mut request = ChatRequest::new("gpt-3.5-turbo".to_string(), Vec::new());
//         request.add_message("user".to_string(), "Hello!".to_string());
//         make_api_request(request).await.unwrap();
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use futures::AsyncReadExt; // Import AsyncReadExt for read_to_string method

//     #[tokio::test]
//     async fn test_process_chat_stream_request() {
//         let mut request = ChatRequest::new("gpt-3.5-turbo".to_string(), Vec::new());
//         request.add_message("user".to_string(), "Hello!".to_string());
        
//         let result = process_chat_stream_request(request).await;
//         match result {
//             Ok(mut chat_response_stream) => {
//                 let mut buffer = String::new();
//                 // Async read from the response stream to a string
//                 if let Err(e) = chat_response_stream.body.read_to_string(&mut buffer).await {
//                     panic!("Failed to read from stream: {}", e);
//                 }
//                 println!("Stream content: {}", buffer);
//             }
//             Err(e) => panic!("Error calling process_chat_stream_request: {}", e),
//         }
//     }
// }
