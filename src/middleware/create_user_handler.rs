// src/middleware/create_user_handler.rs
use bytes::Bytes;
use hyper::{Body, Request};
use crate::models::{create_connection, process_create_user_request};
use crate::models::{CreateUserRequest, CreateUserResponse};

pub async fn create_user_handler(req: Request<Body>) -> Result<String, Box<dyn std::error::Error>> {
    // Parse request body into CreateUserRequest
    let request: CreateUserRequest = parse_create_user_request(req.into_body()).await?;

    // Create a connection
    let conn = create_connection().map_err(|e| format!("Failed to create connection: {}", e))?;

    // Process CreateUserRequest and get CreateUserResponse
    let response: CreateUserResponse = process_create_user_request(&conn, request).map_err(|e| format!("Failed to process create user request: {}", e))?;
    println!("response: {:?}", response);

    // Format CreateUserResponse into JSON string
    let response_string = format_request(&response).await.unwrap_or_else(|_| String::from("Error formatting request"));

    Ok(response_string)
}

pub async fn format_request(response: &CreateUserResponse) -> serde_json::Result<String> {
    let json_string = serde_json::to_string_pretty(response)?;
    Ok(json_string)
}

async fn parse_create_user_request(body: Body) -> Result<CreateUserRequest, String> {
    let body_bytes: Bytes = hyper::body::to_bytes(body).await.map_err(|_| String::from("Bad request"))?;
    let request: CreateUserRequest = serde_json::from_slice(&body_bytes).map_err(|_| String::from("Bad request"))?;
    Ok(request)
}