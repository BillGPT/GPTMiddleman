// src/handlers/create_user.rs
use hyper::{Body, Request, Response};
use crate::middleware::create_user_handler;

pub async fn handle_create_user(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_string = create_user_handler(req).await.unwrap_or_else(|e| format!("Error processing request: {}", e));
    Ok(Response::new(Body::from(request_string)))
}