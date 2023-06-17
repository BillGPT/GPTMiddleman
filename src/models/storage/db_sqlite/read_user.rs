// src/models/storage/db_sqlite/read_user.rs
use rusqlite::{Connection, Result};
use super::schema::{ReadUserRequest, ReadUserResponse};

pub fn process_read_user_request(conn: &Connection, request: ReadUserRequest) -> Result<ReadUserResponse, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT api_token, request_count FROM users WHERE api_token = ?1")?;
    let result = stmt.query_row(&[&request.api_token], |row| Ok((row.get(0)?, row.get(1)?)));

    match result {
        Ok((api_token, request_count)) => {
            Ok(ReadUserResponse { api_token, request_count })
        }
        Err(_) => Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
