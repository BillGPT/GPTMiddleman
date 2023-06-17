// src/models/storage/mod.rs
pub mod memory;
pub mod db_sqlite;

pub use memory::MEMORY_CACHE;
pub use db_sqlite::{
    create_connection, 
    process_create_user_request, 
    process_read_users_request, 
    process_update_user_request, 
    process_delete_users_request,
    process_read_user_request,
    process_decrement_user_request
};
pub use db_sqlite::{
    CreateUserRequest, CreateUserResponse,
    ReadUsersResponse, UpdateUserRequest,
    UpdateUserResponse, 
    DeleteUsersRequest, DeleteUsersResponse,
    ReadUserRequest, ReadUserResponse,
    DecrementUserRequest, DecrementUserResponse,
};