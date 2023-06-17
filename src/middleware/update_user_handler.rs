// src/middleware/update_user_handler.rs
use bytes::Bytes;
use hyper::{Body, Request};
use crate::models::{create_connection, process_update_user_request};
use crate::models::{UpdateUserRequest, UpdateUserResponse};

pub async fn update_user_handler(req: Request<Body>) -> Result<String, Box<dyn std::error::Error>> {
    // Parse request body into UpdateUserRequest
    let request: UpdateUserRequest = parse_update_user_request(req.into_body()).await?;
    // Create a connection
    let conn = create_connection().map_err(|e| format!("Failed to create connection: {}", e))?;

    // Process UpdateUserRequest and get UpdateUserResponse
    let response: UpdateUserResponse = process_update_user_request(&conn, request).map_err(|e| format!("Failed to process create user request: {}", e))?;

    // Format UpdateUserResponse into JSON string
    let response_string = format_request(&response).await.unwrap_or_else(|_| String::from("Error formatting request"));

    Ok(response_string)
}

pub async fn format_request(response: &UpdateUserResponse) -> serde_json::Result<String> {
    let json_string = serde_json::to_string_pretty(response)?;
    Ok(json_string)
}

async fn parse_update_user_request(body: Body) -> Result<UpdateUserRequest, String> {
    let body_bytes: Bytes = hyper::body::to_bytes(body).await.map_err(|_| String::from("Bad request"))?;
    let request: UpdateUserRequest = serde_json::from_slice(&body_bytes).map_err(|_| String::from("Bad request"))?;
    Ok(request)
}