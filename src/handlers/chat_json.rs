// src/handlers/chat.rs
use hyper::{Body, Request, Response};
use crate::middleware::chat_json_request_handler;
use hyper::StatusCode;
use crate::middleware::stream_chat_json_request_handler;

pub async fn handle_chat_json(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_string = chat_json_request_handler(req).await.unwrap_or_else(|e| format!("Error processing request: {}", e));
    Ok(Response::new(Body::from(request_string)))
}

pub async fn handle_stream_chat_json(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match stream_chat_json_request_handler(req).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Error processing stream request: {}", e)))
                    .unwrap())
    }
}