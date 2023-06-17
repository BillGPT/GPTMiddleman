// src/models/storage/db_sqlite/create_user.rs
use rusqlite::{Connection, Result};
use super::schema::{DeleteUsersRequest, DeleteUsersResponse};

pub fn process_delete_users_request(conn: &Connection, request: DeleteUsersRequest) -> Result<DeleteUsersResponse, rusqlite::Error> {
    let mut stmt = conn.prepare("DELETE FROM users WHERE api_token = ?1")?;

    for api_token in request.api_tokens {
        stmt.execute(&[&api_token])?;
    }

    let mut stmt = conn.prepare("SELECT api_token FROM users")?;
    let remaining_api_tokens = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DeleteUsersResponse {
        remaining_api_tokens,
    })
}