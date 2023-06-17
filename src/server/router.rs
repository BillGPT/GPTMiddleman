// src/server/router.rs
use hyper::{
    Body, 
    Request, 
    Response, 
    StatusCode, 
    http::Method,
    header::{HeaderName, HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_HEADERS},
};

use crate::handlers::{
    handle_chat, 
    handle_stream_chat,
    handle_embeddings, 
    handle_chat_json, 
    handle_create_user, 
    handle_read_users, 
    handle_update_user, 
    handle_delete_user, 
    handle_read_user,
    handle_decrement_user,
    handle_stream_chat_json
};

// Place cors_headers outside of the route_request function
static CORS_HEADERS: &'static [(HeaderName, &'static str)] = &[
    (ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
    (ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS"),
    (ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization"),
];

fn add_cors(response: Response<Body>) -> Response<Body> {
    let mut response = response;
    for (header_name, header_value) in CORS_HEADERS {
        response.headers_mut().insert(header_name.clone(), HeaderValue::from_static(header_value));
    }
    response
}

pub async fn route_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // CORS preflight
        (&Method::OPTIONS, _) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap();
            Ok(add_cors(response))
        },
        // openai
        (&Method::POST, "/v1/chat/completions") => Ok(add_cors(handle_chat(req).await?)),
        (&Method::POST, "/v1/chat/json") => Ok(add_cors(handle_chat_json(req).await?)),
        (&Method::POST, "/v1/stream_chat/completions") => Ok(add_cors(handle_stream_chat(req).await?)),
        (&Method::POST, "/v1/stream_chat/json") => Ok(add_cors(handle_stream_chat_json(req).await?)),
        (&Method::POST, "/v1/embeddings") => Ok(add_cors(handle_embeddings(req).await?)),
        // admin
        (&Method::POST, "/admin/create_user") => Ok(add_cors(handle_create_user(req).await?)),
        (&Method::GET, "/admin/read_users") => Ok(add_cors(handle_read_users(req).await?)),
        (&Method::POST, "/admin/update_user") => Ok(add_cors(handle_update_user(req).await?)),
        (&Method::POST, "/admin/delete_users") => Ok(add_cors(handle_delete_user(req).await?)),
        // user
        (&Method::POST, "/user/read_user") => Ok(add_cors(handle_read_user(req).await?)),
        (&Method::POST, "/user/decrement_user") => Ok(add_cors(handle_decrement_user(req).await?)),
        // Error Handling
        _ => Ok(add_cors(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap()
        )),
    }
}
