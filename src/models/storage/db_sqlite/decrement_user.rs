// src/models/storage/db_sqlite/decrement_user.rs
use rusqlite::{Connection, Result};
use super::schema::{DecrementUserRequest, DecrementUserResponse};

pub fn process_decrement_user_request(conn: &Connection, request: DecrementUserRequest) -> Result<DecrementUserResponse, rusqlite::Error> {
    let rows = conn.execute("UPDATE users SET request_count = request_count - 1 WHERE api_token = ?1", &[&request.api_token])?;
    if rows == 0 {
        Err(rusqlite::Error::QueryReturnedNoRows)
    } else {
        let mut stmt = conn.prepare("SELECT request_count FROM users WHERE api_token = ?1")?;
        let new_count: i32 = stmt.query_row(&[&request.api_token], |row| row.get(0))?;
        Ok(DecrementUserResponse {
            api_token: request.api_token,
            request_count: new_count,
        })
    }
}
