// src/models/storage/db_sqlite/read_users.rs
use rusqlite::{Connection, Result};
use super::schema::ReadUsersResponse;
use super::schema::CreateUserResponse;

pub fn process_read_users_request(conn: &Connection) -> Result<ReadUsersResponse, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT api_token, request_count FROM users")?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut responses = Vec::new();
    for result in rows {
        match result {
            Ok((api_token, request_count)) => {
                responses.push(CreateUserResponse::new(api_token, request_count));
            }
            Err(e) => return Err(e),
        }
    }
    
    Ok(ReadUsersResponse { responses })
}