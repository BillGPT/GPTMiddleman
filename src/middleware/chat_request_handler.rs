// src/middleware/chat_request_handler.rs
use bytes::Bytes;
use hyper::{Body, Request, Response};
use crate::models::{ChatRequest, ChatResponse, ChatResponseStream, process_chat_request, process_chat_stream_request};
use futures::AsyncReadExt;

pub async fn chat_request_handler(req: Request<Body>) -> Result<String, Box<dyn std::error::Error>> {
    // 将请求体解析为ChatRequest。
    let request = parse_request(req.into_body()).await?;

    // 处理ChatRequest并返回ChatResponse。
    let response = process_chat_request(request).await.unwrap();

    // 将ChatResponse转换为JSON字符串。
    let request_string = format_request(&response).await.unwrap_or_else(|_| String::from("Error formatting request"));

    Ok(request_string)
}

pub async fn stream_chat_request_handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    // 将请求体解析为ChatRequest。
    let request = parse_request(req.into_body()).await?;

    // 处理ChatRequest并返回ChatResponseStream。
    let ChatResponseStream { body } = process_chat_stream_request(request).await.unwrap();

    // 从 AsyncRead 转换为 Stream
    let stream = futures::stream::unfold(body, |mut body| async {
        let mut buf = vec![0; 4096];
        match body.read(&mut buf).await {
            Ok(0) => None,
            Ok(n) => Some((Result::Ok::<Vec<u8>, std::io::Error>(buf[..n].to_vec()), body)),
            Err(e) => Some((Result::Err(e), body)),
        }
    });

    // 转换为hyper::Body，并包装到Response中
    let body_stream = Body::wrap_stream(stream);
    let response = Response::new(body_stream);

    Ok(response)
}

pub async fn format_request(request: &ChatResponse) -> serde_json::Result<String> {
    let json_string = serde_json::to_string_pretty(request)?;
    Ok(json_string)
}

async fn parse_request(body: Body) -> Result<ChatRequest, String> {
    let body_bytes: Bytes = hyper::body::to_bytes(body).await.map_err(|_| String::from("Bad request"))?;
    let request: ChatRequest = serde_json::from_slice(&body_bytes).map_err(|_| String::from("Bad request"))?;
    Ok(request)
}