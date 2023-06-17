// src/handlers/delete_users.rs
use hyper::{Body, Request, Response};
use crate::middleware::delete_users_handler;

pub async fn handle_delete_user(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let request_string = delete_users_handler(req).await.unwrap_or_else(|e| format!("Error processing request: {}", e));
    Ok(Response::new(Body::from(request_string)))
}