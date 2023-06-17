// db/mod.rs

pub mod create_user;
pub mod read_users;
pub mod update_user;
pub mod delete_users;
pub mod read_user;
pub mod decrement_user;
pub mod schema;

pub use create_user::process_create_user_request;
pub use read_users::process_read_users_request;
pub use update_user::process_update_user_request;
pub use delete_users::process_delete_users_request;
pub use read_user::process_read_user_request;
pub use decrement_user::process_decrement_user_request;

pub use schema::USER_TABLE;
pub use schema::create_connection;
pub use schema::{
    CreateUserRequest, CreateUserResponse, 
    ReadUsersResponse, UpdateUserRequest, 
    UpdateUserResponse,
    DeleteUsersRequest, DeleteUsersResponse,
    ReadUserRequest, ReadUserResponse,
    DecrementUserRequest, DecrementUserResponse,
};