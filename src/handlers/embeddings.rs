// src/handlers/embeddings.rs
use hyper::{Body, Request, Response};
use crate::middleware::embedding_req_process;

pub async fn handle_embeddings(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_string = embedding_req_process(req).await.unwrap_or_else(|e| format!("Error processing request: {}", e));
    Ok(Response::new(Body::from(request_string)))
}