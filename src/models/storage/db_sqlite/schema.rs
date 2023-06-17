// src/models/storage/db_sqlite/schema.rs
use serde::{Serialize, Deserialize};
use rusqlite::{Connection, Result};

pub fn create_connection() -> Result<Connection> {
    let conn = Connection::open("users.db")?;
    conn.execute_batch(USER_TABLE)?;
    Ok(conn)
}

pub static USER_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    api_token TEXT UNIQUE NOT NULL,
    request_count INTEGER NOT NULL
)";

// ADMIN
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub request_count: i32,
}

#[derive(Serialize, Debug)]
pub struct CreateUserResponse {
    pub api_token: String,
    pub request_count: i32,
}

// impl new for CreateUserResponse
impl CreateUserResponse {
    pub fn new(api_token: String, request_count: i32) -> Self {
        Self {
            api_token,
            request_count,
        }
    }
}

// Vec<CreateUserResponse>
#[derive(Serialize, Debug)]
pub struct ReadUsersResponse {
    pub responses: Vec<CreateUserResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub api_token: String,
    pub delta: i32,
}

#[derive(Serialize, Debug)]
pub struct UpdateUserResponse {
    pub api_token: String,
    pub request_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUsersRequest {
    pub api_tokens: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct DeleteUsersResponse {
    pub remaining_api_tokens: Vec<String>,
}

// USER
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadUserRequest {
    pub api_token: String,
}

#[derive(Serialize, Debug)]
pub struct ReadUserResponse {
    pub api_token: String,
    pub request_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecrementUserRequest {
    pub api_token: String,
}

#[derive(Serialize, Debug)]
pub struct DecrementUserResponse {
    pub api_token: String,
    pub request_count: i32,
}
