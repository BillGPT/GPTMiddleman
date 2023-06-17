// src/models/storage/db_sqlite/create_user.rs
use rusqlite::{Connection, Result};
use super::schema::{CreateUserRequest, CreateUserResponse};

use rand::Rng;
use rand::distributions::Alphanumeric;
use rusqlite::params;

pub fn process_create_user_request(conn: &Connection, request_count: CreateUserRequest) -> Result<CreateUserResponse, rusqlite::Error> {
    let api_token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();
    let api_token = format!("sk-{}", api_token);

    conn.execute(
        "INSERT INTO users (api_token, request_count) VALUES (?1, ?2)",
        params![api_token.clone(), request_count.request_count],
    )?;

    Ok(CreateUserResponse::new(api_token, request_count.request_count))
}