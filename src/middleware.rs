use crate::utils;
use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware,
    response::Response,
};

pub async fn basic_authenticate(
    headers: HeaderMap,
    request: Request,
    next: middleware::Next,
) -> Response {
    fn unauthorized() -> Response {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::default())
            .unwrap_or_default()
    }
    match headers.get("Authorization") {
        None => {
            // If Authorization header is not set, issue a basic auth challenge
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(
                    "WWW-Authenticate",
                    HeaderValue::from_static("Basic realm=\"\", charset=\"UTF-8\""),
                )
                .body(axum::body::Body::default())
                .unwrap_or_default()
        }
        Some(authorization) => {
            let auth_header_value = authorization.to_str().unwrap_or_default().to_string();
            let b64 = auth_header_value
                .split_whitespace()
                .last()
                .unwrap_or_default()
                .to_string();
            let request_credentials = utils::decode_base64(b64).unwrap_or_default();
            if request_credentials.is_empty() {
                println!("Failed auth: Empty credentials");
                return unauthorized();
            }

            let username = match std::env::var("API_USERNAME") {
                Ok(username) => username,
                Err(_) => {
                    println!("Failed auth: API_USERNAME not set");
                    return unauthorized();
                }
            };
            let password = match std::env::var("API_PASSWORD") {
                Ok(password) => password,
                Err(_) => {
                    println!("Failed auth: API_PASSWORD not set");
                    return unauthorized();
                }
            };
            let credentials = format!("{}:{}", username, password);

            if credentials != request_credentials {
                let request_username = request_credentials.split(':').next().unwrap();
                println!(
            "\x1b[1;31mFailed auth: Incorrect username/password provided for user '{request_username}'\x1b[0m"
          );
                return unauthorized();
            }

            println!("Authenticated");

            next.run(request).await
        }
    }
}
