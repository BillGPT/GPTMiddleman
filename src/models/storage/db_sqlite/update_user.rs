use rusqlite::{Connection, Result};
use super::schema::{UpdateUserRequest, UpdateUserResponse};

pub fn process_update_user_request(conn: &Connection, request: UpdateUserRequest) -> Result<UpdateUserResponse, rusqlite::Error> {
    let rows = conn.execute("UPDATE users SET request_count = request_count + ?1 WHERE api_token = ?2", &[&request.delta.to_string(), &request.api_token])?;
    if rows == 0 {
        Err(rusqlite::Error::QueryReturnedNoRows)
    } else {
        let mut stmt = conn.prepare("SELECT request_count FROM users WHERE api_token = ?1")?;
        let new_count: i32 = stmt.query_row(&[&request.api_token], |row| row.get(0))?;
        Ok(UpdateUserResponse {
            api_token: request.api_token,
            request_count: new_count,
        })
    }
}
