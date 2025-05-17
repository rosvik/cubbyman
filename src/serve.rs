use crate::{commands, config, middleware::basic_authenticate};
use axum::{
    extract::State,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use clio::Input;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
struct AppState {
    config_arg: Input,
    socket: bollard::Docker,
}

pub async fn serve(socket: bollard::Docker, config_arg: Input) {
    let state = AppState { config_arg, socket };
    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("{CRATE_NAME} v{CRATE_VERSION}") }),
        )
        .nest_service("/api/cubbyman/v1", api_router(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8645").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn api_router(state: AppState) -> Router {
    let auth = middleware::from_fn_with_state(state.clone(), basic_authenticate);
    Router::new()
        .layer(auth)
        .route("/", get(|| async { "Authenticated" }))
        .route("/reload", post(reload))
        .with_state(state)
}

async fn reload(State(state): State<AppState>) -> impl IntoResponse {
    println!("Reloading");
    let config = match config::load_config(Some(state.config_arg)) {
        Ok(config) => config,
        Err(e) => {
            println!("Unable to load config file: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };
    commands::system::reload_all(&state.socket, &config).await;
    (axum::http::StatusCode::OK, "Reloaded").into_response()
}
