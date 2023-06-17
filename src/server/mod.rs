// src/server/mod.rs
pub mod router;
pub use router::route_request;

use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use hyper::Server;


const PORT: u16 = 3000;

pub async fn start_server() {
    let addr = SocketAddr::from(([0, 0, 0, 0], PORT));
    let service = make_service_fn(|_conn| {
        async {
            Ok::<_, hyper::Error>(service_fn(route_request))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("Starting server on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}