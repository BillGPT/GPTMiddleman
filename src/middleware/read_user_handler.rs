// src/middleware/read_user_handler.rs
use hyper::{Body, Request};
use crate::models::create_connection;
use crate::models::{ReadUserRequest, ReadUserResponse};
use crate::models::process_read_user_request;
use bytes::Bytes;

pub async fn read_user_handler(req: Request<Body>) -> Result<String, Box<dyn std::error::Error>> {
    // Parse request body into UpdateUserRequest
    let request: ReadUserRequest = parse_read_user_request(req.into_body()).await?;

    // Create a connection
    let conn = create_connection().map_err(|e| format!("Failed to create connection: {}", e))?;

    // Process CreateUserRequest and get CreateUserResponse
    let response: ReadUserResponse = process_read_user_request(&conn, request).map_err(|e| format!("Failed to process create user request: {}", e))?;

    // Format CreateUserResponse into JSON string
    let response_string = format_request(&response).await.unwrap_or_else(|_| String::from("Error formatting request"));

    Ok(response_string)
}

pub async fn format_request(response: &ReadUserResponse) -> serde_json::Result<String> {
    let json_string = serde_json::to_string_pretty(response)?;
    Ok(json_string)
}

async fn parse_read_user_request(body: Body) -> Result<ReadUserRequest, String> {
    let body_bytes: Bytes = hyper::body::to_bytes(body).await.map_err(|_| String::from("Bad request"))?;
    let request: ReadUserRequest = serde_json::from_slice(&body_bytes).map_err(|_| String::from("Bad request"))?;
    Ok(request)
}