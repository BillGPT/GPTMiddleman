// src/middleware/embedding_request_handler.rs
use bytes::Bytes;
use hyper::{Body, Request};
use crate::models::{EmbeddingsResponse, EmbeddingsRequest, process_embedding_request};

pub async fn embedding_req_process(req: Request<Body>) -> Result<String, Box<dyn std::error::Error>> {
    // 将请求体解析为EmbeddingsRequest。
    let request = parse_request(req.into_body()).await?;

    // 处理EmbeddingsRequest并返回EmbeddingsResponse。
    let response = process_embedding_request(request).await.unwrap();

    // 将EmbeddingsResponse转换为JSON字符串。
    let request_string = format_request(&response).await.unwrap_or_else(|_| String::from("Error formatting request"));

    Ok(request_string)
}

pub async fn format_request(request: &EmbeddingsResponse) -> serde_json::Result<String> {
    let json_string = serde_json::to_string_pretty(request)?;
    Ok(json_string)
}

async fn parse_request(body: Body) -> Result<EmbeddingsRequest, String> {
    let body_bytes: Bytes = hyper::body::to_bytes(body).await.map_err(|_| String::from("Bad request"))?;
    let request: EmbeddingsRequest = serde_json::from_slice(&body_bytes).map_err(|_| String::from("Bad request"))?;
    Ok(request)
}