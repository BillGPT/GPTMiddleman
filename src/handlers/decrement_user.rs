// src/handlers/decrement_user.rs
use hyper::{Body, Request, Response};
use crate::middleware::decrement_user_handler;

pub async fn handle_decrement_user(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_string: String = decrement_user_handler(req).await.unwrap_or_else(|e| format!("Error processing request: {}", e));
    Ok(Response::new(Body::from(request_string)))
}