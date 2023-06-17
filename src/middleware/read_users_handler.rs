// src/middleware/create_user_handler.rs
use hyper::{Body, Request};
use crate::models::create_connection;
use crate::models::ReadUsersResponse;
use crate::models::process_read_users_request;

pub async fn read_users_handler(_req: Request<Body>) -> Result<String, Box<dyn std::error::Error>> {
    // Create a connection
    let conn = create_connection().map_err(|e| format!("Failed to create connection: {}", e))?;

    // Process CreateUserRequest and get CreateUserResponse
    let response: ReadUsersResponse = process_read_users_request(&conn).map_err(|e| format!("Failed to process create user request: {}", e))?;

    // Format CreateUserResponse into JSON string
    let response_string = format_request(&response).await.unwrap_or_else(|_| String::from("Error formatting request"));

    Ok(response_string)
}

pub async fn format_request(response: &ReadUsersResponse) -> serde_json::Result<String> {
    let json_string = serde_json::to_string_pretty(response)?;
    Ok(json_string)
}