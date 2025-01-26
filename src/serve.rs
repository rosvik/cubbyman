use crate::{commands, config::Config, middleware::basic_authenticate};
use axum::{
    extract::State,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
struct AppState {
    config: Config,
    socket: bollard::Docker,
}

pub async fn serve(socket: bollard::Docker, config: Config) {
    let state = AppState { config, socket };
    let auth = middleware::from_fn_with_state(state.clone(), basic_authenticate);
    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("{CRATE_NAME} v{CRATE_VERSION}") }),
        )
        .route(
            "/v1",
            get(|| async { "Authenticated" }).route_layer(auth.clone()),
        )
        .route("/v1/reload", post(reload).route_layer(auth.clone()))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8600").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn reload(State(state): State<AppState>) -> impl IntoResponse {
    println!("Reloading");
    commands::system::reload_all(&state.socket, &state.config).await;
    "Reloaded"
}
