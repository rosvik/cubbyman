use crate::{commands, config::Config};
use axum::{
    extract::State,
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
    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("{CRATE_NAME} v{CRATE_VERSION}") }),
        )
        .route("/v1/reload", post(reload))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8600").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn reload(State(state): State<AppState>) -> impl IntoResponse {
    println!("Reloading");
    commands::system::reload_all(&state.socket, &state.config).await;
    "Reloaded"
}
