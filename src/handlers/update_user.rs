// src/handlers/update_user.rs
use hyper::{Body, Request, Response};
use crate::middleware::update_user_handler;

pub async fn handle_update_user(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_string = update_user_handler(req).await.unwrap_or_else(|e| format!("Error processing request: {}", e));
    Ok(Response::new(Body::from(request_string)))
}