// src/middleware/delete_users_handler.rs
use bytes::Bytes;
use hyper::{Body, Request};
use crate::models::create_connection;
use crate::models::process_delete_users_request;
use crate::models::{DeleteUsersRequest, DeleteUsersResponse};

pub async fn delete_users_handler(req: Request<Body>) -> Result<String, Box<dyn std::error::Error>> {
    // Parse request body into DeleteUsersRequest
    let request: DeleteUsersRequest = parse_delete_users_request(req.into_body()).await?;

    // Create a connection
    let conn = create_connection().map_err(|e| format!("Failed to create connection: {}", e))?;

    // Process DeleteUsersRequest and get DeleteUsersResponse
    let response = process_delete_users_request(&conn, request).map_err(|e| format!("Failed to process create user request: {}", e))?;
    println!("response: {:?}", response);

    // Format DeleteUsersResponse into JSON string
    let response_string = format_request(&response).await.unwrap_or_else(|_| String::from("Error formatting request"));

    Ok(response_string)
}

pub async fn format_request(response: &DeleteUsersResponse) -> serde_json::Result<String> {
    let json_string = serde_json::to_string_pretty(response)?;
    Ok(json_string)
}

async fn parse_delete_users_request(body: Body) -> Result<DeleteUsersRequest, String> {
    let body_bytes: Bytes = hyper::body::to_bytes(body).await.map_err(|_| String::from("Bad request"))?;
    let request: DeleteUsersRequest = serde_json::from_slice(&body_bytes).map_err(|_| String::from("Bad request"))?;
    Ok(request)
}