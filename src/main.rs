// main.rs
mod server;
mod handlers;
mod models;
mod middleware;

use server::start_server;

#[tokio::main]
async fn main() {
    let _ = &*models::MEMORY_CACHE.read().unwrap();

    start_server().await;
}